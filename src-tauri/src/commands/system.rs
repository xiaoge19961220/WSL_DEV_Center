use std::process::Command;
use crate::services::process::clean_output;

fn validate_name(name: &str) -> Result<(), String> {
    if crate::commands::wsl::list_wsl_distros()?.iter().any(|d| d.name == name) { Ok(()) }
    else { Err("实例不存在，请刷新列表后重试。".into()) }
}

#[tauri::command]
pub fn open_terminal(name: String, terminal: Option<String>) -> Result<String, String> {
    validate_name(&name)?;
    // 用户输入仅通过环境变量传递，PowerShell 脚本保持固定。
    let script = "& wsl.exe --distribution $env:WSL_DEV_CENTER_DISTRO";
    if terminal.as_deref() != Some("powershell") && Command::new("wt.exe")
        .env("WSL_DEV_CENTER_DISTRO", &name)
        .args(["new-tab", "powershell.exe", "-NoLogo", "-NoProfile", "-Command", script])
        .spawn().is_ok() {
        return Ok("已请求打开 Windows Terminal。".into());
    }
    Command::new("powershell.exe").env("WSL_DEV_CENTER_DISTRO", &name)
        .args(["-NoLogo", "-NoProfile", "-NoExit", "-Command", script])
        .spawn().map_err(|e| format!("无法打开终端。\n位置：终端快捷入口\n退出码：不可用\n详情：{e}"))?;
    Ok(if terminal.as_deref() == Some("powershell") { "已请求打开 PowerShell。" } else { "Windows Terminal 不可用，已改用 PowerShell。" }.into())
}

#[tauri::command]
pub fn open_home_in_explorer(name: String) -> Result<(), String> {
    validate_name(&name)?;
    Command::new("explorer.exe").arg(format!(r"\\wsl$\{}\home", name))
        .spawn().map_err(|e| format!("无法打开实例文件目录。\n位置：文件快捷入口\n退出码：不可用\n详情：{e}"))?;
    Ok(())
}

#[tauri::command]
pub fn open_vscode_home(name: String) -> Result<(), String> {
    validate_name(&name)?;
    let mut command = Command::new("powershell.exe");
    command.env("WSL_DEV_CENTER_DISTRO", &name).args([
        "-NoLogo", "-NoProfile", "-NonInteractive", "-Command",
        "$ErrorActionPreference='Stop'; & code --remote ('wsl+' + $env:WSL_DEV_CENTER_DISTRO) /home; exit $LASTEXITCODE",
    ]);
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let out = command.output().map_err(|e| format!("无法打开 VS Code。\n位置：VS Code 快捷入口\n退出码：不可用\n详情：{e}"))?;
    if !out.status.success() {
        return Err(format!("无法打开 VS Code，请确认已安装 VS Code、WSL 扩展，并将 code 命令加入 PATH。\n位置：VS Code 快捷入口\n退出码：{:?}\n标准错误：{}\n标准输出：{}", out.status.code(), clean_output(&out.stderr), clean_output(&out.stdout)));
    }
    Ok(())
}
