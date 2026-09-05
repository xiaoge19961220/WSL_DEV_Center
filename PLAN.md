# PLAN.md — WSL Dev Center

## 1. 项目概述

开发一个 Windows 桌面应用，暂定名称为 **WSL Dev Center**。

它是一个类似 OrbStack 使用体验的本地 WSL/WSL2 管理工具，但不是 OrbStack 的完整复刻。

核心定位：

```text
一个纯本地的 Windows WSL 开发环境管理器。
```

主要功能：

```text
1. 查看所有 WSL/WSL2 实例
2. 启动、停止、重启 WSL 实例
3. 打开指定 WSL 实例的终端
4. 打开指定 WSL 实例的文件目录
5. 查看指定 WSL 实例的资源信息
6. 查看指定 WSL 实例中的端口监听情况
7. 查看指定 WSL 实例中的 Docker 容器
8. 后续支持 Docker Compose、导入导出、克隆、项目快捷入口
```

本项目是本地开发者工具，不是 SaaS，不做云服务。

---

## 2. 强制产品边界

以下功能永远不做：

```text
1. 不需要账号系统
2. 不需要登录
3. 不需要注册
4. 不需要用户中心
5. 不需要权限套餐
6. 不需要云同步
7. 不需要远程管理
8. 不需要团队协作
9. 不需要 HTTPS
10. 不需要证书管理
11. 不需要 mkcert
12. 不需要 ACME
13. 不需要 Let's Encrypt
14. 不需要修改系统证书信任区
15. 不需要上传任何本地数据
16. 不需要遥测
17. 不需要埋点统计
```

任何 AI 或开发者不得添加以下内容：

```text
登录页
注册页
用户表
token
session
OAuth
云端配置接口
远程同步接口
远程设备管理
团队 workspace
HTTPS 证书申请
证书信任安装
用户行为统计
```

如果未来实现本地域名或端口代理，只允许做：

```text
HTTP 本地代理
localhost 域名规则
本机访问
```

不允许做：

```text
HTTPS
证书
公网暴露
远程访问
```

---

## 3. 技术栈

固定使用方案 A：

```text
Desktop Framework: Tauri 2
Frontend: React + TypeScript
Backend: Rust
Package Manager: pnpm
Target OS: Windows 10 / Windows 11
Primary Runtime: WSL2
```

前端：

```text
React
TypeScript
CSS Modules 或普通 CSS
不强制 UI 组件库
```

后端：

```text
Rust
Tauri commands
std::process::Command
```

系统调用：

```text
wsl.exe
wt.exe
explorer.exe
code
docker
ss
netstat
free
df
uptime
ps
```

---

## 4. 非目标

MVP 阶段不要做：

```text
Kubernetes 管理
完整 Docker Desktop 替代
完整虚拟机管理器
云同步
账号系统
HTTPS
证书管理
公网代理
远程访问
团队协作
插件市场
复杂主题系统
数据库
```

---

## 5. MVP 完成标准

MVP 必须完成：

```text
1. 可以启动 Windows 桌面应用
2. 可以列出所有 WSL 实例
3. 可以识别实例名称
4. 可以识别实例状态 Running / Stopped
5. 可以识别 WSL 版本 1 / 2
6. 可以识别默认实例
7. 可以启动 stopped 实例
8. 可以停止 running 实例
9. 可以重启实例
10. 可以全局关闭 WSL，但必须二次确认
11. 可以打开指定实例的终端
12. 可以打开指定实例的 home 目录
13. 可以进入实例详情页
14. 可以查看实例内存信息
15. 可以查看实例磁盘信息
16. 可以查看实例 uptime
17. 可以查看实例进程数量
18. 可以查看监听端口
19. 可以查看 Docker 容器列表
20. Docker 不存在时能给出清晰提示
21. 所有命令失败时有可读错误
22. 可以打包 Windows 安装包
```

---

## 6. 推荐目录结构

项目结构：

```text
wsl-dev-center/
  package.json
  pnpm-lock.yaml
  index.html
  src/
    main.tsx
    app/
      App.tsx
      routes.tsx
    components/
      AppShell.tsx
      Sidebar.tsx
      Header.tsx
      DistroCard.tsx
      DistroTable.tsx
      StatusBadge.tsx
      ConfirmDialog.tsx
      ErrorBox.tsx
      EmptyState.tsx
      Loading.tsx
      PortTable.tsx
      DockerTable.tsx
      ResourcePanel.tsx
      ActionButton.tsx
    pages/
      DashboardPage.tsx
      MachinesPage.tsx
      MachineDetailPage.tsx
      DockerPage.tsx
      SettingsPage.tsx
    lib/
      api.ts
      types.ts
      format.ts
      constants.ts
      storage.ts
    styles/
      globals.css
      layout.css
      components.css
  src-tauri/
    Cargo.toml
    tauri.conf.json
    capabilities/
      default.json
    src/
      main.rs
      errors.rs
      commands/
        mod.rs
        wsl.rs
        docker.rs
        system.rs
      services/
        mod.rs
        process.rs
        wsl_parser.rs
        port_parser.rs
        docker_parser.rs
      models/
        mod.rs
        command.rs
        distro.rs
        resource.rs
        port.rs
        docker.rs
```

---

## 7. 初始化项目

创建项目：

```powershell
pnpm create tauri-app wsl-dev-center
```

选择：

```text
React
TypeScript
pnpm
Tauri 2
```

进入目录：

```powershell
cd wsl-dev-center
pnpm install
pnpm tauri dev
```

要求：

```text
1. 确认开发窗口可以正常打开
2. 确认 Rust 编译正常
3. 确认前端热更新正常
4. 确认 Tauri invoke 可用
```

---

## 8. 数据模型

### 8.1 前端类型

创建：

```text
src/lib/types.ts
```

内容：

```ts
export type WslVersion = 1 | 2 | null;

export type DistroState = "Running" | "Stopped" | "Installing" | "Unknown";

export interface WslDistro {
  name: string;
  state: DistroState;
  version: WslVersion;
  isDefault: boolean;
}

export interface CommandOutput {
  success: boolean;
  code?: number | null;
  stdout: string;
  stderr: string;
}

export interface DistroResourceInfo {
  distro: string;
  memoryText?: string;
  diskText?: string;
  uptimeText?: string;
  processCount?: number;
  errors?: string[];
}

export interface PortInfo {
  protocol: "tcp" | "udp" | "unknown";
  localAddress: string;
  port: number;
  processName?: string;
  pid?: number;
  raw: string;
}

export interface DockerContainer {
  id: string;
  image: string;
  command?: string;
  created?: string;
  status: string;
  ports?: string;
  names: string;
}
```

---

### 8.2 Rust 模型

创建：

```text
src-tauri/src/models/mod.rs
```

```rust
pub mod command;
pub mod distro;
pub mod resource;
pub mod port;
pub mod docker;
```

创建：

```text
src-tauri/src/models/command.rs
```

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CommandOutput {
    pub success: bool,
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}
```

创建：

```text
src-tauri/src/models/distro.rs
```

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WslDistro {
    pub name: String,
    pub state: DistroState,
    pub version: Option<u8>,
    pub is_default: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum DistroState {
    Running,
    Stopped,
    Installing,
    Unknown,
}
```

创建：

```text
src-tauri/src/models/resource.rs
```

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DistroResourceInfo {
    pub distro: String,
    pub memory_text: Option<String>,
    pub disk_text: Option<String>,
    pub uptime_text: Option<String>,
    pub process_count: Option<u32>,
    pub errors: Vec<String>,
}
```

创建：

```text
src-tauri/src/models/port.rs
```

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PortInfo {
    pub protocol: String,
    pub local_address: String,
    pub port: u16,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    pub raw: String,
}
```

创建：

```text
src-tauri/src/models/docker.rs
```

```rust
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainer {
    pub id: String,
    pub image: String,
    pub command: Option<String>,
    pub created: Option<String>,
    pub status: String,
    pub ports: Option<String>,
    pub names: String,
}
```

---

## 9. Rust 命令执行层

创建：

```text
src-tauri/src/services/mod.rs
```

```rust
pub mod process;
pub mod wsl_parser;
pub mod port_parser;
pub mod docker_parser;
```

创建：

```text
src-tauri/src/services/process.rs
```

实现通用命令执行：

```rust
use std::process::Command;
use crate::models::command::CommandOutput;

pub fn clean_output(input: &[u8]) -> String {
    String::from_utf8_lossy(input)
        .replace('\u{0}', "")
        .trim()
        .to_string()
}

pub fn run_command(program: &str, args: &[&str]) -> Result<CommandOutput, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run {}: {}", program, e))?;

    Ok(CommandOutput {
        success: output.status.success(),
        code: output.status.code(),
        stdout: clean_output(&output.stdout),
        stderr: clean_output(&output.stderr),
    })
}
```

后续可以增加 timeout。MVP 可以先不做，但必须预留。

要求：

```text
1. 不允许前端传完整 shell command
2. 不允许直接拼接用户输入到 PowerShell 命令
3. 所有命令必须使用 Command::new + args
4. WSL distro name 只能作为单独参数传入
5. 所有 stdout/stderr 必须清理空字符
```

---

## 10. WSL 实例列表

### 10.1 命令

使用：

```powershell
wsl.exe -l -v
```

备用：

```powershell
wsl.exe --list --verbose
```

### 10.2 Parser

创建：

```text
src-tauri/src/services/wsl_parser.rs
```

实现：

```rust
use crate::models::distro::{DistroState, WslDistro};

pub fn parse_wsl_list_verbose(output: &str) -> Vec<WslDistro> {
    let mut distros = Vec::new();

    for raw_line in output.lines() {
        let line = raw_line.replace('\u{0}', "").trim().to_string();

        if line.is_empty() {
            continue;
        }

        if line.to_lowercase().contains("name")
            && line.to_lowercase().contains("state")
            && line.to_lowercase().contains("version")
        {
            continue;
        }

        let is_default = line.starts_with('*');
        let line = line.trim_start_matches('*').trim();

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let version_str = parts[parts.len() - 1];
        let state_str = parts[parts.len() - 2];
        let name_parts = &parts[0..parts.len() - 2];
        let name = name_parts.join(" ");

        let version = version_str.parse::<u8>().ok();

        let state = match state_str {
            "Running" => DistroState::Running,
            "Stopped" => DistroState::Stopped,
            "Installing" => DistroState::Installing,
            _ => DistroState::Unknown,
        };

        distros.push(WslDistro {
            name,
            state,
            version,
            is_default,
        });
    }

    distros
}
```

注意：

```text
1. distro 名称可能包含空格
2. 输出可能包含空字符
3. 表头要跳过
4. 不要因为单行解析失败导致整体失败
```

---

## 11. WSL Commands

创建：

```text
src-tauri/src/commands/mod.rs
```

```rust
pub mod wsl;
pub mod docker;
pub mod system;
```

创建：

```text
src-tauri/src/commands/wsl.rs
```

实现以下 Tauri command：

```rust
use crate::models::command::CommandOutput;
use crate::models::distro::WslDistro;
use crate::models::resource::DistroResourceInfo;
use crate::models::port::PortInfo;
use crate::services::process::run_command;
use crate::services::wsl_parser::parse_wsl_list_verbose;

#[tauri::command]
pub fn list_wsl_distros() -> Result<Vec<WslDistro>, String> {
    let output = run_command("wsl.exe", &["-l", "-v"])?;

    if !output.success {
        return Err(format!("failed to list WSL distros: {}", output.stderr));
    }

    Ok(parse_wsl_list_verbose(&output.stdout))
}

#[tauri::command]
pub fn start_distro(name: String) -> Result<CommandOutput, String> {
    run_command("wsl.exe", &["-d", &name, "--", "echo", "ok"])
}

#[tauri::command]
pub fn terminate_distro(name: String) -> Result<CommandOutput, String> {
    run_command("wsl.exe", &["--terminate", &name])
}

#[tauri::command]
pub fn restart_distro(name: String) -> Result<CommandOutput, String> {
    let stop = run_command("wsl.exe", &["--terminate", &name])?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    let start = run_command("wsl.exe", &["-d", &name, "--", "echo", "ok"])?;

    Ok(CommandOutput {
        success: stop.success && start.success,
        code: start.code,
        stdout: format!("{}\n{}", stop.stdout, start.stdout),
        stderr: format!("{}\n{}", stop.stderr, start.stderr),
    })
}

#[tauri::command]
pub fn shutdown_wsl() -> Result<CommandOutput, String> {
    run_command("wsl.exe", &["--shutdown"])
}

#[tauri::command]
pub fn open_terminal(name: String) -> Result<(), String> {
    let result = std::process::Command::new("wt.exe")
        .args(["wsl.exe", "-d", &name])
        .spawn();

    match result {
        Ok(_) => Ok(()),
        Err(_) => {
            std::process::Command::new("powershell.exe")
                .args(["-NoExit", "-Command", &format!("wsl.exe -d \"{}\"", name)])
                .spawn()
                .map_err(|e| format!("failed to open terminal: {}", e))?;
            Ok(())
        }
    }
}

#[tauri::command]
pub fn open_home_in_explorer(name: String) -> Result<(), String> {
    let path = format!("\\\\wsl$\\{}\\home", name);

    std::process::Command::new("explorer.exe")
        .arg(path)
        .spawn()
        .map_err(|e| format!("failed to open explorer: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn open_vscode_home(name: String) -> Result<(), String> {
    let remote = format!("wsl+{}", name);

    std::process::Command::new("code")
        .args(["--remote", &remote, "/home"])
        .spawn()
        .map_err(|_| "VS Code CLI not found. Please install VS Code and enable the code command in PATH.".to_string())?;

    Ok(())
}
```

注意：

```text
open_terminal 中 PowerShell fallback 暂时用了 format。
该命令只用于打开固定 wsl.exe -d "<name>"，风险较低。
如果要更安全，后续改为生成参数数组或使用 cmd start。
```

---

## 12. 注册 Tauri Commands

修改：

```text
src-tauri/src/main.rs
```

示例：

```rust
mod commands;
mod errors;
mod models;
mod services;

use commands::wsl::{
    list_wsl_distros,
    start_distro,
    terminate_distro,
    restart_distro,
    shutdown_wsl,
    open_terminal,
    open_home_in_explorer,
    open_vscode_home,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_wsl_distros,
            start_distro,
            terminate_distro,
            restart_distro,
            shutdown_wsl,
            open_terminal,
            open_home_in_explorer,
            open_vscode_home,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 13. 前端 API 封装

创建：

```text
src/lib/api.ts
```

```ts
import { invoke } from "@tauri-apps/api/core";
import type {
  WslDistro,
  CommandOutput,
  DistroResourceInfo,
  PortInfo,
  DockerContainer,
} from "./types";

export async function listWslDistros(): Promise<WslDistro[]> {
  return invoke<WslDistro[]>("list_wsl_distros");
}

export async function startDistro(name: string): Promise<CommandOutput> {
  return invoke<CommandOutput>("start_distro", { name });
}

export async function terminateDistro(name: string): Promise<CommandOutput> {
  return invoke<CommandOutput>("terminate_distro", { name });
}

export async function restartDistro(name: string): Promise<CommandOutput> {
  return invoke<CommandOutput>("restart_distro", { name });
}

export async function shutdownWsl(): Promise<CommandOutput> {
  return invoke<CommandOutput>("shutdown_wsl");
}

export async function openTerminal(name: string): Promise<void> {
  return invoke<void>("open_terminal", { name });
}

export async function openHomeInExplorer(name: string): Promise<void> {
  return invoke<void>("open_home_in_explorer", { name });
}

export async function openVscodeHome(name: string): Promise<void> {
  return invoke<void>("open_vscode_home", { name });
}

export async function getDistroResourceInfo(name: string): Promise<DistroResourceInfo> {
  return invoke<DistroResourceInfo>("get_distro_resource_info", { name });
}

export async function listPorts(name: string): Promise<PortInfo[]> {
  return invoke<PortInfo[]>("list_ports", { name });
}

export async function listDockerContainers(name: string): Promise<DockerContainer[]> {
  return invoke<DockerContainer[]>("list_docker_containers", { name });
}
```

---

## 14. 前端页面规划

### 14.1 AppShell

创建：

```text
src/components/AppShell.tsx
```

布局：

```text
左侧 Sidebar
右侧 Main
顶部 Header
主体 Content
```

Sidebar 菜单：

```text
Dashboard
Machines
Docker
Settings
```

---

### 14.2 DashboardPage

创建：

```text
src/pages/DashboardPage.tsx
```

显示：

```text
Running distros count
Stopped distros count
Total distros count
Docker available count
Open ports count
Recent actions
```

MVP 中 Recent actions 可以只存在前端内存，不需要持久化。

---

### 14.3 MachinesPage

创建：

```text
src/pages/MachinesPage.tsx
```

功能：

```text
1. 页面加载时调用 listWslDistros
2. 显示所有 distros
3. 支持手动 Refresh
4. 每 5 秒自动刷新一次
5. 正在操作某个 distro 时，该行按钮显示 loading
6. 操作完成后重新刷新
```

表格列：

```text
Name
State
Version
Default
Actions
```

Actions：

```text
Start
Stop
Restart
Terminal
Files
VS Code
Details
```

状态颜色：

```text
Running: green
Stopped: gray
Installing: yellow
Unknown: red
```

危险操作：

```text
Shutdown All
```

点击后必须弹确认框：

```text
This will stop all WSL distributions. Continue?
Cancel / Shutdown
```

---

### 14.4 MachineDetailPage

创建：

```text
src/pages/MachineDetailPage.tsx
```

路由：

```text
/machines/:name
```

显示：

```text
Header:
  distro name
  state
  version
  quick actions

Tabs 或 sections:
  Overview
  Ports
  Docker
```

Overview：

```text
Memory
Disk
Uptime
Process count
```

Ports：

```text
Port table
Refresh button
Copy localhost:port
Open browser for likely HTTP ports
```

Docker：

```text
Docker containers
Start / Stop / Logs
```

---

### 14.5 DockerPage

创建：

```text
src/pages/DockerPage.tsx
```

MVP 可以简单实现：

```text
1. 列出所有 Running 的 WSL distro
2. 用户选择一个 distro
3. 显示该 distro 的 Docker 容器
```

不要跨 distro 自动疯狂扫描 Docker。默认只扫描用户选择的 distro。

---

### 14.6 SettingsPage

创建：

```text
src/pages/SettingsPage.tsx
```

设置项：

```text
Refresh interval: manual / 3s / 5s / 10s
Enable port scan: true / false
Enable Docker panel: true / false
Default terminal: Windows Terminal / PowerShell
Show stopped distros: true / false
Theme: system / light / dark
```

配置存储：

```text
localStorage
```

不要引入数据库。

不要引入账号。

不要引入云同步。

---

## 15. Resource Info

### 15.1 Rust command

在：

```text
src-tauri/src/commands/wsl.rs
```

增加：

```rust
#[tauri::command]
pub fn get_distro_resource_info(name: String) -> Result<DistroResourceInfo, String> {
    let mut errors = Vec::new();

    let memory = run_command("wsl.exe", &["-d", &name, "--", "free", "-h"]);
    let disk = run_command("wsl.exe", &["-d", &name, "--", "df", "-h", "/"]);
    let uptime = run_command("wsl.exe", &["-d", &name, "--", "uptime", "-p"]);
    let process_count = run_command("wsl.exe", &[
        "-d",
        &name,
        "--",
        "sh",
        "-lc",
        "ps -e --no-headers | wc -l",
    ]);

    let memory_text = match memory {
        Ok(out) if out.success => Some(out.stdout),
        Ok(out) => {
            errors.push(out.stderr);
            None
        }
        Err(e) => {
            errors.push(e);
            None
        }
    };

    let disk_text = match disk {
        Ok(out) if out.success => Some(out.stdout),
        Ok(out) => {
            errors.push(out.stderr);
            None
        }
        Err(e) => {
            errors.push(e);
            None
        }
    };

    let uptime_text = match uptime {
        Ok(out) if out.success => Some(out.stdout),
        Ok(out) => {
            errors.push(out.stderr);
            None
        }
        Err(e) => {
            errors.push(e);
            None
        }
    };

    let process_count = match process_count {
        Ok(out) if out.success => out.stdout.trim().parse::<u32>().ok(),
        Ok(out) => {
            errors.push(out.stderr);
            None
        }
        Err(e) => {
            errors.push(e);
            None
        }
    };

    Ok(DistroResourceInfo {
        distro: name,
        memory_text,
        disk_text,
        uptime_text,
        process_count,
        errors,
    })
}
```

记得注册 command。

### 15.2 前端显示要求

如果某项失败，不要让整个页面崩溃。

显示：

```text
Memory: unavailable
Disk: unavailable
Uptime: unavailable
```

并在底部 ErrorBox 展示 errors。

---

## 16. 端口扫描

### 16.1 Rust parser

创建：

```text
src-tauri/src/services/port_parser.rs
```

实现一个基础 parser。

输入示例：

```text
tcp   LISTEN 0 511 0.0.0.0:3000 0.0.0.0:* users:(("node",pid=1234,fd=22))
tcp   LISTEN 0 128 127.0.0.1:6379 0.0.0.0:* users:(("redis-server",pid=5678,fd=6))
udp   UNCONN 0 0 127.0.0.53%lo:53 0.0.0.0:*
```

输出：

```rust
Vec<PortInfo>
```

解析原则：

```text
1. 只保留 LISTEN / UNCONN 中有本地端口的行
2. 优先解析 tcp/udp
3. 从 local address 中解析最后一个 : 后面的端口
4. IPv6 地址要尽力处理，解析失败就跳过
5. process name 从 users:(("xxx",pid=123 中解析
6. pid 从 pid=123 中解析
7. raw 保留原始行
```

### 16.2 Rust command

在：

```text
src-tauri/src/commands/wsl.rs
```

增加：

```rust
#[tauri::command]
pub fn list_ports(name: String) -> Result<Vec<PortInfo>, String> {
    let ss = run_command("wsl.exe", &["-d", &name, "--", "ss", "-tulpn"]);

    match ss {
        Ok(out) if out.success => {
            return Ok(crate::services::port_parser::parse_ports(&out.stdout));
        }
        _ => {}
    }

    let netstat = run_command("wsl.exe", &["-d", &name, "--", "netstat", "-tulpn"]);

    match netstat {
        Ok(out) if out.success => Ok(crate::services::port_parser::parse_ports(&out.stdout)),
        Ok(out) => Err(format!("failed to list ports: {}", out.stderr)),
        Err(e) => Err(e),
    }
}
```

### 16.3 前端 PortTable

字段：

```text
Protocol
Address
Port
Process
PID
Action
```

Action：

```text
Copy localhost:port
Open
```

Open 只对以下端口默认显示：

```text
80
3000
3001
5173
5174
8000
8080
9000
```

非 HTTP 端口只显示 Copy。

---

## 17. Docker 容器管理

### 17.1 Docker 检测

命令：

```powershell
wsl.exe -d <DistroName> -- docker version
```

如果失败：

```text
Docker not found in this distro.
```

### 17.2 列出容器

命令：

```powershell
wsl.exe -d <DistroName> -- docker ps -a --format "{{json .}}"
```

### 17.3 Rust parser

创建：

```text
src-tauri/src/services/docker_parser.rs
```

每一行是 JSON：

```json
{"ID":"abc","Image":"redis","Command":"...","CreatedAt":"...","Status":"Up 2 hours","Ports":"6379/tcp","Names":"redis"}
```

解析成：

```rust
DockerContainer
```

需要兼容字段：

```text
ID
Image
Command
CreatedAt
Status
Ports
Names
```

### 17.4 Rust command

创建：

```text
src-tauri/src/commands/docker.rs
```

实现：

```rust
#[tauri::command]
pub fn list_docker_containers(name: String) -> Result<Vec<DockerContainer>, String> {
    // wsl.exe -d name -- docker ps -a --format "{{json .}}"
}

#[tauri::command]
pub fn start_container(distro: String, container: String) -> Result<CommandOutput, String> {
    // container 必须作为单独参数
    // wsl.exe -d distro -- docker start container
}

#[tauri::command]
pub fn stop_container(distro: String, container: String) -> Result<CommandOutput, String> {
    // wsl.exe -d distro -- docker stop container
}

#[tauri::command]
pub fn container_logs(distro: String, container: String) -> Result<CommandOutput, String> {
    // wsl.exe -d distro -- docker logs --tail 200 container
}
```

安全要求：

```text
1. container id/name 必须来自 Docker 列表
2. 前端不要提供任意 docker command 输入框
3. 不要支持 docker rm
4. 不要支持 docker rmi
5. 不要支持 volume 删除
```

MVP 只允许：

```text
start
stop
logs
```

---

## 18. UI 样式

视觉方向：

```text
清爽
高密度
开发者工具风
类似 OrbStack 的机器卡片体验
但不要照抄 OrbStack
```

布局：

```text
左侧 Sidebar
右侧内容区
顶部 Header
卡片式 Dashboard
表格式 Machines
详情页分区展示
```

颜色：

```text
支持 light
支持 dark
默认跟随系统
```

CSS 变量：

```css
:root {
  --bg: #f6f7f9;
  --panel: #ffffff;
  --text: #111827;
  --muted: #6b7280;
  --border: #e5e7eb;
  --success: #16a34a;
  --warning: #d97706;
  --danger: #dc2626;
  --radius: 14px;
}
```

暗色可以用：

```css
[data-theme="dark"] {
  --bg: #0f1115;
  --panel: #171a21;
  --text: #f9fafb;
  --muted: #9ca3af;
  --border: #2f3542;
}
```

---

## 19. 错误处理规范

所有错误都要有：

```text
用户可读信息
原始 stderr
命令退出码
发生位置
```

前端 ErrorBox 显示：

```text
Operation failed
Reason: xxx
Details: xxx
```

不要只显示：

```text
Error
failed
unknown
```

常见错误文案：

```text
WSL is not installed.
No WSL distributions found.
This distribution is not running.
Docker is not installed in this distribution.
Windows Terminal was not found. Falling back to PowerShell.
VS Code CLI was not found. Please enable the code command in PATH.
Failed to open \\wsl$ path.
```

---

## 20. 安全规范

强制要求：

```text
1. 不允许提供任意命令执行框
2. 不允许前端传完整 shell 命令给后端
3. 不允许自动 sudo
4. 不允许自动修改 /etc/*
5. 不允许自动修改 Windows hosts
6. 不允许自动修改 Windows 防火墙
7. 不允许自动安装证书
8. 不允许自动上传任何数据
9. 不允许静默执行 wsl --shutdown
10. 不允许静默删除 WSL 实例
11. 不允许静默 unregister
12. 不允许静默删除 Docker container/image/volume
```

所有危险操作必须二次确认。

MVP 危险操作只有：

```text
wsl --shutdown
```

确认框文案：

```text
This will stop all running WSL distributions. Continue?
```

按钮：

```text
Cancel
Shutdown WSL
```

默认焦点必须在 Cancel。

---

## 21. 自动刷新策略

默认刷新：

```text
MachinesPage: 5 秒
MachineDetailPage: 手动刷新 + 进入页面时刷新
Ports: 手动刷新
Docker: 手动刷新
```

不要做：

```text
每秒刷新
同时扫描所有 distro 的端口
同时扫描所有 distro 的 Docker
后台疯狂轮询
```

Settings 中允许：

```text
Manual
3 seconds
5 seconds
10 seconds
```

最低不能低于 3 秒。

---

## 22. 开发顺序

严格按以下顺序开发。

### Step 1: 初始化项目

```text
1. 创建 Tauri 2 + React + TypeScript 项目
2. 确认 pnpm tauri dev 可运行
3. 清理模板无用代码
4. 建立基础目录结构
```

完成后提交：

```bash
git add .
git commit -m "init tauri react project"
```

---

### Step 2: 建立 Rust 基础层

```text
1. 创建 models
2. 创建 services
3. 创建 commands
4. 创建 process.rs
5. 实现 run_command
6. 注册基础 command 测试 invoke
```

提交：

```bash
git add .
git commit -m "add rust command foundation"
```

---

### Step 3: 实现 WSL 列表

```text
1. 实现 parse_wsl_list_verbose
2. 实现 list_wsl_distros command
3. 前端封装 listWslDistros
4. MachinesPage 显示数据
5. 处理 loading/error/empty
```

提交：

```bash
git add .
git commit -m "implement wsl distro listing"
```

---

### Step 4: 实现 WSL 操作

```text
1. start_distro
2. terminate_distro
3. restart_distro
4. shutdown_wsl
5. 前端按钮
6. 操作后自动刷新
7. Shutdown All 确认框
```

提交：

```bash
git add .
git commit -m "implement wsl lifecycle actions"
```

---

### Step 5: 实现终端和文件入口

```text
1. open_terminal
2. open_home_in_explorer
3. open_vscode_home
4. 前端按钮
5. fallback 和错误提示
```

提交：

```bash
git add .
git commit -m "add terminal and file shortcuts"
```

---

### Step 6: 实现实例详情页

```text
1. 创建 MachineDetailPage
2. 建立 /machines/:name 路由
3. 显示基本信息
4. 实现 get_distro_resource_info
5. 显示 memory/disk/uptime/process count
```

提交：

```bash
git add .
git commit -m "add machine detail overview"
```

---

### Step 7: 实现端口扫描

```text
1. 实现 port_parser
2. 实现 list_ports command
3. 支持 ss
4. 支持 netstat fallback
5. 创建 PortTable
6. 支持 Copy localhost:port
7. 支持常见 HTTP 端口 Open
```

提交：

```bash
git add .
git commit -m "add wsl port scanner"
```

---

### Step 8: 实现 Docker 容器列表

```text
1. 实现 docker_parser
2. 实现 list_docker_containers
3. 实现 start_container
4. 实现 stop_container
5. 实现 container_logs
6. 创建 DockerTable
7. 无 Docker 时清晰提示
```

提交：

```bash
git add .
git commit -m "add docker container panel"
```

---

### Step 9: 实现 Settings

```text
1. 创建 SettingsPage
2. localStorage 保存配置
3. 刷新间隔配置
4. 是否启用端口扫描
5. 是否启用 Docker 面板
6. 主题配置
```

提交：

```bash
git add .
git commit -m "add local settings"
```

---

### Step 10: UI 整理和打包

```text
1. 优化 AppShell
2. 优化 Sidebar
3. 优化 Dashboard
4. 优化 MachinesPage
5. 优化 DetailPage
6. 检查错误提示
7. 检查 loading 状态
8. 执行 pnpm tauri build
```

提交：

```bash
git add .
git commit -m "polish ui and prepare build"
```

---

## 23. 测试清单

必须逐项测试。

### 基础测试

```text
[ ] 应用可以启动
[ ] 应用没有白屏
[ ] Sidebar 可以切换页面
[ ] Dashboard 可以显示
[ ] Machines 可以显示
[ ] Settings 可以显示
```

### WSL 测试

```text
[ ] 可以列出所有 WSL 实例
[ ] 可以识别 Running
[ ] 可以识别 Stopped
[ ] 可以识别 WSL1
[ ] 可以识别 WSL2
[ ] 可以识别默认实例
[ ] 实例名称带空格时不崩溃
[ ] 没有 WSL 时有清晰提示
```

### 操作测试

```text
[ ] Stopped 实例可以 Start
[ ] Running 实例可以 Stop
[ ] 实例可以 Restart
[ ] 操作后状态自动刷新
[ ] Shutdown All 有确认框
[ ] Shutdown All 可以取消
[ ] 命令失败时显示错误
```

### 快捷入口测试

```text
[ ] 可以打开 Windows Terminal
[ ] wt.exe 不存在时 fallback 到 PowerShell
[ ] 可以打开 \\wsl$\Distro\home
[ ] 可以打开 VS Code Remote WSL
[ ] code 命令不存在时提示清楚
```

### 详情页测试

```text
[ ] 可以进入详情页
[ ] 可以显示内存信息
[ ] 可以显示磁盘信息
[ ] 可以显示 uptime
[ ] 可以显示进程数量
[ ] 实例未运行时提示清楚
```

### 端口测试

```text
[ ] 可以显示 ss -tulpn 结果
[ ] 没有 ss 时可以 fallback netstat
[ ] 可以识别 tcp
[ ] 可以识别 udp
[ ] 可以识别 port
[ ] 可以识别 process name
[ ] 可以识别 pid
[ ] 可以复制 localhost:port
[ ] HTTP 常见端口可以 Open
```

### Docker 测试

```text
[ ] Docker 存在时可以列出容器
[ ] Docker 不存在时提示清楚
[ ] 可以 start container
[ ] 可以 stop container
[ ] 可以查看最近 200 行 logs
[ ] 不存在删除 container/image/volume 功能
```

### 安全测试

```text
[ ] 前端没有任意命令输入框
[ ] 前端没有登录页
[ ] 前端没有注册页
[ ] 前端没有云同步设置
[ ] 前端没有 HTTPS 设置
[ ] 没有 telemetry
[ ] 没有上传本地数据
[ ] 没有修改 hosts
[ ] 没有安装证书
```

---

## 24. 后续版本规划

### v0.2

```text
支持 export distro
支持 import distro
支持 clone distro
支持 distro 模板
支持项目快捷入口
支持最近打开的项目
```

仍然不做：

```text
账号
云同步
HTTPS
```

---

### v0.3

```text
支持 docker compose ls
支持 compose up
支持 compose down
支持 compose logs
支持 compose 项目面板
```

限制：

```text
不提供任意 docker 命令输入框
不默认删除 volume
不默认删除 image
```

---

### v0.4

```text
支持 HTTP 本地端口代理
支持 localhost 域名规则
支持项目端口收藏
```

限制：

```text
只做 HTTP
不做 HTTPS
不做证书
不做公网暴露
不做远程访问
```

---

### v0.5

```text
支持 VHDX 大小查看
支持 WSL export 备份入口
支持手动 compact 提示
支持磁盘占用分析
```

限制：

```text
不自动压缩磁盘
不自动删除数据
不自动备份到云端
```

---

## 25. 明确禁止的实现

开发过程中如果 AI 试图添加以下内容，必须立即停止并删除：

```text
auth.ts
login.tsx
register.tsx
user.ts
cloud.ts
sync.ts
https.ts
certificate.ts
telemetry.ts
analytics.ts
oauth.ts
workspace.ts
team.ts
billing.ts
subscription.ts
```

禁止依赖：

```text
firebase
supabase
auth0
clerk
sentry
posthog
segment
google analytics
```

除非用户未来明确要求，否则不要加入任何外部服务 SDK。

---

## 26. 最终交付物

最终需要交付：

```text
1. 完整源码
2. 可运行的 pnpm tauri dev
3. 可构建的 pnpm tauri build
4. Windows 安装包
5. README.md
6. 基础截图
```

README.md 必须包含：

```text
项目介绍
功能列表
安装依赖
开发命令
构建命令
安全边界
明确说明本项目不含账号、不含云同步、不含 HTTPS
```

---

## 27. README.md 必须声明

README 中必须包含以下段落：

```md
## Privacy and Local-Only Design

WSL Dev Center is a local-only Windows desktop application.

It does not include:

- user accounts
- login or registration
- cloud sync
- remote management
- telemetry
- HTTPS certificate management
- public network exposure

All WSL, Docker, port, and process information stays on the local machine.
```

---

## 28. 开发执行要求

AI 执行本计划时必须遵守：

```text
1. 按 Step 顺序开发
2. 每完成一个 Step 做一次 git commit
3. 不跳步
4. 不擅自扩展账号/云/HTTPS
5. 不引入数据库
6. 不引入外部 SaaS
7. 不引入遥测
8. 遇到命令错误先修复，不绕过
9. 保持 MVP 简洁
10. 优先可用，再优化 UI
```

---

## 29. 最重要的判断

本项目的第一版不是为了做“大而全平台”。

第一版只解决一个问题：

```text
让我在 Windows 上更舒服地管理 WSL 开发环境。
```

所以 MVP 的成功标准不是功能多，而是：

```text
打开快
列表准
操作稳
错误清楚
不越界
不联网
不登录
不搞证书
```
