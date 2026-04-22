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
import { Badge } from "$lib/components/ui/badge/index.js";
import { cn } from "$lib/utils.js";

type ScanMode = "camera" | "file";

let mode = $state<ScanMode>("camera");
let cameraActive = $state(false);
let scanResult = $state("");
let error = $state("");
let copied = $state(false);
let scanning = $state(false);

let videoEl = $state<HTMLVideoElement | null>(null);
let fileImageSrc = $state<string | null>(null);
let fileInputEl = $state<HTMLInputElement | null>(null);

// biome-ignore lint/suspicious/noExplicitAny: qr-scanner lacks full TS types
let QrScanner: any = null;
// biome-ignore lint/suspicious/noExplicitAny: qr-scanner instance
let scanner: any = null;

onMount(async () => {
  const mod = await import("qr-scanner");
  QrScanner = mod.default;
});

onDestroy(() => {
  destroyScanner();
});

function destroyScanner() {
  if (scanner) {
    scanner.destroy();
    scanner = null;
  }
  cameraActive = false;
}

async function startCamera() {
  if (!videoEl || !QrScanner) return;
  error = "";
  try {
    scanner = new QrScanner(
      videoEl,
      (result: { data: string }) => {
        scanResult = result.data;
      },
      {
        returnDetailedScanResult: true,
        highlightScanRegion: true,
        highlightCodeOutline: true,
      },
    );
    await scanner.start();
    cameraActive = true;
  } catch (e) {
    error =
      e instanceof Error
        ? e.message.includes("permission")
          ? "Camera permission denied. Please allow camera access and try again."
          : e.message
        : "Failed to start camera.";
    destroyScanner();
  }
}

function stopCamera() {
  destroyScanner();
}

function switchMode(newMode: ScanMode) {
  if (newMode === mode) return;
  destroyScanner();
  scanResult = "";
  error = "";
  fileImageSrc = null;
  if (fileInputEl) fileInputEl.value = "";
  mode = newMode;
}

async function handleFileChange(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  if (!file || !QrScanner) return;

  scanResult = "";
  error = "";
  scanning = true;

  fileImageSrc = URL.createObjectURL(file);

  try {
    const result = await QrScanner.scanImage(file, {
      returnDetailedScanResult: true,
    });
    scanResult = result.data;
  } catch (e) {
    error =
      e instanceof Error
        ? e.message.includes("No QR code found")
          ? "No QR code found in this image."
          : e.message
        : "Failed to scan image.";
  } finally {
    scanning = false;
  }
}

async function copyToClipboard() {
  if (!scanResult) return;
  try {
    await navigator.clipboard.writeText(scanResult);
    copied = true;
    setTimeout(() => (copied = false), 2000);
  } catch {
    error = "Failed to copy to clipboard.";
  }
}

function openLink() {
  if (!scanResult) return;
  try {
    const url = new URL(scanResult);
    window.open(url.toString(), "_blank", "noopener,noreferrer");
  } catch {
    // not a URL, ignore
  }
}

let isUrl = $derived.by(() => {
  if (!scanResult) return false;
  try {
    new URL(scanResult);
    return true;
  } catch {
    return false;
  }
});
</script>

<section class="mx-auto max-w-2xl py-4">
  <Card>
    <CardHeader>
      <CardTitle>QR Code Scanner</CardTitle>
      <CardDescription>
        Scan QR codes using your camera or by uploading an image file.
      </CardDescription>
    </CardHeader>
    <CardContent class="space-y-6">

      <!-- Mode toggle -->
      <div class="grid grid-cols-2 gap-2">
        <Button
          type="button"
          variant="outline"
          onclick={() => switchMode("camera")}
          class={cn(mode === "camera" && "bg-primary text-primary-foreground")}
        >
          Camera
        </Button>
        <Button
          type="button"
          variant="outline"
          onclick={() => switchMode("file")}
          class={cn(mode === "file" && "bg-primary text-primary-foreground")}
        >
          Upload Image
        </Button>
      </div>

      <!-- Camera mode -->
      {#if mode === "camera"}
        <div class="space-y-3">
          <div
            class={cn(
              "relative overflow-hidden rounded-lg border bg-black",
              !cameraActive && "flex min-h-48 items-center justify-center",
            )}
          >
            <video
              bind:this={videoEl}
              playsinline
              class={cn(
                "w-full rounded-lg",
                !cameraActive && "hidden",
              )}
            ></video>
            {#if !cameraActive}
              <p class="text-sm text-muted-foreground">
                Camera preview will appear here
              </p>
            {/if}
          </div>

          <Button
            type="button"
            variant={cameraActive ? "destructive" : "default"}
            class="w-full"
            onclick={cameraActive ? stopCamera : startCamera}
          >
            {cameraActive ? "Stop Camera" : "Start Camera"}
          </Button>
        </div>
      {/if}

      <!-- File mode -->
      {#if mode === "file"}
        <div class="space-y-3">
          <label class="text-sm font-medium" for="qr-file-input">
            Select an image containing a QR code
          </label>
          <input
            id="qr-file-input"
            type="file"
            accept="image/*"
            bind:this={fileInputEl}
            onchange={handleFileChange}
            class="block w-full cursor-pointer rounded-lg border border-input bg-background px-3 py-2 text-sm text-foreground file:mr-3 file:cursor-pointer file:rounded file:border-0 file:bg-primary file:px-3 file:py-1 file:text-sm file:font-medium file:text-primary-foreground hover:file:bg-primary/90"
          />

          {#if fileImageSrc}
            <div class="overflow-hidden rounded-lg border">
              <img
                src={fileImageSrc}
                alt="Uploaded file for QR scanning"
                class="max-h-64 w-full object-contain"
              />
            </div>
          {/if}

          {#if scanning}
            <p class="text-sm text-muted-foreground">Scanning image…</p>
          {/if}
        </div>
      {/if}

      <!-- Error -->
      {#if error}
        <div class="rounded-lg border border-destructive/50 bg-destructive/10 px-4 py-3">
          <p class="text-sm text-destructive">{error}</p>
        </div>
      {/if}

      <!-- Result -->
      {#if scanResult}
        <div class="space-y-3">
          <div class="flex items-center gap-2">
            <p class="text-sm font-medium">Result</p>
            {#if isUrl}
              <Badge variant="secondary">URL</Badge>
            {/if}
          </div>
          <textarea
            readonly
            rows={4}
            value={scanResult}
            class="w-full resize-none rounded-lg border bg-muted px-3 py-2 font-mono text-sm text-foreground focus:outline-none"
          ></textarea>
          <div class="flex gap-2">
            <Button
              type="button"
              variant="outline"
              class="flex-1"
              onclick={copyToClipboard}
            >
              {copied ? "Copied!" : "Copy to Clipboard"}
            </Button>
            {#if isUrl}
              <Button
                type="button"
                variant="outline"
                class="flex-1"
                onclick={openLink}
              >
                Open Link
              </Button>
            {/if}
          </div>
        </div>
      {/if}

    </CardContent>
  </Card>
</section>
