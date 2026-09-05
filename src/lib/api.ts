import { invoke } from "@tauri-apps/api/core";
import type { CommandOutput, DockerContainer, DistroResourceInfo, PortInfo, WslDistro } from "./types";
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
export const openTerminal = (name: string) => invoke<void>("open_terminal", { name });
export const openHome = (name: string) => invoke<void>("open_home_in_explorer", { name });
export const openVscode = (name: string) => invoke<void>("open_vscode_home", { name });
export const resources = (name: string) => invoke<DistroResourceInfo>("get_distro_resource_info", { name });
export const ports = (name: string) => invoke<PortInfo[]>("list_ports", { name });
export const containers = (name: string) => invoke<DockerContainer[]>("list_docker_containers", { name });
