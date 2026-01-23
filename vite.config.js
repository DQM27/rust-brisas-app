import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
		tailwindcss(), // DEBE estar ANTES de sveltekit
		sveltekit()
	],
	define: {
		__APP_VERSION__: JSON.stringify('2.5.0'),
	},
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		watch: {
			ignored: ['**/src-tauri/**']
		}
	}
});
