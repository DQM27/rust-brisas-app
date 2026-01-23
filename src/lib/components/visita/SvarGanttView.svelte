<script lang="ts">
	import { Gantt, WillowDark } from '@svar-ui/svelte-gantt';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';

	interface Props {
		pendientes: PreRegistroVisita[];
	}

	let { pendientes }: Props = $props();

	// SVAR Gantt expects scales, tasks and links
	const scales = $derived([
		{ unit: 'day', step: 1, format: 'MMM dd' },
		{ unit: 'hour', step: 1, format: 'HH:mm' }
	]);

	const tasks = $derived(
		pendientes.map((p) => ({
			id: p.id,
			text: `${p.nombre} ${p.apellido}`,
			start_date: new Date(`${p.fecha_esperada}T${p.hora_esperada || '00:00'}`),
			duration: 1, // 1 hour default
			type: 'task'
		}))
	);
</script>

<div class="h-full w-full bg-surface-1">
	<WillowDark>
		<Gantt {tasks} {scales} />
	</WillowDark>
</div>

<style>
	:global(.wx-gantt) {
		height: 100%;
	}
</style>
