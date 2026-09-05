import { readSettings } from "./storage";
import { invoke as tauriInvoke, isTauri } from "@tauri-apps/api/core";
import type { CommandOutput, DockerContainer, DistroResourceInfo, PortInfo, WslDistro } from "./types";
async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) throw new Error("当前为浏览器预览，无法读取本机 WSL。请启动 WSL 开发中心桌面应用。");
  return tauriInvoke<T>(command, args);
}
async function action(command: string, args?: Record<string, unknown>): Promise<CommandOutput> {
  const output = await invoke<CommandOutput>(command, args);
  if (!output.success) throw new Error(`操作未完成\n位置：${command}\n退出码：${output.code ?? "不可用"}\n标准错误：${output.stderr}\n标准输出：${output.stdout}`);
  return output;
}
export const listWslDistros = () => invoke<WslDistro[]>("list_wsl_distros");
export const startDistro = (name: string) => action("start_distro", { name });
export const terminateDistro = (name: string) => action("terminate_distro", { name });
export const restartDistro = (name: string) => action("restart_distro", { name });
export const shutdownWsl = () => action("shutdown_wsl");
export const openTerminal = (name: string) => invoke<string>("open_terminal", { name, terminal: readSettings().terminal });
export const openHome = (name: string) => invoke<void>("open_home_in_explorer", { name });
export const openVscode = (name: string) => invoke<void>("open_vscode_home", { name });
export const resources = (name: string) => invoke<DistroResourceInfo>("get_distro_resource_info", { name });
export const ports = (name: string) => invoke<PortInfo[]>("list_ports", { name });
export const containers = (name: string) => invoke<DockerContainer[]>("list_docker_containers", { name });
export const startContainer = (distro: string, container: string) => action("start_container", { distro, container });
export const stopContainer = (distro: string, container: string) => action("stop_container", { distro, container });
export const containerLogs = (distro: string, container: string) => action("container_logs", { distro, container });
