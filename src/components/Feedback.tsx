import type { DistroState } from "../lib/types";
const labels: Record<DistroState, string> = { Running: "运行中", Stopped: "已停止", Installing: "安装中", Unknown: "未知" };
export function Status({ state }: { state: DistroState }) {
  return <span className={`status ${state.toLowerCase()}`}>● {labels[state] ?? "未知"}</span>;
}
export function ErrorBox({ message }: { message: string }) {
  return message ? <div className="error" role="alert"><strong>操作失败</strong><pre>{message}</pre></div> : null;
}
export function Loading() { return <p className="muted" role="status">正在加载，请稍候…</p>; }
