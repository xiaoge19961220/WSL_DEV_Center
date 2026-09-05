import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";
import { Detail } from "../src/components/Detail";
import { Docker } from "../src/components/InspectionPanels";
import { defaults, readSettings } from "../src/lib/storage";
import * as api from "../src/lib/api";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn(), isTauri: () => true }));
const mockInvoke = vi.mocked(invoke);
const stopped = { name: "Ubuntu 中文", state: "Stopped" as const, version: 2, isDefault: true };
const running = { ...stopped, state: "Running" as const };
const listCalls = () => mockInvoke.mock.calls.filter(([command]) => command === "list_wsl_distros").length;
const settle = () => act(async () => { await Promise.resolve(); await Promise.resolve(); });

beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear();
  window.history.replaceState(null, "", "/#/machines");
  Object.defineProperty(window, "matchMedia", { writable: true, value: vi.fn(() => ({ matches: false, addEventListener: vi.fn(), removeEventListener: vi.fn() })) });
  HTMLDialogElement.prototype.showModal = function () { this.setAttribute("open", ""); };
  HTMLDialogElement.prototype.close = function () { this.removeAttribute("open"); };
  mockInvoke.mockImplementation(async command => command === "list_wsl_distros" ? [stopped] : { success: true, stdout: "", stderr: "", code: 0 });
});
afterEach(() => { cleanup(); vi.useRealTimers(); });

describe("WSL 操作与刷新", () => {
  it("命令非零退出不会被当作成功", async () => {
    mockInvoke.mockResolvedValueOnce({ success: false, code: 5, stdout: "", stderr: "access denied" });
    await expect(api.startDistro("Ubuntu 中文")).rejects.toThrow("access denied");
  });
  it("列表失败显示错误而不是空状态", async () => {
    mockInvoke.mockRejectedValueOnce("读取失败");
    render(<App />); await settle();
    expect(screen.getByRole("alert").textContent).toContain("读取失败");
    expect(screen.queryByText(/没有可显示的 WSL 实例/)).toBeNull();
  });
  it("默认五秒刷新，卸载后取消计时器", async () => {
    vi.useFakeTimers(); const view = render(<App />); await settle();
    expect(listCalls()).toBe(1);
    await act(async () => { await vi.advanceTimersByTimeAsync(5000); });
    expect(listCalls()).toBe(2);
    view.unmount();
    await act(async () => { await vi.advanceTimersByTimeAsync(10000); });
    expect(listCalls()).toBe(2);
  });
  it("手动模式不轮询", async () => {
    vi.useFakeTimers(); localStorage.setItem("wsl-dev-center.settings", JSON.stringify({ ...defaults, refresh: 0 }));
    render(<App />); await settle();
    await act(async () => { await vi.advanceTimersByTimeAsync(15000); });
    expect(listCalls()).toBe(1);
  });
  it("尚未完成的列表请求不会重叠", async () => {
    vi.useFakeTimers(); mockInvoke.mockImplementation(() => new Promise(() => {}));
    render(<App />);
    await act(async () => { await vi.advanceTimersByTimeAsync(15000); });
    expect(listCalls()).toBe(1);
  });
  it("关闭确认默认焦点在取消，取消不发送关闭命令", async () => {
    render(<App />); await settle();
    fireEvent.click(screen.getByRole("button", { name: "关闭所有 WSL 实例" }));
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "取消" }));
    fireEvent.click(screen.getByRole("button", { name: "取消" }));
    expect(mockInvoke.mock.calls.some(([command]) => command === "shutdown_wsl")).toBe(false);
  });
  it("操作完成后重新查询，不复用操作前尚未完成的列表", async () => {
    let finishOld!: (value: unknown) => void;
    let queries = 0;
    mockInvoke.mockImplementation(command => {
      if (command !== "list_wsl_distros") return Promise.resolve({ success: true, code: 0, stdout: "", stderr: "" });
      queries++;
      if (queries === 2) return new Promise(resolve => { finishOld = resolve; });
      return Promise.resolve([queries === 1 ? stopped : running]);
    });
    render(<App />); await settle();
    fireEvent.click(screen.getByRole("button", { name: "刷新列表" }));
    fireEvent.click(screen.getByRole("button", { name: "启动", exact: true })); await settle();
    await act(async () => { finishOld([stopped]); }); await settle();
    expect(queries).toBe(3);
    expect(screen.getByRole("button", { name: "停止", exact: true })).toBeTruthy();
  });
});

describe("按需查询与设置", () => {
  it("停止的实例不会查询资源、端口或 Docker", async () => {
    render(<Detail distro={stopped} settings={defaults} />); await settle();
    expect(mockInvoke).not.toHaveBeenCalled();
    expect(screen.getByText(/此实例未运行/)).toBeTruthy();
  });
  it("禁用的面板不会执行查询，并展示资源部分错误", async () => {
    mockInvoke.mockResolvedValue({ distro: running.name, processCount: 0, errors: ["磁盘查询失败"] });
    render(<Detail distro={running} settings={{ ...defaults, ports: false, docker: false }} />); await settle();
    expect(mockInvoke.mock.calls.map(([command]) => command)).toEqual(["get_distro_resource_info"]);
    expect(screen.getByRole("alert").textContent).toContain("磁盘查询失败");
    expect(screen.getByText("0")).toBeTruthy();
  });
  it("切换实例后忽略旧 Docker 请求", async () => {
    let first!: (value: unknown) => void;
    mockInvoke.mockImplementation((_command, args) => (args as { name: string }).name === "A" ? new Promise(resolve => { first = resolve; }) : Promise.resolve([]));
    render(<Docker distros={[{ ...running, name: "A" }, { ...running, name: "B" }]} />);
    fireEvent.change(screen.getByRole("combobox"), { target: { value: "A" } });
    fireEvent.change(screen.getByRole("combobox"), { target: { value: "B" } }); await settle();
    await act(async () => { first([{ id: "abc", names: "旧容器", image: "redis", status: "Up", ports: "" }]); });
    expect(screen.queryByText("旧容器")).toBeNull();
    expect(screen.getByText(/暂无容器/)).toBeTruthy();
  });
  it("非法或损坏设置回退到安全的刷新间隔", () => {
    localStorage.setItem("wsl-dev-center.settings", JSON.stringify({ refresh: 1 }));
    expect(readSettings().refresh).toBe(5);
    localStorage.setItem("wsl-dev-center.settings", "{");
    expect(readSettings()).toEqual(defaults);
  });
});
