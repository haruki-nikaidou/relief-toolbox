import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasmPack from "vite-plugin-wasm-pack";

const crates = ["./apps/sat_solver", "./apps/g_j_elimination"];

// The SSR and client environments both call buildStart concurrently.
// vite-plugin-wasm-pack uses fs-extra.copy which internally does unlink then
// copy (in overwrite mode), causing an ENOENT race condition when two builds
// try to overwrite the same node_modules files simultaneously.
// Wrap the plugin to serialize all buildStart calls behind a single shared
// promise so the copy runs exactly once per build invocation.
let buildStartPromise: Promise<void> | null = null;

function serializedWasmPack(crateList: string[]) {
	const plugin = wasmPack(crateList);
	const origBuildStart = (plugin as any).buildStart;
	(plugin as any).buildStart = async function (this: unknown, opts: unknown) {
		if (buildStartPromise === null) {
			buildStartPromise = origBuildStart.call(this, opts);
		}
		await buildStartPromise;
	};
	return plugin;
}

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), serializedWasmPack(crates)],
	worker: {
		plugins: () => [serializedWasmPack(crates)],
	},
});
