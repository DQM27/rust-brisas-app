<script lang="ts">
	import { showLogoutWarning } from '$lib/stores/sessionStore';
	import { fade, fly } from 'svelte/transition';
	import { AlertTriangle, Clock } from 'lucide-svelte';

	function dismiss() {
		// Just moving the mouse triggers recordActivity which clears the idleness,
		// but we can also forcefully close it (though the next check would likely reopen it if idle remains)
		// The logic in sessionStore clears the warning state if not idle.
		// So this button is more of an "I'm here!" signal.
		// Triggering an event is enough.
		// We'll simulate a mouse move
		window.dispatchEvent(new Event('mousemove'));
	}
</script>

{#if $showLogoutWarning}
	<div
		class="fixed bottom-4 right-4 z-[9999] max-w-sm w-full bg-red-900/90 text-white rounded-lg shadow-2xl border border-red-500/50 backdrop-blur-sm p-4"
		transition:fly={{ y: 20, duration: 300 }}
	>
		<div class="flex items-start gap-3">
			<div class="p-2 bg-red-500/20 rounded-full shrink-0">
				<Clock class="w-6 h-6 text-red-200 animate-pulse" />
			</div>

			<div class="flex-1">
				<h3 class="font-bold text-lg mb-1">Cierre de sesión inminente</h3>
				<p class="text-sm text-red-100 opacity-90 mb-3">
					Tu sesión se cerrará en menos de 1 minuto por inactividad. ¿Sigues ahí?
				</p>

				<button
					onclick={dismiss}
					class="w-full py-2 bg-white text-red-900 font-bold rounded hover:bg-red-50 transition-colors text-sm"
				>
					¡Estoy aquí! Mantener sesión
				</button>
			</div>
		</div>
	</div>
{/if}
