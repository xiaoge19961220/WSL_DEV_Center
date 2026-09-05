mod models;
mod services;
mod commands;

use commands::{wsl::*, docker::*, system::*};

#[cfg_attr(mobile, tauri::mobile_entry_point)] pub fn run_app(){tauri::Builder::default().plugin(tauri_plugin_opener::init()).invoke_handler(tauri::generate_handler![list_wsl_distros,start_distro,terminate_distro,restart_distro,shutdown_wsl,open_terminal,open_home_in_explorer,open_vscode_home,get_distro_resource_info,list_ports,list_docker_containers,start_container,stop_container,container_logs]).run(tauri::generate_context!()).expect("WSL 开发中心启动失败")}
