<script lang="ts">
  import * as Select from "$lib/components/ui/select/index.js";
  import {onMount, onDestroy, untrack} from "svelte";
  import {Button} from "$lib/components/ui/button/index.js";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card/index.js";
  import {Loader2} from "@lucide/svelte";
  import {
    GJClient,
    type GJResult,
    type StepData,
    type RatCell,
  } from "./gj-client.js";

  function gcd(a: number, b: number): number {
    a = Math.abs(Math.round(a));
    b = Math.abs(Math.round(b));
    while (b) {
      [a, b] = [b, a % b];
    }
    return a || 1;
  }

  function parseRational(s: string): [number, number] | null {
    s = s.trim();
    if (!s) return null;
    const frac = s.match(/^(-?\d+)\s*\/\s*(-?\d+)$/);
    if (frac) {
      const p = parseInt(frac[1], 10);
      const q = parseInt(frac[2], 10);
      if (q === 0) return null;
      const g = gcd(Math.abs(p), Math.abs(q));
      const sign = q < 0 ? -1 : 1;
      return [(sign * p) / g, (sign * q) / g];
    }
    if (!/^-?\d+$/.test(s)) return null;
    return [parseInt(s, 10), 1];
  }

  function formatRat(p: number, q: number): string {
    if (q === 1) return `${p}`;
    if (q === -1) return `${-p}`;
    return `${p}/${q}`;
  }

  function stepDescription(step: StepData): string {
    switch (step.op) {
      case "start":
        return "Initial matrix";
      case "row_reorder":
        return `R${step.a + 1} ↔ R${step.b + 1}`;
      case "row_mul":
        return `R${step.target + 1} ×= ${formatRat(step.factor.p, step.factor.q)}`;
      case "row_add": {
        const f = formatRat(step.factor.p, step.factor.q);
        return `R${step.to_row + 1} += (${f}) × R${step.from_row + 1}`;
      }
    }
  }

  let rowString = $state("3");
  let rows = $derived.by(() => {
    const n = parseInt(rowString, 10);
    return Number.isNaN(n) || n < 2 || n > 10 ? 3 : n;
  });
  let colString = $state("4");
  let cols = $derived.by(() => {
    const n = parseInt(colString, 10);
    return Number.isNaN(n) || n < 2 || n > 10 ? 4 : n;
  });
  let untrackedCells = Array.from({length: 3}, () => Array.from({length: 4}, () => "0"));
  let cells = $state<string[][]>(
    newCells(untrackedCells, () => rows, () => cols)
  );
  let computing = $state(false);
  let result = $state<GJResult | null>(null);
  let parseError = $state<string | null>(null);
  let client: GJClient | null = null;

  function newCells(cells: string[][], getRow: () => number, getCol: () => number): string[][] {
    return Array.from({length: getRow()}, (_, i) =>
      Array.from({length: getCol()}, (_, j) => cells[i]?.[j] ?? "0"),
    );
  }

  $effect(
    () => {
      untrackedCells = cells;
    }
  )

  async function compute() {
    if (computing || !client) return;
    parseError = null;
    const parsedRows: [number, number][][] = [];
    for (let i = 0; i < rows; i++) {
      const row: [number, number][] = [];
      for (let j = 0; j < cols; j++) {
        const r = parseRational(cells[i][j]);
        if (r === null) {
          parseError = `Invalid value at row ${i + 1}, column ${j + 1}: "${cells[i][j]}"`;
          return;
        }
        row.push(r);
      }
      parsedRows.push(row);
    }
    computing = true;
    result = null;
    result = await client.solve({rows: parsedRows});
    computing = false;
  }

  onMount(() => {
    if (typeof window !== "undefined") {
      client = new GJClient();
    }
  });

  onDestroy(() => {
    client?.terminate();
  });
</script>

{#snippet matrixDisplay(matrix: RatCell[][])}
  <div class="overflow-x-auto">
    <div class="relative inline-flex items-stretch gap-0 text-sm font-mono">
      <div
          class="border-foreground w-2 rounded-l-sm border-b-2 border-l-2 border-t-2"
      ></div>
      <div
          class="grid gap-x-3 gap-y-0.5 px-2 py-1"
          style={`grid-template-columns: repeat(${matrix[0]?.length ?? 1}, auto)`}
      >
        {#each matrix as row, rowIdx (rowIdx)}
          {#each row as cell, colIdx (colIdx)}
            <span class="min-w-10 text-center">
              {formatRat(cell.p, cell.q)}
            </span>
          {/each}
        {/each}
      </div>
      <div
          class="border-foreground w-2 rounded-r-sm border-b-2 border-r-2 border-t-2"
      ></div>
    </div>
  </div>
{/snippet}

<section class="mx-auto max-w-4xl py-4">
  <Card>
    <CardHeader>
      <CardTitle>Gauss–Jordan Elimination</CardTitle>
      <CardDescription>
        Reduces a matrix to reduced row echelon form (RREF). Supports
        2×2 to 10×10 matrices. Enter integers or fractions (e.g. 1/2,
        -3/4).
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-6">
      <!-- Controls -->
      <div class="flex flex-wrap items-center gap-4">
        <label class="flex items-center gap-2 text-sm font-medium">
          Rows:
          <Select.Root type="single" bind:value={rowString}>
            <Select.Trigger class="w-20">
              {rowString}
            </Select.Trigger>
            <Select.Content>
              {#each Array.from({length: 9}, (_, i) => i + 2) as n (n)}
                <Select.Item value={n.toString()}>
                  {n}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </label>
        <label class="flex items-center gap-2 text-sm font-medium">
          Cols:
          <Select.Root type="single" bind:value={colString}>
            <Select.Trigger class="w-20">
              {colString}
            </Select.Trigger>
            <Select.Content>
              {#each Array.from({length: 9}, (_, i) => i + 2) as n (n)}
                <Select.Item value={n.toString()}>
                  {n}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </label>
        <Button onclick={compute} disabled={computing}>
          {#if computing}
            <Loader2 class="mr-2 size-4 animate-spin"/>
            Computing…
          {:else}
            Compute
          {/if}
        </Button>
      </div>

      <!-- Matrix input -->
      <div class="overflow-x-auto">
        <div class="relative inline-flex items-stretch gap-0">
          <div
              class="border-foreground w-3 rounded-l-sm border-b-2 border-l-2 border-t-2"
          ></div>
          <div
              class="grid gap-1 p-2"
              style={`grid-template-columns: repeat(${cols}, minmax(4rem, 1fr))`}
          >
            {#each Array.from({length: rows}, (_, i) => i) as i (i)}
              {#each Array.from({length: cols}, (_, j) => j) as j (j)}
                <input
                    type="text"
                    class="border-border bg-background focus:ring-ring w-16 rounded border px-1 py-0.5 text-center font-mono text-sm focus:outline-none focus:ring-1"
                    bind:value={cells[i][j]}
                />
              {/each}
            {/each}
          </div>
          <div
              class="border-foreground w-3 rounded-r-sm border-b-2 border-r-2 border-t-2"
          ></div>
        </div>
      </div>

      {#if parseError}
        <p class="text-destructive text-sm">{parseError}</p>
      {/if}

      <!-- Result -->
      {#if result !== null}
        <div class="space-y-4">
          {#if result.kind === "error"}
            <p class="text-destructive text-sm">{result.error}</p>
          {:else if result.kind === "result"}
            <p class="text-muted-foreground text-sm">
              More than 100 steps — showing final RREF only.
            </p>
            {@render matrixDisplay(result.matrix)}
          {:else if result.kind === "steps"}
            <div class="flex items-center gap-2">
              <span
                  class="bg-secondary text-secondary-foreground rounded-full px-2.5 py-0.5 text-xs font-semibold"
              >
                {result.steps.length} step{result.steps.length === 1 ? "" : "s"}
              </span>
            </div>
            <div class="space-y-2">
              {#each result.steps as step, idx (idx)}
                <div
                    class="border-border rounded-lg border p-3 space-y-2"
                >
                  <div class="flex items-center gap-2">
                    <span class="text-muted-foreground text-xs">Step {idx + 1}</span>
                    <span class="text-sm font-medium">{stepDescription(step)}</span>
                  </div>
                  {@render matrixDisplay(step.matrix)}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/if}
    </CardContent>
  </Card>
</section>
