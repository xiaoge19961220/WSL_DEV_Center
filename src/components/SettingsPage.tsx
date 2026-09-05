import { useState } from "react";
import { saveSettings, type Settings as Preferences } from "../lib/storage";
import { ErrorBox } from "./Feedback";

export function Settings({ value, change }: { value: Preferences; change: (value: Preferences) => void }) {
  const [error, setError] = useState("");
  const update = (patch: Partial<Preferences>) => {
    try { const next = { ...value, ...patch }; saveSettings(next); change(next); setError(""); }
    catch (e) { setError(`无法保存本地设置，本次更改未生效。${String(e)}`); }
  };
  return <section className="panel"><h2>本地设置</h2><ErrorBox message={error} /><div className="settings-grid">
    <label>实例列表刷新间隔<select value={value.refresh} onChange={e => update({ refresh: Number(e.target.value) })}>
      <option value={0}>手动</option><option value={3}>3 秒</option><option value={5}>5 秒</option><option value={10}>10 秒</option>
    </select></label>
    <label>默认终端<select value={value.terminal} onChange={e => update({ terminal: e.target.value as Preferences["terminal"] })}>
      <option value="windows">Windows Terminal</option><option value="powershell">PowerShell</option>
    </select></label>
    <label>外观<select value={value.theme} onChange={e => update({ theme: e.target.value as Preferences["theme"] })}>
      <option value="system">跟随系统</option><option value="light">浅色</option><option value="dark">深色</option>
    </select></label>
    <label className="check"><input type="checkbox" checked={value.showStopped} onChange={e => update({ showStopped: e.target.checked })} />显示已停止的实例</label>
    <label className="check"><input type="checkbox" checked={value.ports} onChange={e => update({ ports: e.target.checked })} />启用端口面板</label>
    <label className="check"><input type="checkbox" checked={value.docker} onChange={e => update({ docker: e.target.checked })} />启用 Docker 面板</label>
  </div><p className="muted">自动刷新仅作用于实例列表，最短为 3 秒。端口手动查询，Docker 仅查询选中的实例。</p><p className="muted">更改后立即生效，设置仅保存在本机。</p></section>;
}
