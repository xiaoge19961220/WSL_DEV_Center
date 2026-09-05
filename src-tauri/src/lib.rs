mod commands;
mod models;
mod services;

// 系统调用在工作线程执行，避免阻塞桌面界面。
#[tauri::command]
async fn list_wsl_distros() -> Result<Vec<models::Distro>, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::list_wsl_distros())
        .await
        .map_err(|e| format!("后台任务失败（list_wsl_distros）：{e}"))?
}

#[tauri::command]
async fn list_online_distros() -> Result<Vec<models::OnlineDistro>, String> {
    tauri::async_runtime::spawn_blocking(commands::wsl::list_online_distros)
        .await
        .map_err(|e| format!("后台任务失败（list_online_distros）：{e}"))?
}

#[tauri::command]
async fn install_distro(distribution: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::install_distro(distribution))
        .await
        .map_err(|e| format!("后台任务失败（install_distro）：{e}"))?
}

#[tauri::command]
async fn import_distro(
    name: String,
    install_location: String,
    archive_path: String,
    vhd: bool,
) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::wsl::import_distro(name, install_location, archive_path, vhd)
    })
    .await
    .map_err(|e| format!("后台任务失败（import_distro）：{e}"))?
}

#[tauri::command]
async fn export_distro(
    name: String,
    archive_path: String,
    vhd: bool,
) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::wsl::export_distro(name, archive_path, vhd)
    })
    .await
    .map_err(|e| format!("后台任务失败（export_distro）：{e}"))?
}

#[tauri::command]
async fn clone_distro(
    source: String,
    target: String,
    install_location: String,
) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::wsl::clone_distro(source, target, install_location)
    })
    .await
    .map_err(|e| format!("后台任务失败（clone_distro）：{e}"))?
}

#[tauri::command]
async fn unregister_distro(name: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::unregister_distro(name))
        .await
        .map_err(|e| format!("后台任务失败（unregister_distro）：{e}"))?
}

#[tauri::command]
async fn start_distro(name: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::start_distro(name))
        .await
        .map_err(|e| format!("后台任务失败（start_distro）：{e}"))?
}

#[tauri::command]
async fn terminate_distro(name: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::terminate_distro(name))
        .await
        .map_err(|e| format!("后台任务失败（terminate_distro）：{e}"))?
}

#[tauri::command]
async fn restart_distro(name: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::restart_distro(name))
        .await
        .map_err(|e| format!("后台任务失败（restart_distro）：{e}"))?
}

#[tauri::command]
async fn shutdown_wsl() -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::shutdown_wsl())
        .await
        .map_err(|e| format!("后台任务失败（shutdown_wsl）：{e}"))?
}

#[tauri::command]
async fn open_terminal(name: String, terminal: Option<String>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || commands::system::open_terminal(name, terminal))
        .await
        .map_err(|e| format!("后台任务失败（open_terminal）：{e}"))?
}

#[tauri::command]
async fn open_home_in_explorer(name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || commands::system::open_home_in_explorer(name))
        .await
        .map_err(|e| format!("后台任务失败（open_home_in_explorer）：{e}"))?
}

#[tauri::command]
async fn open_vscode_home(name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || commands::system::open_vscode_home(name))
        .await
        .map_err(|e| format!("后台任务失败（open_vscode_home）：{e}"))?
}

#[tauri::command]
async fn get_distro_resource_info(name: String) -> Result<models::Resource, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::get_distro_resource_info(name))
        .await
        .map_err(|e| format!("后台任务失败（get_distro_resource_info）：{e}"))?
}

#[tauri::command]
async fn list_ports(name: String) -> Result<Vec<models::Port>, String> {
    tauri::async_runtime::spawn_blocking(move || commands::wsl::list_ports(name))
        .await
        .map_err(|e| format!("后台任务失败（list_ports）：{e}"))?
}

#[tauri::command]
async fn list_docker_containers(name: String) -> Result<Vec<models::Docker>, String> {
    tauri::async_runtime::spawn_blocking(move || commands::docker::list_docker_containers(name))
        .await
        .map_err(|e| format!("后台任务失败（list_docker_containers）：{e}"))?
}

#[tauri::command]
async fn start_container(distro: String, container: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::docker::start_container(distro, container)
    })
    .await
    .map_err(|e| format!("后台任务失败（start_container）：{e}"))?
}

#[tauri::command]
async fn stop_container(distro: String, container: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::docker::stop_container(distro, container)
    })
    .await
    .map_err(|e| format!("后台任务失败（stop_container）：{e}"))?
}

#[tauri::command]
async fn container_logs(distro: String, container: String) -> Result<models::Output, String> {
    tauri::async_runtime::spawn_blocking(move || {
        commands::docker::container_logs(distro, container)
    })
    .await
    .map_err(|e| format!("后台任务失败（container_logs）：{e}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_app() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_wsl_distros,
            list_online_distros,
            install_distro,
            import_distro,
            export_distro,
            clone_distro,
            unregister_distro,
            start_distro,
            terminate_distro,
            restart_distro,
            shutdown_wsl,
            open_terminal,
            open_home_in_explorer,
            open_vscode_home,
            get_distro_resource_info,
            list_ports,
            list_docker_containers,
            start_container,
            stop_container,
            container_logs,
        ])
        .run(tauri::generate_context!())
        .expect("WSL 开发中心启动失败");
}
