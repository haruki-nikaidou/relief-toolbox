import init, { solveSatJson } from 'sat_solver';

let ready: Promise<unknown> | null = null;

self.onmessage = async (ev: MessageEvent<{ id: number; expr: string }>) => {
	ready ??= init();
	await ready;
	const json = solveSatJson(ev.data.expr);
	(self as unknown as Worker).postMessage({ id: ev.data.id, json });
};
