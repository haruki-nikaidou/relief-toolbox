<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} />
    <title>Relief Toolbox</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col bg-background">
	<header
		class="z-10 flex items-center px-4"
		style="height: var(--header-height); box-shadow: var(--shadow-outer);"
	>
		<h1 class="m-0 text-2xl font-semibold text-foreground">Relief Toolbox</h1>
	</header>
	<main class="flex flex-1 flex-col">
		{@render children()}
	</main>
</div>
