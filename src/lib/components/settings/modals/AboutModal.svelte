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
			class="relative z-10 w-full max-w-sm overflow-hidden rounded-xl bg-surface-1 shadow-2xl border border-surface"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			<div class="pt-8 pb-4 flex justify-center relative">
				<button
					onclick={onClose}
					class="absolute top-2 right-2 p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
					aria-label="Cerrar"
				>
					<X size={20} />
				</button>
				<div class="overflow-hidden bg-surface-2 rounded-2xl shadow-sm border border-surface">
					<img src="/icono-brisas.png" alt="Brisas" class="w-24 h-24 object-cover" />
				</div>
			</div>

			<!-- Body -->
			<div class="pb-8 px-6 text-center">
				<h2 class="text-xl font-bold text-primary mb-1">
					{APP_CONFIG.name}
				</h2>
				<p class="text-sm text-secondary mb-6 font-medium">
					{APP_CONFIG.description}
				</p>

				<div class="space-y-4">
					<div class="text-[10px] font-bold text-secondary flex flex-wrap justify-center gap-2">
						<span class="px-2 py-1 rounded-md bg-surface-3 border border-surface">RUST</span>
						<span class="px-2 py-1 rounded-md bg-surface-3 border border-surface">TAURI</span>
						<span class="px-2 py-1 rounded-md bg-surface-3 border border-surface">SVELTEKIT</span>
						<span class="px-2 py-1 rounded-md bg-surface-3 border border-surface">SURREALDB</span>
					</div>

					<div
						class="flex items-center justify-between p-2.5 rounded-lg bg-surface-2 border border-surface shadow-inner"
					>
						<span class="text-xs text-secondary font-medium uppercase tracking-wider">Versión</span>
						<span class="font-mono text-xs font-bold text-primary">{appVersion}</span>
					</div>

					<!-- Agradecimientos especiales con scroll suave -->
					<div class="p-4 rounded-lg bg-surface-2 border border-surface">
						<div class="flex items-center justify-center gap-2 mb-2">
							<span class="text-[10px] text-accent font-bold uppercase tracking-widest">
								✨ Agradecimiento Especial
							</span>
						</div>
						<div class="h-24 overflow-hidden relative credits-mask">
							<div class="credits-scroll">
								{#each [...contributors, ...contributors, ...contributors] as name}
									<span
										class="block text-center text-sm font-bold text-white py-2 opacity-40 hover:opacity-100 transition-opacity duration-300 cursor-default"
									>
										{name}
									</span>
								{/each}
							</div>
						</div>
					</div>

					<div class="pt-4 border-t border-surface">
						<p class="text-[10px] text-tertiary uppercase tracking-widest font-bold mb-3">
							Desarrollado por
						</p>
						<div class="flex items-center justify-center gap-2 text-primary font-bold">
							<Code2 class="w-4 h-4 text-accent" />
							<span class="text-lg">27Design</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div class="bg-surface-2 px-6 py-4 border-t border-surface text-center">
				<p class="text-[10px] text-tertiary">
					© {new Date().getFullYear()} Todos los derechos reservados
				</p>
			</div>
		</div>
	</div>
{/if}

<style>
	.credits-mask {
		mask-image: linear-gradient(to bottom, transparent, black 25%, black 75%, transparent);
		-webkit-mask-image: linear-gradient(to bottom, transparent, black 25%, black 75%, transparent);
	}

	.credits-scroll {
		animation: scroll-up 40s linear infinite;
	}

	@keyframes scroll-up {
		0% {
			transform: translateY(0);
		}
		100% {
			transform: translateY(-33.33%);
		}
	}

	.credits-scroll:hover {
		animation-play-state: paused;
	}

	.credits-scroll span {
		text-shadow: 0 0 12px rgba(59, 130, 246, 0.15);
	}
</style>
