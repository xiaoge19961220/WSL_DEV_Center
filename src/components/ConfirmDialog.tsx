import { useEffect, useRef, useState } from "react";
import { ErrorBox } from "./Feedback";

export function ConfirmDialog({ cancel, confirm }: { cancel: () => void; confirm: () => Promise<void> }) {
  const dialog = useRef<HTMLDialogElement>(null);
  const cancelButton = useRef<HTMLButtonElement>(null);
  const lock = useRef(false);
  const [busy, setBusy] = useState(false), [error, setError] = useState("");
  useEffect(() => {
    const node = dialog.current;
    node?.showModal(); cancelButton.current?.focus();
    return () => node?.close();
  }, []);
  const submit = async () => {
    if (lock.current) return;
    lock.current = true; setBusy(true); setError("");
    try { await confirm(); }
    catch (e) { setError(String(e)); }
    finally { lock.current = false; setBusy(false); }
  };
  return <dialog ref={dialog} className="confirm-dialog" aria-labelledby="shutdown-title" onCancel={e => { e.preventDefault(); if (!busy) cancel(); }}>
    <h2 id="shutdown-title">关闭所有 WSL 实例？</h2>
    <p>这将停止所有正在运行的 WSL 实例，未保存的工作可能丢失。是否继续？</p>
    <ErrorBox message={error} />
    <footer><button ref={cancelButton} disabled={busy} onClick={cancel}>取消</button><button className="danger" disabled={busy} onClick={() => void submit()}>{busy ? "正在关闭…" : "确认关闭"}</button></footer>
  </dialog>;
}
