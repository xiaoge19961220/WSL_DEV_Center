# 项目进度与验收记录

更新时间：2026-09-05。依据根目录 PLAN.md 核对；主界面、提示及项目说明使用中文，后续关键节点使用中文 Git 提交说明。

## 本轮实现

| 计划步骤 | 代码状态 | 验证边界 |
| --- | --- | --- |
| Step 1 初始化 | Tauri 2 + React + TypeScript 已建立 | 前端构建通过；发布版进程启动检查通过 |
| Step 2 Rust 基础层 | models / services / commands 分层，UTF-16LE 解码，后台线程执行命令 | Rust 编译、格式检查与 7 项单元测试通过 |
| Step 3 WSL 列表 | 中文状态、名称空格、默认实例、版本、加载／失败／空状态、自动刷新 | 前端回归通过；真实 Ubuntu WSL2 列表与停止保护只读检查通过 |
| Step 4 生命周期 | 失败结果检查、停止失败时中止重启、行内忙状态、关闭确认 | 确认取消及错误处理测试通过；真实启停待验收 |
| Step 5 快捷入口 | Windows Terminal、PowerShell 回退、文件、VS Code | 实机入口待验收 |
| Step 6 详情 | 哈希路由、停止实例保护、资源分项错误、零进程数显示 | 前端保护与错误展示测试通过 |
| Step 7 端口 | ss、非零退出回退 netstat、TCP/UDP、IPv6、复制、常见 HTTP 打开 | Rust 解析测试通过；真实端口实机待验收 |
| Step 8 Docker | 大小写字段修复、列表、启动、停止、200 行日志、操作前核对容器 | 切换实例旧请求隔离测试通过；实际 Docker 待验收 |
| Step 9 设置 | 全部六类设置接通、localStorage 校验、系统主题 | 前端构建与设置校验测试通过；浅色页面已视觉检查 |
| Step 10 整理与交付 | 中文 README、基础截图、回归测试和窗口尺寸 | 简体中文 MSI 和 NSIS EXE 安装包已生成，安装流程待人工验收 |

## 已执行检查

- `pnpm test`：11 项通过。使用模拟接口，不会启动、停止 WSL 或操作真实容器。
- `pnpm build`：TypeScript 检查和前端生产构建通过。
- `cargo test --manifest-path src-tauri/Cargo.toml --locked -- --include-ignored --nocapture`：8 项通过（7 项单元测试 + 1 项只读 WSL 集成检查）。
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`：通过。
- `pnpm tauri build`：通过，生成简体中文 MSI 和 NSIS EXE 两种安装包。
- 发布版进程启动检查：运行 5 秒未提前退出，随后关闭本次测试进程；这不等于完整桌面 UI 验收。
- `git diff --check`：通过。
- 中文设置页：浏览器实际渲染检查通过，已保存浅色截图。

## 构建环境

Rust 1.98.1 已安装在用户的 `.cargo/bin`，当前任务的原 PATH 未包含该目录。已通过当前进程 PATH 使用工具链。

已安装微软官方 Visual Studio Build Tools 2022（17.14.39）的 C++ 工作负载及 Windows SDK，解决缺少 MSVC `link.exe` 的问题。随后后端测试与 Windows 打包均通过。构建工具安装正常退出，未执行系统重启。

本机只读查询结果：默认实例 `Ubuntu`，WSL2，状态 `Stopped`。集成测试验证了真实列表解析和停止实例保护，没有启动或停止用户实例。

## 安装包

- `src-tauri/target/release/bundle/nsis/WSL 开发中心_0.1.0_x64-setup.exe`（2,041,685 字节）。
- `src-tauri/target/release/bundle/msi/WSL 开发中心_0.1.0_x64_zh-CN.msi`（3,137,536 字节）。

安装向导配置为简体中文。生成文件保留在构建目录，不纳入 Git。

## 尚未完成的验收与限制

- 桌面完整界面、Tauri invoke 的端到端交互、安装包安装及安装后启动。
- 实际 WSL1/WSL2、中文系统输出、带空格名称、未安装 WSL 等实机情境。
- 实例启停、全局关闭、终端回退、文件与 VS Code 入口。
- 真实 Docker 可用／不存在／服务未启动／权限不足，以及容器启停和日志。
- 真实端口复制及浏览器访问、无 ss 时的 netstat 回退。
- 系统命令目前没有执行超时；卡住的命令不会阻塞 UI，但会使该操作持续等待。
- 概览页展示实例数量，Docker 为“按需查看”；尚未实现计划中的最近操作记录、端口总数和 Docker 可用实例数量。
- 详情页快捷操作目前集中在实例列表，详情有返回入口。
- 根目录原有未跟踪的 PLAN.md、package-lock.json 保留，未替用户删除或纳入提交。

原计划的人工测试清单未自动勾选；只有实际执行过的检查才能视为已验收。
