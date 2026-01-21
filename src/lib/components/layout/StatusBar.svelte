<!-- src/lib/components/layout/StatusBar.svelte -->
<script lang="ts">
	import { Wifi, WifiOff } from 'lucide-svelte';
	import { online } from '$lib/stores/network';
	import { statusBarInfo } from '$lib/stores/ui';
	import { Database } from 'lucide-svelte';

	// Clases reactivas para el estado de conexión
	$: connectionClasses = $online ? 'text-success' : 'text-error';
	$: connectionIcon = $online ? Wifi : WifiOff;
</script>

<div
	class="flex h-6 items-center justify-between bg-surface-3 px-3
            text-xs text-text-primary select-none font-sans
            max-md:py-1 border-t border-surface"
>
	<!-- Sección izquierda - Estado de conexión -->
	<div class="flex items-center gap-4">
		<div
			class="flex items-center gap-1.5 whitespace-nowrap rounded-sm px-2 py-0.5
             transition-colors duration-200 {connectionClasses}"
			role="status"
			aria-live="polite"
		>
			<svelte:component
				this={connectionIcon}
				size={14}
				class="transition-all duration-300 {$online ? 'animate-pulse' : 'animate-bounce'}"
			/>
			<span class="font-medium">
				{$online ? 'En línea' : 'Sin conexión'}
			</span>
		</div>

		<!-- Separador sutil -->
		<div class="h-3 w-px bg-white/10"></div>

		<!-- Badge de Registros (Sutil) -->
		{#if $statusBarInfo.count !== undefined}
			<div class="flex items-center gap-1.5 px-2 text-blue-400/80">
				<Database size={12} />
				<span class="font-bold text-[11px] uppercase tracking-wider"
					>{$statusBarInfo.count} {$statusBarInfo.label || 'Registros'}</span
				>
			</div>
		{/if}

		<!-- Badge de Selección (Solo si hay algo seleccionado) -->
		{#if $statusBarInfo.selectedCount && $statusBarInfo.selectedCount > 0}
			<div
				class="flex items-center gap-1.5 bg-yellow-400/10 px-2 py-0.5 rounded border border-yellow-400/20 text-yellow-500 animate-in fade-in zoom-in duration-300"
			>
				<span class="font-black text-[10px] uppercase tracking-tighter"
					>{$statusBarInfo.selectedCount} Seleccionados</span
				>
			</div>
		{/if}
	</div>

	<!-- Sección derecha - Espacio reservado para futuras funcionalidades -->
	<div class="flex items-center gap-4 text-gray-500 italic text-[10px]">
		{$statusBarInfo.message || 'Sistema Brisas - Listo'}
	</div>
</div>



