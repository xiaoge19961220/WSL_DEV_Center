import { useRef, useState } from "react";
import * as api from "../lib/api";
import type { WslDistro } from "../lib/types";
import { Status } from "./Feedback";
import { ConfirmDialog } from "./ConfirmDialog";
import { DistroOperationDialog, type DistroOperation } from "./DistroOperationDialog";

export function Machines({ distros, refresh, select, error }: {
  distros: WslDistro[]; refresh: () => Promise<void>;
  select: (d: WslDistro) => void; error: (v: string) => void;
}) {
  const [busy, setBusy] = useState("");
  const [notice, setNotice] = useState("");
  const [operation, setOperation] = useState<DistroOperation>();
  const [deleteName, setDeleteName] = useState("");
  const lock = useRef(false);
  const run = async (name: string, fn: () => Promise<unknown>) => {
    if (lock.current) return;
    lock.current = true; setBusy(name); error(""); setNotice("");
    try { const result = await fn(); if (typeof result === "string") setNotice(result); await refresh(); }
    catch (e) { error(String(e)); }
    finally { lock.current = false; setBusy(""); }
  };
  const complete = async (message: string) => { setOperation(undefined); setNotice(message); await refresh(); };
  return <><div className="machine-tools"><button onClick={() => setOperation({ kind: "install" })}>安装新实例</button><button onClick={() => setOperation({ kind: "import" })}>导入实例</button></div>
    <p role="status">{notice}</p>{!distros.length ? <p className="muted">没有可显示的 WSL 实例。可以安装在线发行版或导入已有备份。</p> :
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
          <button disabled={!!busy} onClick={() => setOperation({ kind: "clone", name: d.name })}>复制</button>
          <button disabled={!!busy} onClick={() => setOperation({ kind: "export", name: d.name })}>导出</button>
          <button className="danger-link" disabled={!!busy} onClick={() => setDeleteName(d.name)}>删除</button>
          {busy === d.name && <span role="status">正在操作…</span>}
        </div></td>
      </tr>)}</tbody></table></div>}
    {operation && <DistroOperationDialog operation={operation} cancel={() => setOperation(undefined)} complete={complete} />}
    {deleteName && <ConfirmDialog title={`删除 ${deleteName}？`} message={`这会永久删除实例“${deleteName}”及其中全部文件，且无法撤销。请先确认重要数据已经导出。`} confirmLabel="确认删除" busyLabel="正在删除…" cancel={() => setDeleteName("")} confirm={async () => { await api.unregisterDistro(deleteName); const deleted = deleteName; setDeleteName(""); setNotice(`实例“${deleted}”已删除。`); await refresh(); }} />}
  </>;
}
