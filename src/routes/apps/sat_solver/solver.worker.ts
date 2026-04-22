import init, { solveSatJson } from "sat_solver";
import wasmUrl from "sat_solver/sat_solver_bg.wasm?url";

let ready: Promise<unknown> | null = null;

self.onmessage = async (ev: MessageEvent<{ id: number; expr: string }>) => {
  ready ??= init({ module_or_path: wasmUrl });
  await ready;
  const json = solveSatJson(ev.data.expr);
  (self as unknown as Worker).postMessage({ id: ev.data.id, json });
};
