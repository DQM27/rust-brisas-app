<script lang="ts">
	import { fade } from 'svelte/transition';
	import { X, Code2 } from 'lucide-svelte';
	import { scale } from 'svelte/transition';
	import { onDestroy } from 'svelte';
	import { APP_CONFIG } from '$lib/config/app';
	import { getVersion } from '@tauri-apps/api/app';

	interface Props {
		show: boolean;
		onClose: () => void;
	}

	let { show, onClose }: Props = $props();

	// Versión dinámica - fetch al inicio (Svelte 5 idiom)
	let appVersion = $state<string>(APP_CONFIG.version);

	// Fetch version immediately on component mount
	getVersion()
		.then((v) => {
			if (v) appVersion = v;
		})
		.catch((err) => console.error('Error fetching version:', err));

	// Lista de colaboradores (hardcoded por ahora)
	const contributors = [
		'Daniel Quintana',
		'María González',
		'Carlos Rodríguez',
		'Ana Martínez',
		'José López',
		'Laura Sánchez',
		'Pedro García',
		'Sofia Hernández',
		'Miguel Torres',
		'Valentina Díaz'
	];

	let currentIndex = $state(0);
	let intervalId: ReturnType<typeof setInterval> | null = null;

	$effect(() => {
		if (show) {
			currentIndex = 0;
			intervalId = setInterval(() => {
				currentIndex = (currentIndex + 1) % contributors.length;
			}, 2500);
		} else if (intervalId) {
			clearInterval(intervalId);
			intervalId = null;
		}
	});

	onDestroy(() => {
		if (intervalId) clearInterval(intervalId);
	});
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="relative z-10 w-full max-w-sm overflow-hidden rounded-xl bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="pt-8 pb-4 flex justify-center relative">
				<button
					onclick={onClose}
					class="absolute top-2 right-2 p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
					aria-label="Cerrar"
				>
					<X size={20} />
				</button>
				<div
					class="overflow-hidden bg-white dark:bg-[#161b22] rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700"
				>
					<img src="/icono-brisas.png" alt="Brisas" class="w-24 h-24 object-cover" />
				</div>
			</div>

			<!-- Body -->
			<div class="pb-8 px-6 text-center">
				<h2 class="text-xl font-bold text-gray-900 dark:text-white mb-1">
					{APP_CONFIG.name}
				</h2>
				<p class="text-sm text-gray-500 dark:text-gray-400 mb-6 font-medium">
					{APP_CONFIG.description}
				</p>

				<div class="space-y-4">
					<div class="text-xs text-gray-500 dark:text-gray-400 flex flex-wrap justify-center gap-2">
						<span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]">Rust</span>
						<span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]">Tauri</span>
						<span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]">SvelteKit</span>
						<span class="px-2 py-1 rounded bg-gray-100 dark:bg-[#161b22]">SurrealDB</span>
					</div>

					<div
						class="flex items-center justify-between p-2 rounded-lg bg-gray-50 dark:bg-[#161b22] border border-gray-100 dark:border-gray-700/50"
					>
						<span class="text-xs text-gray-500 dark:text-gray-400">Versión</span>
						<span class="font-mono text-xs font-medium text-gray-900 dark:text-gray-200"
							>{appVersion}</span
						>
					</div>

					<!-- Agradecimientos especiales con scroll suave -->
					<div
						class="p-3 rounded-lg bg-gray-50 dark:bg-[#161b22] border border-gray-100 dark:border-gray-700/50"
					>
						<div class="flex items-center justify-center gap-2 mb-2">
							<span
								class="text-xs text-gray-400 dark:text-gray-500 uppercase tracking-wider font-semibold"
							>
								✨ Agradecimiento Especial
							</span>
						</div>
						<div class="h-5 overflow-hidden relative">
							<div class="credits-scroll">
								{#each [...contributors, ...contributors] as name}
									<span
										class="block text-center text-sm font-medium text-gray-700 dark:text-gray-300 py-0.5"
									>
										{name}
									</span>
								{/each}
							</div>
						</div>
					</div>

					<div class="pt-4 border-t border-gray-100 dark:border-gray-700/50">
						<p
							class="text-xs text-gray-400 dark:text-gray-500 uppercase tracking-wider font-semibold mb-3"
						>
							Desarrollado por
						</p>
						<div
							class="flex items-center justify-center gap-2 text-gray-800 dark:text-gray-200 font-medium"
						>
							<Code2 class="w-4 h-4 text-blue-500" />
							<span>27Design</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div
				class="bg-gray-50 dark:bg-[#161b22] px-6 py-3 border-t border-gray-200 dark:border-gray-700 text-center"
			>
				<p class="text-xs text-gray-400 dark:text-gray-500">
					© {new Date().getFullYear()} Todos los derechos reservados
				</p>
			</div>
		</div>
	</div>
{/if}

<style>
	.credits-scroll {
		animation: scroll-up 50s linear infinite;
	}

	@keyframes scroll-up {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(-50%);
		}
	}

	.credits-scroll:hover {
		animation-play-state: paused;
	}
</style>
