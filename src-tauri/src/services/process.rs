use std::process::Command;
use crate::models::Output;

// wsl.exe 列表使用 UTF-16LE，而发行版内的命令通常输出 UTF-8。
pub fn clean_output(bytes: &[u8]) -> String {
    let utf16 = bytes.starts_with(&[0xff, 0xfe])
        || (bytes.len() >= 4 && bytes.chunks_exact(2).take(32).filter(|p| p[1] == 0).count() > bytes.chunks_exact(2).take(32).count() / 2);
    let text = if utf16 {
        let start = if bytes.starts_with(&[0xff, 0xfe]) { 2 } else { 0 };
        String::from_utf16_lossy(&bytes[start..].chunks_exact(2).map(|p| u16::from_le_bytes([p[0], p[1]])).collect::<Vec<_>>())
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    };
    text.replace('\0', "").trim_matches('\u{feff}').trim().to_owned()
}

pub fn run_command(program: &str, args: &[&str]) -> Result<Output, String> {
    let mut command = Command::new(program);
    command.args(args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let out = command.output().map_err(|e| format!("无法执行 {program}。请确认程序已安装。\n位置：命令执行\n退出码：不可用\n详情：{e}"))?;
    Ok(Output {
        success: out.status.success(),
        code: out.status.code(),
        stdout: clean_output(&out.stdout),
        stderr: clean_output(&out.stderr),
    })
}

pub fn checked(output: Output, location: &str) -> Result<Output, String> {
    if output.success { return Ok(output); }
    Err(format!("{location}失败\n位置：{location}\n退出码：{:?}\n标准错误：{}\n标准输出：{}", output.code, output.stderr, output.stdout))
}

pub fn run_wsl(args: &[&str]) -> Result<Output, String> {
    run_command("wsl.exe", args)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decodes_chinese_utf16_and_utf8() {
        let bytes: Vec<u8> = "  Ubuntu 中文  运行中  2\r\n".encode_utf16().flat_map(u16::to_le_bytes).collect();
        assert_eq!(clean_output(&bytes), "Ubuntu 中文  运行中  2");
        assert_eq!(clean_output("中文\n".as_bytes()), "中文");
    }
    #[test]
    fn nonzero_exit_includes_diagnostics() {
        let error = checked(Output {success: false, code: Some(1), stdout: "out".into(), stderr: "err".into()}, "停止实例").unwrap_err();
        assert!(error.contains("err") && error.contains("out") && error.contains('1'));
    }
}
