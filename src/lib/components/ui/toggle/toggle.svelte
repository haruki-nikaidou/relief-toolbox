<script lang="ts" module>
import { type VariantProps, tv } from "tailwind-variants";

export const toggleVariants = tv({
  base: "gap-1 rounded-3xl text-sm font-medium transition-all duration-200 [&_svg:not([class*='size-'])]:size-4 group/toggle inline-flex items-center justify-center whitespace-nowrap cursor-pointer select-none outline-none focus-visible:ring-2 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 bg-background text-foreground shadow-outer hover:shadow-outer-sm active:shadow-inner active:translate-y-px aria-pressed:shadow-inner aria-pressed:translate-y-px aria-pressed:bg-muted aria-pressed:text-primary",
  variants: {
    variant: {
      default: "",
      outline: "border border-border/20",
    },
    size: {
      default:
        "h-9 min-w-9 px-4 has-data-[icon=inline-end]:pr-2.5 has-data-[icon=inline-start]:pl-2.5",
      sm: "h-8 min-w-8 px-3 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
      lg: "h-10 min-w-10 px-5 has-data-[icon=inline-end]:pr-3 has-data-[icon=inline-start]:pl-3",
    },
  },
  defaultVariants: {
    variant: "default",
    size: "default",
  },
});

export type ToggleVariant = VariantProps<typeof toggleVariants>["variant"];
export type ToggleSize = VariantProps<typeof toggleVariants>["size"];
export type ToggleVariants = VariantProps<typeof toggleVariants>;
</script>

<script lang="ts">
	import { Toggle as TogglePrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		pressed = $bindable(false),
		class: className,
		size = "default",
		variant = "default",
		...restProps
	}: TogglePrimitive.RootProps & {
		variant?: ToggleVariant;
		size?: ToggleSize;
	} = $props();
</script>

<TogglePrimitive.Root
	bind:ref
	bind:pressed
	data-slot="toggle"
	class={cn(toggleVariants({ variant, size }), className)}
	{...restProps}
/>
