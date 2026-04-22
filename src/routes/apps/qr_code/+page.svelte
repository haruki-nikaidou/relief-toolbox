<script lang="ts">
import { onDestroy } from "svelte";
import QRCode from "qrcode";
import { Button } from "$lib/components/ui/button/index.js";
import { Textarea } from "$lib/components/ui/textarea/index.js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "$lib/components/ui/card/index.js";
import { cn } from "$lib/utils.js";

type ErrorCorrectionLevel = "L" | "M" | "Q" | "H";

const levels: ErrorCorrectionLevel[] = ["L", "M", "Q", "H"];
const levelLabels: Record<ErrorCorrectionLevel, string> = {
  L: "L — Low",
  M: "M — Medium",
  Q: "Q — Quartile",
  H: "H — High",
};

let text = $state("https://example.com");
let level = $state<ErrorCorrectionLevel>("M");
let svgOutput = $state("");
let error = $state("");

let debounceId: ReturnType<typeof setTimeout> | undefined;

async function generateSvg(input: string, ecl: ErrorCorrectionLevel) {
  if (!input.trim()) {
    svgOutput = "";
    error = "";
    return;
  }
  try {
    svgOutput = await QRCode.toString(input, {
      type: "svg",
      errorCorrectionLevel: ecl,
      margin: 2,
    });
    error = "";
  } catch (e) {
    svgOutput = "";
    error = e instanceof Error ? e.message : "Failed to generate QR code.";
  }
}

$effect(() => {
  const input = text;
  const ecl = level;
  if (debounceId) clearTimeout(debounceId);
  debounceId = setTimeout(() => {
    generateSvg(input, ecl);
  }, 150);
});

onDestroy(() => {
  if (debounceId) clearTimeout(debounceId);
});

async function downloadPng() {
  if (!text.trim()) return;
  const dataUrl = await QRCode.toDataURL(text, {
    errorCorrectionLevel: level,
    width: 512,
    margin: 2,
  });
  const a = document.createElement("a");
  a.href = dataUrl;
  a.download = "qrcode.png";
  a.click();
}
</script>

<section class="mx-auto max-w-2xl py-4">
    <Card>
        <CardHeader>
            <CardTitle>QR Code Generator</CardTitle>
            <CardDescription>
                Enter text or a URL to generate a QR code. Adjust error
                correction level as needed.
            </CardDescription>
        </CardHeader>
        <CardContent class="space-y-6">
            <div class="space-y-2">
                <label class="text-sm font-medium" for="qr-input">Text / URL</label>
                <Textarea
                    id="qr-input"
                    placeholder="https://example.com"
                    rows={3}
                    bind:value={text}
                    class="resize-none font-mono text-sm"
                />
            </div>

            <div class="space-y-2">
                <p class="text-sm font-medium">Error Correction Level</p>
                <div class="grid grid-cols-2 gap-2 sm:grid-cols-4">
                    {#each levels as l}
                        <Button
                            type="button"
                            variant="outline"
                            size="sm"
                            onclick={() => (level = l)}
                            class={cn(
                                "w-full",
                                level === l &&
                                    "bg-primary text-primary-foreground",
                            )}
                        >
                            {levelLabels[l]}
                        </Button>
                    {/each}
                </div>
            </div>

            {#if error}
                <p class="text-sm text-destructive">{error}</p>
            {:else if svgOutput}
                <div class="space-y-3">
                    <p class="text-sm font-medium">Preview</p>
                    <div
                        class="flex items-center justify-center rounded-lg border bg-white p-4"
                    >
                        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                        {@html svgOutput}
                    </div>
                    <Button
                        type="button"
                        variant="outline"
                        class="w-full"
                        onclick={downloadPng}
                    >
                        Download PNG
                    </Button>
                </div>
            {:else if !text.trim()}
                <p class="text-sm text-muted-foreground">
                    Enter some text above to generate a QR code.
                </p>
            {/if}
        </CardContent>
    </Card>
</section>
