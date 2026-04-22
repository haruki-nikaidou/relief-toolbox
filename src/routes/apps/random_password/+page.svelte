<script lang="ts">
import { onDestroy } from "svelte";
import { Button } from "$lib/components/ui/button/index.js";
import { Toggle } from "$lib/components/ui/toggle/index.js";
import { Slider } from "$lib/components/ui/slider/index.js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "$lib/components/ui/card/index.js";
import {
  generatePassword,
  type PasswordOptions,
  mapSliderToPasswordLength,
  PASSWORD_SLIDER_DEFAULT,
  PASSWORD_SLIDER_MAX,
  PASSWORD_SLIDER_MIN,
} from "$lib/password.js";
type PasswordOptionKey = keyof PasswordOptions;

const PasswordOptionKeyArray = [
  "number",
  "uppercase",
  "symbol",
  "pin",
] as PasswordOptionKey[];

let sliderValue = $state(PASSWORD_SLIDER_DEFAULT);
let options = $state<PasswordOptions>({
  number: true,
  uppercase: true,
  symbol: true,
  pin: false,
});
let copied = $state(false);
let copyTimeoutId: ReturnType<typeof setTimeout> | undefined;
const mappedLength = $derived(mapSliderToPasswordLength(sliderValue));
let password = $derived(generatePassword(mappedLength, options));

onDestroy(() => {
  if (copyTimeoutId) {
    clearTimeout(copyTimeoutId);
  }
});

function toggleOption(option: PasswordOptionKey) {
  const isActive = options[option];

  if (option === "pin") {
    if (!isActive) {
      options.number = false;
      options.uppercase = false;
      options.symbol = false;
    }
    options.pin = !isActive;
    return;
  }

  if (!isActive) {
    options.pin = false;
  }

  options[option] = !isActive;
}

async function copyPassword() {
  if (!password || typeof navigator === "undefined") {
    return;
  }

  await navigator.clipboard.writeText(password);
  copied = true;
  if (copyTimeoutId) {
    clearTimeout(copyTimeoutId);
  }
  copyTimeoutId = setTimeout(() => {
    copied = false;
  }, 700);
}

function optionLabel(option: PasswordOptionKey): string {
  if (option === "number") return "Number";
  if (option === "uppercase") return "Caps";
  if (option === "symbol") return "Symbol";
  return "PIN";
}
</script>

<section class="mx-auto max-w-2xl py-4">
    <Card>
        <CardHeader>
            <CardTitle>Random Password</CardTitle>
            <CardDescription>
                Generate a secure password with live length and option controls.
            </CardDescription>
        </CardHeader>
        <CardContent class="space-y-6">
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <label class="text-sm font-medium" for="length-slider"
                        >Length</label
                    >
                    <span class="text-sm text-muted-foreground"
                        >{mappedLength}</span
                    >
                </div>
                <Slider
                    type="single"
                    max={PASSWORD_SLIDER_MAX}
                    min={PASSWORD_SLIDER_MIN}
                    step={1}
                    bind:value={sliderValue}
                />
            </div>

            <div class="space-y-2">
                <p class="text-sm font-medium">Options</p>
                <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
                    {#each PasswordOptionKeyArray as option}
                        <Toggle
                            pressed={options[option]}
                            onPressedChange={() => toggleOption(option)}
                            size="sm"
                            class="w-full"
                        >
                            {optionLabel(option)}
                        </Toggle>
                    {/each}
                </div>
            </div>

            <div class="space-y-3">
                <p class="text-sm font-medium">Password</p>
                <Button
                    type="button"
                    variant="outline"
                    size="lg"
                    onclick={copyPassword}
                    class="h-auto min-h-14 w-full px-4 py-3 text-left text-base font-normal"
                >
                    <span
                        class="w-full overflow-x-auto whitespace-nowrap font-mono"
                        >{password}</span
                    >
                </Button>
                <p class="text-xs text-muted-foreground">
                    {copied ? "Copied to clipboard" : "Click password to copy"}
                </p>
            </div>
        </CardContent>
    </Card>
</section>
