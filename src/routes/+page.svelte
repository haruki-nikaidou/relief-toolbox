<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "$lib/components/ui/card/index.js";
</script>

<section class="mx-auto flex w-full max-w-3xl flex-1 items-center px-4 py-8">
	<Card class="w-full">
		<CardHeader>
			<CardTitle>Generator Tools</CardTitle>
			<CardDescription>Select a tool to start.</CardDescription>
		</CardHeader>
		<CardContent>
			<Button href="/apps/random_password" class="w-full justify-start" variant="outline">
				Random Password
			</Button>
		</CardContent>
	</Card>
</section>
