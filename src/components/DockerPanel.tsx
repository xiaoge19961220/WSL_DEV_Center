import { useEffect, useRef, useState } from "react";
import * as api from "../lib/api";
import type { DockerContainer } from "../lib/types";
import { ErrorBox, Loading } from "./Feedback";

export function DockerPanel({ name }: { name: string }) {
  const [items, setItems] = useState<DockerContainer[] | null>(null);
  const [revision, setRevision] = useState(0), [busy, setBusy] = useState(true);
  const [error, setError] = useState(""), [operation, setOperation] = useState("");
  const [logs, setLogs] = useState<{ name: string; text: string }>();
  const lock = useRef(false);
  const alive = useRef(true);
  useEffect(() => { alive.current = true; return () => { alive.current = false; }; }, []);
  useEffect(() => {
    let active = true;
    setBusy(true); setError(""); setItems(null);
    api.containers(name).then(value => { if (active) setItems(value); })
      .catch(e => { if (active) setError(String(e)); })
      .finally(() => { if (active) setBusy(false); });
    return () => { active = false; };
  }, [name, revision]);
  const run = async (container: DockerContainer, kind: "start" | "stop" | "logs") => {
    if (lock.current) return;
    lock.current = true; setOperation(container.id); setError("");
    try {
      if (kind === "logs") {
        const output = await api.containerLogs(name, container.id);
        if (alive.current) setLogs({ name: container.names, text: [output.stdout, output.stderr].filter(Boolean).join("\n") || "暂无日志。" });
      } else {
        await (kind === "start" ? api.startContainer(name, container.id) : api.stopContainer(name, container.id));
        if (alive.current) { setLogs(undefined); setRevision(v => v + 1); }
      }
    } catch (e) { if (alive.current) setError(String(e)); }
    finally { lock.current = false; if (alive.current) setOperation(""); }
  };
  return <section className="panel"><div className="toolbar"><h2>Docker 容器</h2><button disabled={busy || !!operation} onClick={() => { setLogs(undefined); setRevision(v => v + 1); }}>刷新容器</button></div>
    <ErrorBox message={error} />{busy && <Loading />}
    {items?.length === 0 && <p className="muted">Docker 可用，此实例中暂无容器。</p>}
    {!!items?.length && <div className="table"><table><thead><tr>{["名称", "镜像", "状态", "端口", "操作"].map(t => <th key={t}>{t}</th>)}</tr></thead>
      <tbody>{items.map(c => <tr key={c.id}><td>{c.names}</td><td>{c.image}</td><td>{c.status}</td><td>{c.ports || "—"}</td><td><div className="actions">
        <button disabled={busy || !!operation || c.status.startsWith("Up")} onClick={() => void run(c, "start")}>启动</button>
        <button disabled={busy || !!operation || !c.status.startsWith("Up")} onClick={() => void run(c, "stop")}>停止</button>
        <button disabled={busy || !!operation} onClick={() => void run(c, "logs")}>查看日志</button>
        {operation === c.id && <span role="status">正在操作…</span>}
      </div></td></tr>)}</tbody></table></div>}
    {logs && <section><div className="toolbar"><h3>{logs.name} · 最近 200 行日志</h3><button onClick={() => setLogs(undefined)}>关闭日志</button></div><pre className="logs">{logs.text}</pre></section>}
  </section>;
}
