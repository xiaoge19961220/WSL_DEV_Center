import type { Settings } from "../lib/storage";
import { useEffect, useState } from "react";
import * as api from "../lib/api";
import type { DistroResourceInfo, WslDistro } from "../lib/types";
import { DockerPanel } from "./DockerPanel";
import { PortPanel } from "./PortPanel";
import { ErrorBox, Loading, Status } from "./Feedback";

export function Detail({ distro, settings }: { distro: WslDistro; settings: Settings }) {
  const [resource, setResource] = useState<DistroResourceInfo>();
  const [busy, setBusy] = useState(false), [error, setError] = useState("");
  const [revision, setRevision] = useState(0);
  useEffect(() => {
    if (distro.state !== "Running") return;
    let active = true;
    setBusy(true); setError(""); setResource(undefined);
    api.resources(distro.name).then(value => { if (active) setResource(value); })
      .catch(e => { if (active) setError(String(e)); })
      .finally(() => { if (active) setBusy(false); });
    return () => { active = false; };
  }, [distro.name, distro.state, revision]);
  return <><a href="#/machines">← 返回实例列表</a><section className="panel">
    <div className="toolbar"><div><h2>{distro.name}</h2><Status state={distro.state} /> · WSL {distro.version ?? "—"}{distro.isDefault && " · 默认实例"}</div>
      <button disabled={busy || distro.state !== "Running"} onClick={() => setRevision(v => v + 1)}>刷新资源</button>
    </div>
    {distro.state !== "Running" ? <p className="muted">此实例未运行。请在实例列表中启动后查看资源、端口和 Docker。</p> : <>
      {busy && <Loading />}<ErrorBox message={error} />
      {resource && <><div className="resource">{[
        ["系统版本", resource.osVersionText], ["内核版本", resource.kernelVersionText], ["CPU 占用", resource.cpuText],
        ["内存", resource.memoryText], ["磁盘", resource.diskText], ["运行时间", resource.uptimeText], ["进程数", resource.processCount],
      ].map(([label, value]) => <article key={label}><span>{label}</span><strong>{value ?? "暂不可用"}</strong></article>)}</div>
      <ErrorBox message={resource.errors.join("\n\n")} /></>}
    </>}
  </section>{distro.state === "Running" && <>{settings.ports && <PortPanel key={distro.name} name={distro.name} />}{settings.docker && <DockerPanel key={`docker:${distro.name}`} name={distro.name} />}</>}</>;
}
