<script lang="ts">
	import { Info } from 'lucide-svelte';
</script>

<div class="h-full overflow-auto bg-surface-1">
	<div class="p-6 max-w-5xl mx-auto space-y-8 font-sans text-gray-900 dark:text-gray-100">
		<!-- Header Section -->
		<div class="text-center space-y-4 mb-10">
			<h1 class="text-4xl font-extrabold tracking-tight text-primary">Acerca de Mega Brisas</h1>
			<p class="text-secondary text-lg max-w-2xl mx-auto">
				Gestión integral de accesos y control de contratistas.
			</p>
		</div>

		<div class="max-w-2xl mx-auto">
			<!-- Info Card (GitHub Box Style) -->
			<div class="rounded-xl border border-surface bg-surface-1 shadow-lg overflow-hidden">
				<!-- Header -->
				<div class="bg-surface-2 px-4 py-3 border-b border-surface flex items-center gap-2">
					<Info class="w-4 h-4 text-secondary" />
					<h3 class="font-bold text-sm text-primary uppercase tracking-wider">
						Información del Sistema
					</h3>
				</div>

				<!-- Body -->
				<div class="p-8 space-y-8 text-sm">
					<p class="text-secondary leading-relaxed">
						Mega Brisas es una plataforma robusta construida con tecnologías de alto rendimiento
						para garantizar la eficiencia operativa y seguridad institucional.
					</p>

					<div class="border rounded-lg border-surface divide-y divide-surface bg-surface-2">
						<div class="flex justify-between items-center px-4 py-3">
							<span class="text-secondary font-bold uppercase text-[10px] tracking-widest"
								>Versión</span
							>
							<div class="flex items-center gap-2">
								<span
									class="bg-surface-3 text-primary px-2 py-0.5 rounded-md text-xs font-mono font-bold border border-surface"
									>v1.0.0-alpha</span
								>
							</div>
						</div>
						<div class="flex justify-between items-center px-4 py-3">
							<span class="text-secondary font-bold uppercase text-[10px] tracking-widest"
								>Desarrollador</span
							>
							<span class="text-primary font-bold">27Design</span>
						</div>
						<div class="flex justify-between items-center px-4 py-3">
							<span class="text-secondary font-bold uppercase text-[10px] tracking-widest"
								>Stack</span
							>
							<span class="text-accent font-bold">Rust - Tauri - SvelteKit - SurrealDB</span>
						</div>
					</div>

					<div
						class="bg-accent/5 text-accent p-4 rounded-lg text-xs border border-accent/20 flex items-center gap-3"
					>
						<div class="p-2 bg-accent/10 rounded-full">
							<Info class="w-4 h-4" />
						</div>
						<div>
							<strong class="block mb-1">¿Necesitas ayuda?</strong>
							<p class="opacity-80">
								Para soporte técnico o reportar problemas, por favor contacta directamente al
								administrador del sistema.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
