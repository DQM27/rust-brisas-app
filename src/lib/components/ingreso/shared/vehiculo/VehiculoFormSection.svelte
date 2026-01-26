<script lang="ts">
	import { slide } from 'svelte/transition';

	import type { VehiculoResponse } from '$lib/types/vehiculo';

	export let tieneVehiculo: boolean = false;
	export let vehiculoId: string | null | undefined = undefined; // ID selected
	export let vehiculosRegistrados: VehiculoResponse[] = []; // List of vehicles available for the person
	export let error: string | undefined = undefined;

	// Si hay vehiculos registrados y se activa tieneVehiculo, preseleccionar el primero o dejar vacio?
	$: if (tieneVehiculo && vehiculosRegistrados.length === 1 && !vehiculoId) {
		vehiculoId = vehiculosRegistrados[0].id;
	}
</script>

<div class="bg-base-200 p-4 rounded-lg border border-base-300">
	<div class="form-control">
		<label class="label cursor-pointer justify-start gap-4">
			<input type="checkbox" class="toggle toggle-primary" bind:checked={tieneVehiculo} />
			<span class="label-text font-medium text-base">¿Ingresa con Vehículo?</span>
		</label>
	</div>

	{#if tieneVehiculo}
		<div transition:slide class="mt-4 flex flex-col gap-3">
			{#if vehiculosRegistrados.length > 0}
				<div class="form-control w-full">
					<label class="label" for="vehiculo-select">
						<span class="label-text">Seleccionar Vehículo Registrado</span>
					</label>
					<select
						id="vehiculo-select"
						class="select select-bordered w-full {error ? 'select-error' : ''}"
						bind:value={vehiculoId}
					>
						<option value="" disabled selected>-- Seleccione --</option>
						{#each vehiculosRegistrados as v}
							<option value={v.id}>
								{v.placa} - {v.marca || 'Marca desc.'}
								{v.modelo || ''}
							</option>
						{/each}
						<option value="otro">Otro / No Registrado (Manual)</option>
					</select>
				</div>
			{:else}
				<div class="alert alert-info text-sm py-2">
					<span>Esta persona no tiene vehículos registrados.</span>
				</div>
			{/if}

			{#if vehiculoId === 'otro' || vehiculosRegistrados.length === 0}
				<!-- 
            Aquí podríamos poner inputs manuales si permitimos registrar vehículos al vuelo.
            Por ahora, el backend espera vehiculo_id opcional.
            Si es "otro", quizás debamos manejarlo como NULL o manejar creación.
            Para este MVP, asumiremos que si es "otro" o no hay, se deja como null en ID,
            o se debería mostrar un aviso de que se debe registrar el vehículo primero en el módulo de vehículos.
           -->
				<div class="alert alert-warning text-sm shadow-sm">
					<span>
						Nota: El registro de nuevos vehículos debe hacerse en el módulo de
						Contratistas/Proveedores. Se registrará el ingreso sin asociar vehículo específico.
					</span>
				</div>
			{/if}

			{#if error}
				<div class="text-error text-sm mt-1">{error}</div>
			{/if}
		</div>
	{/if}
</div>
