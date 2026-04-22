<script lang="ts">
import { ToggleGroup as ToggleGroupPrimitive } from "bits-ui";
import { getToggleGroupCtx } from "./toggle-group.svelte";
import { cn } from "$lib/utils.js";
import {
  type ToggleVariants,
  toggleVariants,
} from "$lib/components/ui/toggle/index.js";

let {
  ref = $bindable(null),
  value = $bindable(),
  class: className,
  size,
  variant,
  ...restProps
}: ToggleGroupPrimitive.ItemProps & ToggleVariants = $props();

const ctx = getToggleGroupCtx();
</script>

<ToggleGroupPrimitive.Item
	bind:ref
	data-slot="toggle-group-item"
	data-variant={ctx.variant || variant}
	data-size={ctx.size || size}
	data-spacing={ctx.spacing}
	class={cn(
		// Shared item base
		"shrink-0 focus:z-10 focus-visible:z-10 cursor-pointer select-none transition-all duration-200",
		// Spaced-group items: each has its own raised shadow (same as standalone toggle)
		"group-data-[spacing!=0]/toggle-group:data-[state=on]:shadow-inner group-data-[spacing!=0]/toggle-group:data-[state=on]:translate-y-px group-data-[spacing!=0]/toggle-group:data-[state=on]:bg-muted group-data-[spacing!=0]/toggle-group:data-[state=on]:text-primary",
		// Compact-group (spacing=0) items: no individual shadow; container carries it
		"group-data-[spacing=0]/toggle-group:rounded-none group-data-[spacing=0]/toggle-group:shadow-none group-data-[spacing=0]/toggle-group:hover:shadow-none group-data-[spacing=0]/toggle-group:active:shadow-none group-data-[spacing=0]/toggle-group:px-3 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-2.5 group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-2.5",
		// Compact-group pressed: inner shadow + tint
		"group-data-[spacing=0]/toggle-group:data-[state=on]:shadow-inner group-data-[spacing=0]/toggle-group:data-[state=on]:translate-y-0 group-data-[spacing=0]/toggle-group:data-[state=on]:bg-muted group-data-[spacing=0]/toggle-group:data-[state=on]:text-primary",
		// Rounded ends for horizontal compact group
		"group-data-horizontal/toggle-group:data-[spacing=0]:first:rounded-l-3xl group-data-horizontal/toggle-group:data-[spacing=0]:last:rounded-r-3xl",
		// Rounded ends for vertical compact group
		"group-data-vertical/toggle-group:data-[spacing=0]:first:rounded-t-3xl group-data-vertical/toggle-group:data-[spacing=0]:last:rounded-b-3xl",
		toggleVariants({
			variant: ctx.variant || variant,
			size: ctx.size || size,
		}),
		className
	)}
	{value}
	{...restProps}
/>
