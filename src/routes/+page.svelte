<script lang="ts">
import AppIcon from "./AppIcon.svelte";

interface AppEntry {
  icon: string;
  appName: string;
  href: string;
}

interface AppCollection {
  title: string;
  apps: AppEntry[];
}

const appCollections: AppCollection[] = [
  {
    title: "Generator Tools",
    apps: [
      {
        icon: "mage:key-fill",
        appName: "Random Password",
        href: "/apps/random_password",
      },
    ],
  },
  {
    title: "Daily Life Algorithm",
    apps: [
      {
        icon: "tabler:logic-and",
        appName: "SAT Solver",
        href: "/apps/sat_solver",
      },
      {
        icon: "tabler:matrix",
        appName: "Gauss–Jordan Elimination",
        href: "/apps/gauss_jordan_elimination",
      },
    ],
  },
];
</script>

<main class="container mx-auto px-4 py-8">
  {#each appCollections as appCollection}
	  <h2 class="text-xl font-semibold">{appCollection.title}</h2>
		<section class="flex flex-row flex-wrap gap-4 justify-start mt-4 mb-8">
			{#each appCollection.apps as app}
				<AppIcon icon={app.icon} appName={app.appName} href={app.href} />
			{/each}
		</section>
  {/each}
</main>

