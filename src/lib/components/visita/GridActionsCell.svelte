<script lang="ts">
	import { LogIn, Trash2 } from 'lucide-svelte';

	let { row, onaction } = $props();

	function handleIngreso(e: MouseEvent) {
		e.stopPropagation();
		onaction({ action: 'ingreso', data: row });
	}

	function handleCancel(e: MouseEvent) {
		e.stopPropagation();
		onaction({ action: 'cancel', data: row });
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
