use crate::models::*;
use crate::services::process::run_wsl as run;

#[tauri::command] pub fn list_docker_containers(name:String)->Result<Vec<Docker>,String>{let out=run(&["-d",&name,"--","docker","ps","-a","--format","{{json .}}"])?;if !out.success{return Err(format!("Docker is not installed in this distribution: {}",out.stderr))};Ok(out.stdout.lines().filter_map(|x|serde_json::from_str(x).ok()).collect())}
