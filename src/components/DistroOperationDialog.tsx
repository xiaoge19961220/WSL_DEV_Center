import { type FormEvent, useEffect, useRef, useState } from "react";
import * as api from "../lib/api";
import type { OnlineDistro } from "../lib/types";
import { ErrorBox, Loading } from "./Feedback";

export type DistroOperation =
  | { kind: "install" }
  | { kind: "import" }
  | { kind: "export"; name: string }
  | { kind: "clone"; name: string };

const titles = { install: "安装新实例", import: "导入实例", export: "导出实例", clone: "复制实例" };

export function DistroOperationDialog({ operation, cancel, complete }: {
  operation: DistroOperation;
  cancel: () => void;
  complete: (message: string) => Promise<void>;
}) {
  const dialog = useRef<HTMLDialogElement>(null);
  const cancelButton = useRef<HTMLButtonElement>(null);
  const [online, setOnline] = useState<OnlineDistro[]>([]);
  const [loadingOnline, setLoadingOnline] = useState(operation.kind === "install");
  const [busy, setBusy] = useState(false), [error, setError] = useState("");
  const [distribution, setDistribution] = useState("");
  const [name, setName] = useState("");
  const [installLocation, setInstallLocation] = useState("");
  const [archivePath, setArchivePath] = useState("");
  const [vhd, setVhd] = useState(false);

  useEffect(() => {
    dialog.current?.showModal();
    cancelButton.current?.focus();
    return () => dialog.current?.close();
  }, []);
  useEffect(() => {
    if (operation.kind !== "install") return;
    let active = true;
    api.listOnlineDistros()
      .then(rows => { if (active) setOnline(Array.isArray(rows) ? rows : []); })
      .catch(e => { if (active) setError(String(e)); })
      .finally(() => { if (active) setLoadingOnline(false); });
    return () => { active = false; };
  }, [operation.kind]);

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy) return;
    setBusy(true); setError("");
    try {
      if (operation.kind === "install") {
        await api.installDistro(distribution);
        await complete(`已提交 ${distribution} 的安装任务。首次启动时可能需要完成用户初始化。`);
      } else if (operation.kind === "import") {
        await api.importDistro(name, installLocation, archivePath, vhd);
        await complete(`实例“${name}”已导入。`);
      } else if (operation.kind === "export") {
        await api.exportDistro(operation.name, archivePath, vhd);
        await complete(`实例“${operation.name}”已导出到 ${archivePath}。`);
      } else {
        await api.cloneDistro(operation.name, name, installLocation);
        await complete(`实例“${operation.name}”已复制为“${name}”。`);
      }
    } catch (e) { setError(String(e)); setBusy(false); }
  };

  return <dialog ref={dialog} className="operation-dialog" aria-labelledby="operation-title" onCancel={e => { e.preventDefault(); if (!busy) cancel(); }}>
    <form onSubmit={e => void submit(e)}>
      <h2 id="operation-title">{titles[operation.kind]}</h2>
      {operation.kind === "install" && <>
        <p className="muted">从 Microsoft 提供的在线发行版列表安装，过程可能需要网络和 Windows 管理权限。</p>
        {loadingOnline ? <Loading /> : <label>发行版<select required value={distribution} onChange={e => setDistribution(e.target.value)}>
          <option value="">请选择发行版</option>{online.map(item => <option key={item.name} value={item.name}>{item.friendlyName}（{item.name}）</option>)}
        </select></label>}
      </>}
      {operation.kind === "import" && <>
        <label>实例名称<input required value={name} onChange={e => setName(e.target.value)} /></label>
        <label>安装目录<input required placeholder="D:\\WSL\\我的实例" value={installLocation} onChange={e => setInstallLocation(e.target.value)} /></label>
        <label>导入文件<input required placeholder="D:\\备份\\实例.tar 或 .vhdx" value={archivePath} onChange={e => setArchivePath(e.target.value)} /></label>
        <label className="check"><input type="checkbox" checked={vhd} onChange={e => setVhd(e.target.checked)} />导入文件是 VHDX</label>
      </>}
      {operation.kind === "export" && <>
        <p>导出“{operation.name}”的完整文件系统。</p>
        <label>导出文件<input required placeholder="D:\\备份\\实例.tar" value={archivePath} onChange={e => setArchivePath(e.target.value)} /></label>
        <label className="check"><input type="checkbox" checked={vhd} onChange={e => setVhd(e.target.checked)} />导出为 VHDX（仅 WSL 2）</label>
      </>}
      {operation.kind === "clone" && <>
        <p>复制“{operation.name}”。应用会先导出临时 TAR，再导入为新实例。</p>
        <label>副本名称<input required value={name} onChange={e => setName(e.target.value)} /></label>
        <label>副本安装目录<input required placeholder="D:\\WSL\\实例副本" value={installLocation} onChange={e => setInstallLocation(e.target.value)} /></label>
      </>}
      <ErrorBox message={error} />
      <footer><button type="button" ref={cancelButton} disabled={busy} onClick={cancel}>取消</button><button type="submit" disabled={busy || loadingOnline}>{busy ? "正在执行…" : "开始"}</button></footer>
    </form>
  </dialog>;
}
