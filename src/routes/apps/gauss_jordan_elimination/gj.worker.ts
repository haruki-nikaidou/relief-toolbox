import init, { solveGaussJordanJson } from 'g_j_elimination';
import wasmUrl from 'g_j_elimination/g_j_elimination_bg.wasm?url';

let ready: Promise<unknown> | null = null;

self.onmessage = async (ev: MessageEvent<{ id: number; input: string }>) => {
	ready ??= init({ module_or_path: wasmUrl });
	await ready;
	const json = solveGaussJordanJson(ev.data.input);
	(self as unknown as Worker).postMessage({ id: ev.data.id, json });
};
