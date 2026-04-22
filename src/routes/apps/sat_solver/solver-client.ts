export type SolveResult =
  | { kind: "minterms"; values: string[] }
  | { kind: "boolean"; value: boolean }
  | { kind: "error"; error: string };

type Resolver = (result: SolveResult) => void;

export class SolverClient {
  private worker: Worker;
  private pending = new Map<number, Resolver>();
  private nextId = 0;

  constructor() {
    this.worker = new Worker(new URL("./solver.worker.ts", import.meta.url), {
      type: "module",
    });
    this.worker.onmessage = (
      ev: MessageEvent<{ id: number; json: string }>,
    ) => {
      const resolver = this.pending.get(ev.data.id);
      if (resolver) {
        this.pending.delete(ev.data.id);
        resolver(JSON.parse(ev.data.json) as SolveResult);
      }
    };
  }

  solve(expr: string): Promise<SolveResult> {
    return new Promise((resolve) => {
      const id = this.nextId++;
      this.pending.set(id, resolve);
      this.worker.postMessage({ id, expr });
    });
  }

  terminate() {
    this.worker.terminate();
  }
}
