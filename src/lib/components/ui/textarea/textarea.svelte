<script lang="ts">
import { cn, type WithElementRef, type WithoutChildren } from "$lib/utils.js";
import type { HTMLTextareaAttributes } from "svelte/elements";

let {
  ref = $bindable(null),
  value = $bindable(),
  class: className,
  "data-slot": dataSlot = "textarea",
  ...restProps
}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();
</script>

<textarea
	bind:this={ref}
	data-slot={dataSlot}
	class={cn(
		"neuo-input aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 resize-none rounded-2xl px-3 py-3 text-base transition-[color,box-shadow,background-color] aria-invalid:ring-3 md:text-sm placeholder:text-muted-foreground flex field-sizing-content min-h-16 w-full disabled:cursor-not-allowed disabled:opacity-50",
		className
	)}
	bind:value
	{...restProps}
></textarea>
