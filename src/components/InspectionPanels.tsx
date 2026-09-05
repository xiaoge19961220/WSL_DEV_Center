import { useState } from "react";
import type { WslDistro } from "../lib/types";
import { DockerPanel } from "./DockerPanel";
export function Docker({ distros }: { distros: WslDistro[]; error?: (v: string) => void }) {
  const [name, setName] = useState("");
  const running = distros.filter(d => d.state === "Running");
  const selected = running.some(d => d.name === name) ? name : "";
  return <><section className="panel"><h2>按实例查看 Docker</h2><p className="muted">仅查询当前选择的实例。</p>
    <label>WSL 实例 <select value={selected} onChange={e => setName(e.target.value)}><option value="">请选择运行中的实例</option>{running.map(d => <option key={d.name} value={d.name}>{d.name}</option>)}</select></label>
    {!running.length && <p>没有运行中的实例，请先在实例页面启动。</p>}
  </section>{selected && <DockerPanel key={selected} name={selected} />}</>;
}
