use crate::models::*;
use crate::services::process::run_wsl as run;

#[tauri::command]
pub fn list_wsl_distros() -> Result<Vec<Distro>, String> {
    let output = crate::services::process::checked(run(&["--list", "--verbose"] )?, "读取 WSL 实例列表（请确认已安装 WSL）")?;
    Ok(crate::services::wsl_parser::parse_wsl_list(&output.stdout))
}
#[tauri::command] pub fn start_distro(name:String)->Result<Output,String>{run(&["-d",&name,"--","echo","ok"])}
#[tauri::command] pub fn terminate_distro(name:String)->Result<Output,String>{run(&["--terminate",&name])}
#[tauri::command] pub fn restart_distro(name:String)->Result<Output,String>{let a=terminate_distro(name.clone())?;std::thread::sleep(std::time::Duration::from_millis(500));let b=start_distro(name)?;Ok(Output{success:a.success&&b.success,code:b.code,stdout:format!("{}\n{}",a.stdout,b.stdout),stderr:format!("{}\n{}",a.stderr,b.stderr)})}
#[tauri::command] pub fn shutdown_wsl()->Result<Output,String>{run(&["--shutdown"])}
fn text(result:Result<Output,String>, errors:&mut Vec<String>)->Option<String>{match result{Ok(o)if o.success=>Some(o.stdout),Ok(o)=>{errors.push(format!("{} (exit {:?})",o.stderr,o.code));None},Err(e)=>{errors.push(e);None}}}
#[tauri::command] pub fn get_distro_resource_info(name:String)->Result<Resource,String>{let mut e=vec![];let memory=text(run(&["-d",&name,"--","free","-h"]),&mut e);let disk=text(run(&["-d",&name,"--","df","-h","/"]),&mut e);let uptime=text(run(&["-d",&name,"--","uptime","-p"]),&mut e);let process_count=text(run(&["-d",&name,"--","sh","-lc","ps -e --no-headers | wc -l"]),&mut e).and_then(|v|v.parse().ok());Ok(Resource{distro:name,memory_text:memory,disk_text:disk,uptime_text:uptime,process_count,errors:e})}
#[tauri::command] pub fn list_ports(name:String)->Result<Vec<Port>,String>{let out=run(&["-d",&name,"--","ss","-tulpn"]).or_else(|_|run(&["-d",&name,"--","netstat","-tulpn"]))?;if !out.success{return Err(format!("Failed to list ports: {}",out.stderr))};Ok(out.stdout.lines().filter_map(|raw|{if !(raw.contains("LISTEN")||raw.contains("UNCONN")){return None};let local=raw.split_whitespace().find(|x|x.rsplit_once(':').and_then(|(_,p)|p.parse::<u16>().ok()).is_some())?.to_string();let port=local.rsplit_once(':')?.1.parse().ok()?;let process_name=raw.split("users:((\"").nth(1).and_then(|x|x.split('\"').next()).map(str::to_string);let pid=raw.split("pid=").nth(1).and_then(|x|x.split(|c:char|!c.is_ascii_digit()).next()).and_then(|x|x.parse().ok());Some(Port{protocol:raw.split_whitespace().next()?.chars().take(3).collect(),local_address:local,port,process_name,pid,raw:raw.into()})}).collect())}
