use std::process::Command;

#[tauri::command] pub fn open_terminal(name:String)->Result<(),String>{if Command::new("wt.exe").args(["wsl.exe","-d",&name]).spawn().is_ok(){return Ok(())};Command::new("powershell.exe").args(["-NoExit","-Command","& wsl.exe -d $args[0]",&name]).spawn().map_err(|e|format!("Windows Terminal was not found and PowerShell fallback failed: {e}"))?;Ok(())}
#[tauri::command] pub fn open_home_in_explorer(name:String)->Result<(),String>{Command::new("explorer.exe").arg(format!(r"\\wsl$\{}\home",name)).spawn().map_err(|e|format!("Failed to open WSL home path: {e}"))?;Ok(())}
#[tauri::command] pub fn open_vscode_home(name:String)->Result<(),String>{Command::new("code").args(["--remote",&format!("wsl+{name}"),"/home"]).spawn().map_err(|_|"VS Code CLI not found. Please enable the code command in PATH.".to_string())?;Ok(())}
