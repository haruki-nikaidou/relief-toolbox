<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { Button } from "$lib/components/ui/button/index.js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "$lib/components/ui/card/index.js";
import { Textarea } from "$lib/components/ui/textarea/index.js";
import { Loader2 } from "@lucide/svelte";
import { SolverClient, type SolveResult } from "./solver-client.js";

let expr = $state("");
let result = $state<SolveResult | null>(null);
let solving = $state(false);
let copied = $state(false);

let client: SolverClient | null = null;
let copyTimeoutId: ReturnType<typeof setTimeout> | undefined;

const variableCount = $derived(
  new Set(expr.toLowerCase().match(/[a-z]/g) ?? []).size,
);

onMount(() => {
  if (typeof window !== "undefined") {
    client = new SolverClient();
  }
});

onDestroy(() => {
  client?.terminate();
  if (copyTimeoutId) {
    clearTimeout(copyTimeoutId);
  }
});

async function solve() {
  if (!expr.trim() || solving) return;
  solving = true;
  result = null;
  if (!client) {
    solving = false;
    return;
  }
  result = await client.solve(expr);
  solving = false;
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    solve();
  }
}

async function copyMinterms() {
  if (result?.kind !== "minterms") return;
  await navigator.clipboard.writeText(result.values.join("\n"));
  copied = true;
  if (copyTimeoutId) {
    clearTimeout(copyTimeoutId);
  }
  copyTimeoutId = setTimeout(() => {
    copied = false;
  }, 700);
}

const examples = ["A & !B", "(A|B) & (!A|C)", "A^B^C", "!(A&B)|C"];
</script>

<section class="mx-auto max-w-2xl py-4">
  <Card>
    <CardHeader>
      <CardTitle>SAT Solver</CardTitle>
      <CardDescription>
        Check satisfiability of boolean expressions. Finds all minterms for ≤10
        variables.
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-6">
      <div class="rounded-lg bg-muted p-4 text-sm space-y-1">
        <p class="font-medium">Syntax</p>
        <div class="flex flex-col gap-y-2">
          <span><code>a–z / A–Z</code>: variables (case-insensitive)</span>
          <span><code>!</code>: NOT</span>
          <span><code>&amp;</code>: AND</span>
          <span><code>|</code>: OR</span>
          <span><code>^</code>: XOR</span>
          <span><code>(...)</code>: grouping</span>
        </div>
      </div>

      <div class="flex flex-wrap gap-2">
        <span class="text-sm font-medium self-center">Examples:</span>
        {#each examples as ex (ex)}
          <Button
            type="button"
            variant="outline"
            size="sm"
            onclick={() => { expr = ex; }}
          >
            {ex}
          </Button>
        {/each}
      </div>

      <div class="space-y-2">
        <Textarea
          placeholder="Enter a boolean expression to solve"
          rows={3}
          class="font-mono"
          bind:value={expr}
          onkeydown={handleKeydown}
        />
        <div class="flex items-center justify-between">
          <span class="bg-secondary text-secondary-foreground px-2 py-0.5 text-xs rounded-full">
            Variables: {variableCount}
          </span>
          <Button
            type="button"
            variant="default"
            disabled={solving || !expr.trim()}
            onclick={solve}
          >
            {#if solving}
              <Loader2 class="animate-spin size-4" />
              Solving...
            {:else}
              Solve
            {/if}
          </Button>
        </div>
      </div>

      {#if result !== null}
        <div class="space-y-3">
          {#if result.kind === "error"}
            <p class="text-sm text-destructive">{result.error}</p>
          {:else if result.kind === "boolean"}
            {#if result.value}
              <span class="bg-green-500 text-white px-3 py-1 rounded-full text-sm font-semibold">
                SATISFIABLE
              </span>
              <p class="text-sm text-muted-foreground">
                Expression is satisfiable (&gt;10 variables, minterms not listed)
              </p>
            {:else}
              <span class="bg-red-500 text-white px-3 py-1 rounded-full text-sm font-semibold">
                UNSATISFIABLE
              </span>
            {/if}
          {:else if result.kind === "minterms"}
            <div class="flex items-center gap-3 flex-wrap">
              <span class="bg-green-500 text-white px-3 py-1 rounded-full text-sm font-semibold">
                SATISFIABLE
              </span>
              <span class="text-sm text-muted-foreground">
                {result.values.length} minterm(s)
              </span>
            </div>
            <div class="max-h-48 overflow-y-auto bg-muted rounded p-2">
              {#each result.values as minterm (minterm)}
                <div class="text-xs font-mono">{minterm}</div>
              {/each}
            </div>
            <Button
              type="button"
              variant="outline"
              size="sm"
              onclick={copyMinterms}
            >
              {copied ? "Copied!" : "Copy minterms"}
            </Button>
          {/if}
        </div>
      {/if}
    </CardContent>
  </Card>
</section>
