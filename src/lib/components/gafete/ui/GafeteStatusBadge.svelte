<!-- src/lib/components/gafete/ui/GafeteStatusBadge.svelte -->
<script lang="ts">
	import { CheckCircle, PlayCircle, AlertCircle, Zap, HelpCircle, BadgeAlert } from 'lucide-svelte';

	interface Props {
		status: string;
		showIcon?: boolean;
		size?: 'xs' | 'sm' | 'md' | 'lg';
		animate?: boolean;
	}

	let { status, showIcon = true, size = 'md', animate = false }: Props = $props();

	const statusConfigs: Record<
		string,
		{ label: string; classes: string; icon: any; iconColor: string }
	> = {
		disponible: {
			label: 'Disponible',
			classes:
				'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20',
			icon: CheckCircle,
			iconColor: 'text-emerald-500'
		},
		activo: {
			label: 'Disponible',
			classes:
				'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20',
			icon: CheckCircle,
			iconColor: 'text-emerald-500'
		},
		en_uso: {
			label: 'En Uso',
			classes:
				'bg-blue-500/10 text-blue-600 border-blue-500/20 dark:bg-blue-500/10 dark:text-blue-400 dark:border-blue-500/20',
			icon: PlayCircle,
			iconColor: 'text-blue-500'
		},
		perdido: {
			label: 'Perdido',
			classes:
				'bg-red-500/10 text-red-600 border-red-500/20 dark:bg-red-500/10 dark:text-red-400 dark:border-red-500/20',
			icon: BadgeAlert,
			iconColor: 'text-red-500'
		},
		danado: {
			label: 'Dañado',
			classes:
				'bg-rose-500/10 text-rose-600 border-rose-500/20 dark:bg-rose-500/10 dark:text-rose-400 dark:border-rose-500/20',
			icon: Zap,
			iconColor: 'text-rose-500'
		},
		extraviado: {
			label: 'Extraviado',
			classes:
				'bg-amber-500/10 text-amber-600 border-amber-500/20 dark:bg-amber-500/10 dark:text-amber-400 dark:border-amber-500/20',
			icon: HelpCircle,
			iconColor: 'text-amber-500'
		}
	};

	const config = $derived(
		statusConfigs[status] || {
			label: status || 'Desconocido',
			classes:
				'bg-gray-500/10 text-gray-600 border-gray-500/20 dark:bg-gray-500/10 dark:text-gray-400 dark:border-gray-500/20',
			icon: AlertCircle,
			iconColor: 'text-gray-500'
		}
	);

	const sizeClasses = {
		xs: 'px-1.5 py-0.5 text-[9px] gap-1',
		sm: 'px-2 py-0.5 text-[10px] gap-1.5',
		md: 'px-2.5 py-0.5 text-xs gap-1.5',
		lg: 'px-3 py-1 text-sm gap-2'
	};

	const iconSizes = {
		xs: 10,
		sm: 11,
		md: 13,
		lg: 15
	};
</script>

<span
	class="inline-flex items-center font-bold rounded-full border shadow-sm backdrop-blur-[2px] transition-all duration-300 hover:scale-[1.02] cursor-default {sizeClasses[
		size
	]} {config.classes}"
>
	{#if showIcon}
		<div class="flex items-center justify-center {animate ? 'animate-pulse' : ''}">
			<config.icon
				size={iconSizes[size]}
				class="{config.iconColor} shrink-0 opacity-90"
				strokeWidth={2.5}
			/>
		</div>
	{/if}
	<span class="uppercase tracking-widest leading-none">
		{config.label}
	</span>
</span>

<style>
	/* Puedes agregar efectos de brillo o glassmorphism extra aquí si quieres */
</style>
