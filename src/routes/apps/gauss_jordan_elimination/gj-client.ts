export type RatCell = { p: number; q: number };

export type StepData =
	| { op: 'start'; matrix: RatCell[][] }
	| { op: 'row_add'; from_row: number; to_row: number; factor: RatCell; matrix: RatCell[][] }
	| { op: 'row_mul'; target: number; factor: RatCell; matrix: RatCell[][] }
	| { op: 'row_reorder'; a: number; b: number; matrix: RatCell[][] };

export type GJResult =
	| { kind: 'steps'; steps: StepData[] }
	| { kind: 'result'; matrix: RatCell[][] }
	| { kind: 'error'; error: string };

export type GJInput = {
	rows: [number, number][][];
};

type Resolver = (result: GJResult) => void;

export class GJClient {
	private worker: Worker;
	private pending = new Map<number, Resolver>();
	private nextId = 0;

	constructor() {
		this.worker = new Worker(new URL('./gj.worker.ts', import.meta.url), { type: 'module' });
		this.worker.onmessage = (ev: MessageEvent<{ id: number; json: string }>) => {
			const resolver = this.pending.get(ev.data.id);
			if (resolver) {
				this.pending.delete(ev.data.id);
				resolver(JSON.parse(ev.data.json) as GJResult);
			}
		};
	}

	solve(input: GJInput): Promise<GJResult> {
		return new Promise((resolve) => {
			const id = this.nextId++;
			this.pending.set(id, resolve);
			this.worker.postMessage({ id, input: JSON.stringify(input) });
		});
	}

	terminate() {
		this.worker.terminate();
	}
}
