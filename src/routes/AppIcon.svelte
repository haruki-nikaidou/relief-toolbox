<script lang="ts">
import Icon from "@iconify/svelte";

interface Props {
  icon: string;
  appName: string;
  href: string;
}

let { icon, appName, href }: Props = $props();
</script>

<a 
	class="neuo-btn h-36 w-36 rounded-xl flex flex-col items-center justify-center gap-2" 
	href={href}
>
    <Icon icon={icon} class="size-12" />
		<p class="text-sm font-semibold text-shadow-lg">{appName}</p>
</a>