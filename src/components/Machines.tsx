import { useRef, useState } from "react";
import * as api from "../lib/api";
import type { WslDistro } from "../lib/types";
import { Status } from "./Feedback";

export function Machines({ distros, refresh, select, error }: {
  distros: WslDistro[]; refresh: () => Promise<void>;
  select: (d: WslDistro) => void; error: (v: string) => void;
}) {
  const [busy, setBusy] = useState("");
  const lock = useRef(false);
  const run = async (name: string, fn: () => Promise<unknown>) => {
    if (lock.current) return;
    lock.current = true; setBusy(name); error("");
    try { await fn(); await refresh(); }
    catch (e) { error(String(e)); }
    finally { lock.current = false; setBusy(""); }
  };
  return !distros.length ? <p className="muted">没有可显示的 WSL 实例。请检查“显示已停止的实例”设置，或安装 WSL 发行版。</p> :
    <div className="table"><table><thead><tr>{["名称", "状态", "版本", "默认", "操作"].map(t => <th key={t}>{t}</th>)}</tr></thead>
      <tbody>{distros.map(d => <tr key={d.name}>
        <td>{d.name}</td><td><Status state={d.state} /></td><td>WSL {d.version ?? "—"}</td><td>{d.isDefault ? "默认" : "—"}</td>
        <td><div className="actions" aria-busy={busy === d.name}>
          {(d.state === "Stopped" || d.state === "Running") && <button disabled={!!busy} onClick={() => void run(d.name, () => d.state === "Stopped" ? api.startDistro(d.name) : api.terminateDistro(d.name))}>{d.state === "Stopped" ? "启动" : "停止"}</button>}
          <button disabled={!!busy || d.state !== "Running"} onClick={() => void run(d.name, () => api.restartDistro(d.name))}>重启</button>
          <button disabled={!!busy} onClick={() => void run(d.name, () => api.openTerminal(d.name))}>终端</button>
          <button disabled={!!busy} onClick={() => void run(d.name, () => api.openHome(d.name))}>文件</button>
          <button disabled={!!busy} onClick={() => void run(d.name, () => api.openVscode(d.name))}>VS Code</button>
          <button disabled={!!busy} onClick={() => select(d)}>详情</button>
          {busy === d.name && <span role="status">正在操作…</span>}
        </div></td>
      </tr>)}</tbody></table></div>;
}
