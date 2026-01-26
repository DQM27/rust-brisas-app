<script lang="ts">
	import { DownloadCloud, RefreshCw, CheckCircle2 } from 'lucide-svelte';
	import { scale } from 'svelte/transition';
	import { checkAndInstallUpdate } from '$lib/services/updateService';

	let isChecking = $state(false);

	async function handleCheckUpdate() {
		isChecking = true;
		try {
			await checkAndInstallUpdate();
		} finally {
			isChecking = false;
		}
	}
</script>

<div
	class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
	in:scale={{ duration: 300, start: 0.95 }}
>
	<div class="max-w-2xl">
		<!-- Header -->
		<div class="mb-6">
			<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">Actualizaciones</h2>
			<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
				Verifica si hay nuevas versiones disponibles.
			</p>
		</div>

		<!-- Card GitHub Style -->
		<div
			class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
		>
			<!-- Header -->
			<div
				class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
			>
				<DownloadCloud class="w-4 h-4 text-gray-500" />
				<h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
					Sistema de Actualizacion
				</h3>
			</div>

			<!-- Body -->
			<div class="p-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-3">
						<div class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-300">
							<CheckCircle2 class="w-4 h-4 text-green-500" />
							<span>Version actual instalada</span>
						</div>
						<span
							class="bg-gray-100 dark:bg-[#21262d] text-gray-700 dark:text-gray-300 px-2 py-0.5 rounded-full text-xs font-mono border border-gray-200 dark:border-gray-600"
						>
							v1.2.0
						</span>
					</div>

					<button
						class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md border transition-colors
              {isChecking
							? 'bg-gray-100 dark:bg-[#21262d] text-gray-500 dark:text-gray-400 border-gray-300 dark:border-gray-600 cursor-not-allowed'
							: 'bg-[#2da44e] hover:bg-[#2c974b] text-white border-[#2da44e] hover:border-[#2c974b]'}"
						onclick={handleCheckUpdate}
						disabled={isChecking}
					>
						{#if isChecking}
							<RefreshCw class="w-4 h-4 animate-spin" />
							<span>Buscando...</span>
						{:else}
							<RefreshCw class="w-4 h-4" />
							<span>Buscar actualizacion</span>
						{/if}
					</button>
				</div>

				<p class="text-xs text-gray-500 dark:text-gray-400 mt-4">
					Las actualizaciones se descargan de forma segura desde el servidor oficial.
				</p>
			</div>
		</div>
	</div>
</div>
