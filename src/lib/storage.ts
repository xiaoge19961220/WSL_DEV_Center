export interface Settings {
  refresh: number;
  ports: boolean;
  docker: boolean;
  showStopped: boolean;
  terminal: "windows" | "powershell";
  theme: "system" | "light" | "dark";
}
export const defaults: Settings = { refresh: 5, ports: true, docker: true, showStopped: true, terminal: "windows", theme: "system" };
export function readSettings(): Settings {
  try {
    const value = JSON.parse(localStorage.getItem("wsl-dev-center.settings") ?? "null") ?? {};
    const refresh = Number(value.refresh ?? localStorage.getItem("refresh") ?? 5);
    return {
      refresh: [0, 3, 5, 10].includes(refresh) ? refresh : 5,
      ports: typeof value.ports === "boolean" ? value.ports : true,
      docker: typeof value.docker === "boolean" ? value.docker : true,
      showStopped: typeof value.showStopped === "boolean" ? value.showStopped : true,
      terminal: value.terminal === "powershell" ? "powershell" : "windows",
      theme: value.theme === "light" || value.theme === "dark" ? value.theme : "system",
    };
  } catch { return defaults; }
}
export function saveSettings(settings: Settings) {
  localStorage.setItem("wsl-dev-center.settings", JSON.stringify(settings));
}
