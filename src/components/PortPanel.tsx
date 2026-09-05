import { useRef, useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import * as api from "../lib/api";
import type { PortInfo } from "../lib/types";
import { ErrorBox, Loading } from "./Feedback";

const httpPorts = [80, 3000, 3001, 5173, 5174, 8000, 8080, 9000];
export function PortPanel({ name }: { name: string }) {
  const [ports, setPorts] = useState<PortInfo[] | null>(null);
  const [busy, setBusy] = useState(false), [error, setError] = useState(""), [notice, setNotice] = useState("");
  const lock = useRef(false);
  const refresh = async () => {
    if (lock.current) return;
    lock.current = true; setBusy(true); setError(""); setNotice(""); setPorts(null);
    try { setPorts(await api.ports(name)); } catch (e) { setError(String(e)); }
    finally { lock.current = false; setBusy(false); }
  };
  const action = async (fn: () => Promise<void>, message: string) => {
    setError(""); setNotice("");
    try { await fn(); setNotice(message); } catch (e) { setError(`端口操作失败：${String(e)}`); }
  };
  return <section className="panel"><div className="toolbar"><h2>监听端口</h2><button disabled={busy} onClick={() => void refresh()}>刷新端口</button></div>
    <ErrorBox message={error} />{busy && <Loading />}{notice && <p role="status">{notice}</p>}
    {!busy && !error && ports === null && <p className="muted">点击“刷新端口”查询此实例的监听端口。</p>}
    {ports?.length === 0 && <p className="muted">没有发现监听端口。</p>}
    {!!ports?.length && <div className="table"><table><thead><tr>{["协议", "地址", "端口", "进程", "PID", "操作"].map(t => <th key={t}>{t}</th>)}</tr></thead>
      <tbody>{ports.map((port, i) => <tr key={`${port.raw}:${i}`}><td>{port.protocol}</td><td>{port.localAddress}</td><td>{port.port}</td><td>{port.processName ?? "—"}</td><td>{port.pid ?? "—"}</td><td><div className="actions">
        <button onClick={() => void action(() => navigator.clipboard.writeText(`localhost:${port.port}`), "地址已复制。")}>复制地址</button>
        {port.protocol === "tcp" && httpPorts.includes(port.port) && <button onClick={() => void action(() => openUrl(`http://localhost:${port.port}`), "已请求浏览器打开本地地址。")}>浏览器打开</button>}
      </div></td></tr>)}</tbody></table></div>}
  </section>;
}
