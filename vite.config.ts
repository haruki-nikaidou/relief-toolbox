import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), wasmPack(['./apps/sat_solver', './apps/g_j_elimination'])],
	worker: {
		plugins: () => [wasmPack(['./apps/sat_solver', './apps/g_j_elimination'])],
	},
});
