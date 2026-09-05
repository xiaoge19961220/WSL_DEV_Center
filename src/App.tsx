import { useCallback, useEffect, useRef, useState } from "react";
import * as api from "./lib/api";
import type { WslDistro } from "./lib/types";
import { ErrorBox, Loading } from "./components/Feedback";
import { Machines } from "./components/Machines";
import { ConfirmDialog } from "./components/ConfirmDialog";
import { Detail } from "./components/Detail";
import { Docker } from "./components/InspectionPanels";
import { Settings } from "./components/SettingsPage";
import { readSettings } from "./lib/storage";
import "./App.css";

const menu = { "/": "概览", "/machines": "实例", "/docker": "Docker", "/settings": "设置" };
function currentRoute() { return window.location.hash.slice(1) || "/"; }

export default function App() {
  const [settings, setSettings] = useState(readSettings);
  const [route, setRoute] = useState(currentRoute);
  const [distros, setDistros] = useState<WslDistro[]>([]);
  const [loading, setLoading] = useState(true), [listError, setListError] = useState("");
  const [error, setError] = useState(""), [confirm, setConfirm] = useState(false);
  const fetching = useRef<Promise<void> | null>(null);
  const refresh = useCallback(async function refreshList(force = false): Promise<void> {
    if (fetching.current) {
      await fetching.current;
      if (force) return refreshList();
      return;
    }
    setLoading(true);
    const request = (async () => {
      try { setDistros(await api.listWslDistros()); setListError(""); }
      catch (e) { setListError(String(e)); }
      finally { setLoading(false); fetching.current = null; }
    })();
    fetching.current = request;
    return request;
  }, []);
  useEffect(() => {
    const changed = () => { setRoute(currentRoute()); setError(""); };
    window.addEventListener("hashchange", changed);
    return () => window.removeEventListener("hashchange", changed);
  }, []);
  useEffect(() => { void refresh(); }, [refresh, route]);
  useEffect(() => {
    if (route !== "/machines") return;
    const seconds = settings.refresh;
    if (![3, 5, 10].includes(seconds)) return;
    const timer = window.setInterval(() => { if (!document.hidden) void refresh(); }, seconds * 1000);
    return () => clearInterval(timer);
  }, [route, refresh, settings.refresh]);
  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = () => { document.documentElement.dataset.theme = settings.theme === "system" ? (media.matches ? "dark" : "light") : settings.theme; };
    apply(); media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [settings.theme]);
  let name = "";
  if (route.startsWith("/machines/")) {
    try { name = decodeURIComponent(route.slice("/machines/".length)); } catch { name = ""; }
  }
  const selected = distros.find(d => d.name === name);
  const title = name || menu[route as keyof typeof menu] || "页面不存在";
  return <div className="shell">
    <aside><div className="brand"><b>WSL</b> 开发中心</div><nav aria-label="主导航">
      {Object.entries(menu).map(([path, label]) => <a key={path} href={`#${path}`} aria-current={route === path || (name && path === "/machines") ? "page" : undefined}>{label}</a>)}
    </nav><p className="sidebar-note">纯本地 · 无账号 · 无云同步</p></aside>
    <main><header><div><span className="eyebrow">本地开发环境</span><h1>{title}</h1></div>
      {route === "/machines" && <button className="danger" onClick={() => setConfirm(true)}>关闭所有 WSL 实例</button>}
    </header><ErrorBox message={error} />
    {route !== "/settings" && <><ErrorBox message={listError} />{loading && <Loading />}{listError && <button disabled={loading} onClick={() => void refresh()}>重试读取实例</button>}</>}
    {route === "/" && <>
      <section className="hero"><span className="eyebrow">WSL 开发环境</span><h2>运行状态，一目了然。<br />开发环境，尽在本地。</h2><p className="muted">在本机管理 WSL 实例、端口和 Docker 容器。</p></section>
      {!loading && !listError && <section className="metrics">{[
        ["运行中", distros.filter(d => d.state === "Running").length],
        ["已停止", distros.filter(d => d.state === "Stopped").length],
        ["实例总数", distros.length], ["Docker", "按需查看"],
      ].map(([label, value]) => <article className="metric" key={label}><span>{label}</span><strong>{value}</strong></article>)}</section>}
    </>}
    {route === "/machines" && <section className="panel"><div className="toolbar"><h2>WSL 实例</h2><button disabled={loading} onClick={() => void refresh()}>{loading ? "正在刷新…" : "刷新列表"}</button></div>
      {!listError && (!loading || distros.length > 0) && <Machines distros={settings.showStopped ? distros : distros.filter(d => d.state !== "Stopped")} refresh={() => refresh(true)} select={d => { window.location.hash = `/machines/${encodeURIComponent(d.name)}`; }} error={setError} />}
    </section>}
    {route === "/docker" && !listError && (settings.docker ? <Docker distros={distros} /> : <p>Docker 面板已关闭，可在设置中启用。</p>)}
    {route === "/settings" && <Settings value={settings} change={setSettings} />}
    {name && !loading && !listError && (selected ? <Detail key={`${selected.name}:${selected.state}`} distro={selected} settings={settings} /> : <p>实例不存在，请返回实例列表刷新。</p>)}
    {!name && !(route in menu) && <p>页面不存在。<a href="#/">返回概览</a></p>}
    {confirm && <ConfirmDialog cancel={() => setConfirm(false)} confirm={async () => { await api.shutdownWsl(); setConfirm(false); await refresh(true); }} />}
    </main>
  </div>;
}
