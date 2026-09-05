export type DistroState = "Running" | "Stopped" | "Installing" | "Unknown";
export interface WslDistro { name: string; state: DistroState; version: number | null; isDefault: boolean; }
export interface OnlineDistro { name: string; friendlyName: string; }
export interface CommandOutput { success: boolean; code?: number | null; stdout: string; stderr: string; }
export interface DistroResourceInfo { distro: string; osVersionText?: string; kernelVersionText?: string; cpuText?: string; memoryText?: string; diskText?: string; uptimeText?: string; processCount?: number; errors: string[]; }
export interface PortInfo { protocol: string; localAddress: string; port: number; processName?: string; pid?: number; raw: string; }
export interface DockerContainer { id: string; image: string; status: string; ports?: string; names: string; }
