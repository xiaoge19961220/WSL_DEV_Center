use crate::models::*;
use crate::services::process::run_wsl as run;

#[tauri::command]
pub fn list_wsl_distros() -> Result<Vec<Distro>, String> {
    let output = crate::services::process::checked(run(&["--list", "--verbose"] )?, "读取 WSL 实例列表（请确认已安装 WSL）")?;
    Ok(crate::services::wsl_parser::parse_wsl_list(&output.stdout))
}
#[tauri::command] pub fn start_distro(name:String)->Result<Output,String>{run(&["-d",&name,"--","echo","ok"])}
#[tauri::command] pub fn terminate_distro(name:String)->Result<Output,String>{run(&["--terminate",&name])}
#[tauri::command] pub fn restart_distro(name:String)->Result<Output,String>{let a=crate::services::process::checked(terminate_distro(name.clone())?, "重启实例：停止阶段")?;std::thread::sleep(std::time::Duration::from_millis(500));let b=start_distro(name)?;Ok(Output{success:a.success&&b.success,code:b.code,stdout:format!("{}\n{}",a.stdout,b.stdout),stderr:format!("{}\n{}",a.stderr,b.stderr)})}
#[tauri::command] pub fn shutdown_wsl()->Result<Output,String>{run(&["--shutdown"])}
fn text(result:Result<Output,String>, errors:&mut Vec<String>)->Option<String>{match result{Ok(o)if o.success=>Some(o.stdout),Ok(o)=>{errors.push(format!("{} (exit {:?})",o.stderr,o.code));None},Err(e)=>{errors.push(e);None}}}
#[tauri::command] pub fn get_distro_resource_info(name:String)->Result<Resource,String>{ensure_running(&name)?;let mut e=vec![];let memory=text(run(&["-d",&name,"--","free","-h"]),&mut e);let disk=text(run(&["-d",&name,"--","df","-h","/"]),&mut e);let uptime=text(run(&["-d",&name,"--","uptime","-p"]),&mut e);let process_count=text(run(&["-d",&name,"--","sh","-lc","ps -e --no-headers | wc -l"]),&mut e).and_then(|v|v.parse().ok());Ok(Resource{distro:name,memory_text:memory,disk_text:disk,uptime_text:uptime,process_count,errors:e})}
#[tauri::command]
pub fn list_ports(name: String) -> Result<Vec<Port>, String> {
    ensure_running(&name)?;
    let ss = run(&["-d", &name, "--", "ss", "-tulpn"])
        .and_then(|out| crate::services::process::checked(out, "ss 端口查询"));
    match ss {
        Ok(out) => Ok(crate::services::port_parser::parse_ports(&out.stdout, false)),
        Err(first) => {
            let out = run(&["-d", &name, "--", "netstat", "-tulpn"])
                .and_then(|out| crate::services::process::checked(out, "netstat 端口查询"))
                .map_err(|second| format!("{first}\n\n回退查询也失败，请确认已安装 ss 或 netstat。\n{second}"))?;
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
