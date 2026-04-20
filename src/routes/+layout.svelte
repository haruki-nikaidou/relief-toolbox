<script lang="ts">
    import "./layout.css";
    import favicon from "$lib/assets/favicon.svg";

    let { children } = $props();
</script>

<svelte:head
    ><link rel="icon" href={favicon} />
    <title>Relief Toolbox</title>
</svelte:head>

<div class="flex min-h-screen w-full flex-col bg-background">
    <header
        class="z-10 flex items-center px-4"
        style="height: var(--header-height); box-shadow: var(--shadow-outer);"
    >
        <h1 class="m-0 text-2xl font-semibold text-foreground">
            <a href="/"> Relief Toolbox </a>
        </h1>
    </header>
    <div class="flex flex-1 flex-col">
        {@render children()}
    </div>
    <footer style="box-shadow: var(--shadow-outer);" class="p-4">
        <p>
            Open source collection of tools to help you with your daily tasks.
            Built with SvelteKit and Rust.
        </p>
        <p>
            <a
                href="https://github.com/haruki-nikaidou/relief-toolbox"
                class="text-primary underline underline-offset-1"
                target="_blank"
                rel="noopener noreferrer"
            >
                Github
            </a>
        </p>
    </footer>
</div>
