use crate::models::*;
use crate::services::process::run_wsl as run;

fn validate_instance_name(name: &str) -> Result<&str, String> {
    let value = name.trim();
    if value.is_empty() {
        return Err("实例名称不能为空。".into());
    }
    if value.len() > 128 || value.chars().any(char::is_control) {
        return Err("实例名称过长或包含控制字符。".into());
    }
    Ok(value)
}

fn validate_path<'a>(path: &'a str, label: &str) -> Result<&'a str, String> {
    let value = path.trim();
    if value.is_empty() {
        return Err(format!("{label}不能为空。"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{label}包含无效字符。"));
    }
    if !std::path::Path::new(value).is_absolute() {
        return Err(format!("{label}必须是 Windows 绝对路径。"));
    }
    Ok(value)
}

fn run_owned(args: Vec<String>) -> Result<Output, String> {
    let values: Vec<&str> = args.iter().map(String::as_str).collect();
    run(&values)
}

fn import_args(
    name: &str,
    install_location: &str,
    archive_path: &str,
    vhd: bool,
) -> Result<Vec<String>, String> {
    let mut args = vec![
        "--import".into(),
        validate_instance_name(name)?.into(),
        validate_path(install_location, "安装目录")?.into(),
        validate_path(archive_path, "导入文件")?.into(),
        "--version".into(),
        "2".into(),
    ];
    if vhd {
        args.push("--vhd".into());
    }
    Ok(args)
}

fn export_args(name: &str, archive_path: &str, vhd: bool) -> Result<Vec<String>, String> {
    let mut args = vec![
        "--export".into(),
        validate_instance_name(name)?.into(),
        validate_path(archive_path, "导出文件")?.into(),
    ];
    if vhd {
        args.extend(["--format".into(), "vhd".into()]);
    }
    Ok(args)
}

pub fn list_wsl_distros() -> Result<Vec<Distro>, String> {
    let output = crate::services::process::checked(
        run(&["--list", "--verbose"])?,
        "读取 WSL 实例列表（请确认已安装 WSL）",
    )?;
    Ok(crate::services::wsl_parser::parse_wsl_list(&output.stdout))
}
pub fn list_online_distros() -> Result<Vec<OnlineDistro>, String> {
    let output =
        crate::services::process::checked(run(&["--list", "--online"])?, "读取可安装发行版")?;
    Ok(crate::services::wsl_parser::parse_online_distros(
        &output.stdout,
    ))
}
pub fn install_distro(distribution: String) -> Result<Output, String> {
    let distribution = validate_instance_name(&distribution)?;
    run(&["--install", "--distribution", distribution, "--no-launch"])
}
pub fn import_distro(
    name: String,
    install_location: String,
    archive_path: String,
    vhd: bool,
) -> Result<Output, String> {
    run_owned(import_args(&name, &install_location, &archive_path, vhd)?)
}
pub fn export_distro(name: String, archive_path: String, vhd: bool) -> Result<Output, String> {
    run_owned(export_args(&name, &archive_path, vhd)?)
}
pub fn unregister_distro(name: String) -> Result<Output, String> {
    run(&["--unregister", validate_instance_name(&name)?])
}
pub fn clone_distro(
    source: String,
    target: String,
    install_location: String,
) -> Result<Output, String> {
    validate_instance_name(&source)?;
    validate_instance_name(&target)?;
    validate_path(&install_location, "副本安装目录")?;
    if source.trim() == target.trim() {
        return Err("副本名称不能与源实例相同。".into());
    }
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("无法生成临时文件名：{e}"))?
        .as_nanos();
    let archive = std::env::temp_dir().join(format!(
        "wsl-dev-center-clone-{}-{unique}.tar",
        std::process::id()
    ));
    let archive = archive.to_string_lossy().into_owned();
    let result = (|| {
        let exported = run_owned(export_args(&source, &archive, false)?)?;
        crate::services::process::checked(exported, "复制实例：导出源实例")?;
        let imported = run_owned(import_args(&target, &install_location, &archive, false)?)?;
        crate::services::process::checked(imported, "复制实例：导入副本")
    })();
    let _ = std::fs::remove_file(&archive);
    result
}
pub fn start_distro(name: String) -> Result<Output, String> {
    run(&["-d", &name, "--", "echo", "ok"])
}
pub fn terminate_distro(name: String) -> Result<Output, String> {
    run(&["--terminate", &name])
}
pub fn restart_distro(name: String) -> Result<Output, String> {
    let a =
        crate::services::process::checked(terminate_distro(name.clone())?, "重启实例：停止阶段")?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    let b = start_distro(name)?;
    Ok(Output {
        success: a.success && b.success,
        code: b.code,
        stdout: format!("{}\n{}", a.stdout, b.stdout),
        stderr: format!("{}\n{}", a.stderr, b.stderr),
    })
}
pub fn shutdown_wsl() -> Result<Output, String> {
    run(&["--shutdown"])
}
fn text(
    result: Result<Output, String>,
    location: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match result.and_then(|out| crate::services::process::checked(out, location)) {
        Ok(o) => Some(o.stdout),
        Err(e) => {
            errors.push(e);
            None
        }
    }
}
pub fn get_distro_resource_info(name: String) -> Result<Resource, String> {
    ensure_running(&name)?;
    let mut e = vec![];
    let os_version = text(
        run(&["-d", &name, "--", "cat", "/etc/os-release"]),
        "读取系统版本",
        &mut e,
    );
    let os_version_text = os_version
        .as_deref()
        .and_then(crate::services::wsl_parser::parse_os_release);
    if os_version.is_some() && os_version_text.is_none() {
        e.push("读取系统版本成功，但未找到 PRETTY_NAME。".into());
    }
    let kernel_version_text = text(
        run(&["-d", &name, "--", "uname", "-r"]),
        "读取内核版本",
        &mut e,
    );
    let cpu = text(
        run(&["-d", &name, "--", "top", "-bn1"]),
        "读取 CPU 占用",
        &mut e,
    );
    let cpu_text = cpu
        .as_deref()
        .and_then(crate::services::wsl_parser::parse_cpu_summary);
    if cpu.is_some() && cpu_text.is_none() {
        e.push("读取 CPU 占用成功，但无法识别 top 输出。".into());
    }
    let memory = text(run(&["-d", &name, "--", "free", "-h"]), "读取内存", &mut e);
    let disk = text(
        run(&["-d", &name, "--", "df", "-h", "/"]),
        "读取磁盘",
        &mut e,
    );
    let uptime = text(
        run(&["-d", &name, "--", "uptime", "-p"]),
        "读取运行时间",
        &mut e,
    );
    let process_count = text(
        run(&["-d", &name, "--", "ps", "-e", "--no-headers"]),
        "读取进程数",
        &mut e,
    )
    .map(|v| v.lines().filter(|line| !line.trim().is_empty()).count() as u32);
    Ok(Resource {
        distro: name,
        os_version_text,
        kernel_version_text,
        cpu_text,
        memory_text: memory,
        disk_text: disk,
        uptime_text: uptime,
        process_count,
        errors: e,
    })
}
pub fn list_ports(name: String) -> Result<Vec<Port>, String> {
    ensure_running(&name)?;
    let ss = run(&["-d", &name, "--", "ss", "-tulpn"])
        .and_then(|out| crate::services::process::checked(out, "ss 端口查询"));
    match ss {
        Ok(out) => Ok(crate::services::port_parser::parse_ports(
            &out.stdout,
            false,
        )),
        Err(first) => {
            let out = run(&["-d", &name, "--", "netstat", "-tulpn"])
                .and_then(|out| crate::services::process::checked(out, "netstat 端口查询"))
                .map_err(|second| {
                    format!("{first}\n\n回退查询也失败，请确认已安装 ss 或 netstat。\n{second}")
                })?;
            Ok(crate::services::port_parser::parse_ports(&out.stdout, true))
        }
    }
}

pub fn ensure_running(name: &str) -> Result<(), String> {
    match list_wsl_distros()?.into_iter().find(|d| d.name == name) {
        Some(d) if d.state == "Running" => Ok(()),
        Some(_) => Err("此实例未运行，请先在实例页面启动，再读取资源、端口或 Docker。".into()),
        None => Err("实例不存在，请刷新列表。".into()),
    }
}

#[cfg(test)]
mod live_tests {
    use super::*;

    #[test]
    #[ignore = "需要 Windows WSL 环境；仅只读列出实例并验证停止保护"]
    fn live_wsl_listing_and_stopped_guard() {
        let distros = list_wsl_distros().expect("无法列出真实 WSL 实例");
        println!("{}", serde_json::to_string(&distros).unwrap());
        for distro in distros.iter().filter(|d| d.state == "Stopped") {
            assert!(ensure_running(&distro.name).is_err());
        }
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    #[test]
    fn validates_instance_names_before_invoking_wsl() {
        assert!(validate_instance_name("Ubuntu 开发").is_ok());
        assert!(validate_instance_name("  ").is_err());
        assert!(validate_instance_name("bad\nname").is_err());
    }

    #[test]
    fn operation_arguments_keep_names_and_paths_separate() {
        assert_eq!(
            import_args(
                "开发环境",
                r"D:\\WSL\\开发环境",
                r"D:\\备份\\ubuntu.tar",
                false
            )
            .unwrap(),
            vec![
                "--import",
                "开发环境",
                r"D:\\WSL\\开发环境",
                r"D:\\备份\\ubuntu.tar",
                "--version",
                "2"
            ]
        );
        assert_eq!(
            export_args("Ubuntu", r"D:\\备份\\ubuntu.vhdx", true).unwrap(),
            vec![
                "--export",
                "Ubuntu",
                r"D:\\备份\\ubuntu.vhdx",
                "--format",
                "vhd"
            ]
        );
        assert!(import_args("开发环境", "relative/path", r"D:\\备份\\ubuntu.tar", false).is_err());
    }
}
