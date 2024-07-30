import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { internalIpV4 } from 'internal-ip';

// https://vitejs.dev/config/
export default defineConfig(async () => {
	const host = await internalIpV4();
	console.log(`Vite dev server running at http://${host}:5173`);

	/** @type {import('vite').UserConfig} */
	const config = {
		plugins: [sveltekit()],
		test: {
			include: ['src/**/*.{test,spec}.{js,ts}']
		},
		clearScreen: false,
		server: {
			strictPort: true
		},
		envPrefix: ['VITE_', 'TAURI_']
	};

	return config;
});
