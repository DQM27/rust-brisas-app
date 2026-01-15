<script lang="ts">
	import type { ValidacionIngresoResult } from '$lib/logic/ingreso/types';

	export let validation: ValidacionIngresoResult;
	export let onAuthorize: () => void = () => {}; // For overriding blocked status if allowed

	// Determine Main State
	$: isBlocked = !validation.puedeIngresar;
	$: hasWarnings = validation.alertas.length > 0;
	$: ingresoAbierto = validation.ingresoAbierto;
</script>

<div
	class="card bg-base-100 shadow-md border-2
    {isBlocked ? 'border-error' : hasWarnings ? 'border-warning' : 'border-success'}"
>
	<div class="card-body p-6">
		<div class="flex items-start gap-4">
			<!-- Icon -->
			<div class="text-4xl mt-1">
				{#if isBlocked}
					🔴
				{:else if hasWarnings}
					🟡
				{:else}
					🟢
				{/if}
			</div>

			<div class="flex-1">
				<h3 class="card-title text-xl font-bold mb-1">
					{#if isBlocked}
						Ingreso NO Autorizado
					{:else if hasWarnings}
						Autorizado con Advertencias
					{:else}
						Ingreso Autorizado
					{/if}
				</h3>

				<p class="text-base-content/70 text-sm">
					{#if validation.persona}
						Validación para <strong>{validation.persona.nombreCompleto}</strong>
						({validation.persona.cedula})
					{/if}
				</p>

				<!-- Bloqueos Details -->
				<!-- Assuming `bloqueos` is exposed or derived from validation in a standardized way. 
                     If validation.motivoRechazo is simple string, we use that if no structured blocks.
                     But `validation` comes from types.ts where we have `bloqueos`? 
                     No, types.ts has `motivoRechazo?: string` for generic.
                     Ideally we should map the structured reasons correctly. 
                     For now, let's assume we display motivoRechazo if blocked.
                -->
				{#if isBlocked && validation.motivoRechazo}
					<div class="alert alert-error mt-4 shadow-sm">
						<span>{validation.motivoRechazo}</span>
					</div>
				{/if}

				{#if ingresoAbierto}
					<div class="alert alert-warning mt-4 shadow-sm">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							class="stroke-current shrink-0 w-6 h-6"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
							></path></svg
						>
						<div>
							<h3 class="font-bold">Ingreso Previo Activo</h3>
							<div class="text-xs">
								Entrada: {new Date(ingresoAbierto.fechaHoraIngreso).toLocaleString('es-ES', {
									hour12: false
								})}
							</div>
						</div>
					</div>
				{/if}

				<!-- Warnings -->
				{#if validation.alertas.length > 0}
					<div class="mt-4 flex flex-col gap-2">
						{#each validation.alertas as alerta}
							<div class="alert alert-warning py-2 text-sm flex items-center">
								<span>⚠️ {alerta}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Actions Area (e.g. Override) -->
			{#if isBlocked}
				<div class="flex flex-col gap-2">
					<button class="btn btn-sm btn-outline btn-error" on:click={onAuthorize}>
						Autorizar Excepcionalmente
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>
