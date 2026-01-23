<script lang="ts">
	import { getContext } from 'svelte';
	import { LogIn, Trash2 } from 'lucide-svelte';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';

	let { row } = $props();

	const actions = getContext<{
		ingreso: (r: PreRegistroVisita) => void;
		cancel: (r: PreRegistroVisita) => void;
	}>('gridActions');

	function handleIngreso(e: MouseEvent) {
		e.stopPropagation();
		actions.ingreso(row);
	}

	function handleCancel(e: MouseEvent) {
		e.stopPropagation();
		actions.cancel(row);
	}
</script>

<div class="flex items-center gap-3 px-2 h-full">
	<button
		onclick={handleIngreso}
		title="Dar Ingreso"
		class="text-blue-400 hover:text-blue-300 transition-colors p-1"
	>
		<LogIn size={16} />
	</button>
	<button
		onclick={handleCancel}
		title="Borrar"
		class="text-red-400 hover:text-red-300 transition-colors p-1"
	>
		<Trash2 size={16} />
	</button>
</div>
