use crate::models::{Docker, Output};
use crate::services::{docker_parser::parse_docker, process::{checked, run_wsl}};
use crate::commands::wsl::ensure_running;

#[tauri::command]
pub fn list_docker_containers(name: String) -> Result<Vec<Docker>, String> {
    ensure_running(&name)?;
    let out = run_wsl(&["-d", &name, "--", "docker", "ps", "-a", "--format", "{{json .}}"])?;
    if !out.success {
        let detail = format!("{} {}", out.stderr, out.stdout).to_lowercase();
        let reason = if detail.contains("not found") || detail.contains("no such file") {
            "此实例中未找到 Docker，请先安装 Docker 或检查 PATH"
        } else { "Docker 查询失败，请检查服务是否启动，以及当前用户是否有访问权限" };
        return checked(out, reason).map(|_| Vec::new());
    }
    parse_docker(&out.stdout)
}

fn container_action(distro: String, container: String, action: &str) -> Result<Output, String> {
    let rows = list_docker_containers(distro.clone())?;
    let selected = rows.iter().find(|c| c.id == container && !c.id.is_empty() && c.id.chars().all(|v| v.is_ascii_hexdigit()))
        .ok_or_else(|| "容器已不存在或标识无效，请刷新容器列表。".to_string())?;
    let mut args = vec!["-d", &distro, "--", "docker", action];
    if action == "logs" { args.extend(["--tail", "200"]); }
    args.push(&selected.id);
    checked(run_wsl(&args)?, match action { "start" => "启动容器", "stop" => "停止容器", _ => "读取容器日志" })
}

#[tauri::command]
pub fn start_container(distro: String, container: String) -> Result<Output, String> {
    container_action(distro, container, "start")
}
#[tauri::command]
pub fn stop_container(distro: String, container: String) -> Result<Output, String> {
    container_action(distro, container, "stop")
}
#[tauri::command]
pub fn container_logs(distro: String, container: String) -> Result<Output, String> {
    container_action(distro, container, "logs")
}
