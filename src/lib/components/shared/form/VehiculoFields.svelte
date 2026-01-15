<script lang="ts">
	import { Truck } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';

	interface Props {
		// Stores de superforms
		form: any;
		errors: any;
		constraints: any;
		tainted?: any; // Para verificar si cambió
		loading?: boolean;
		readonly?: boolean;
		originalPlaca?: string; // Para evitar validar el valor original
	}

	let {
		form,
		errors,
		constraints,
		loading = false,
		readonly = false,
		originalPlaca = ''
	}: Props = $props();

	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all appearance-none';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
	const errorClass = 'text-xs text-red-500 mt-1 pb-1';

	let checkTimeout: ReturnType<typeof setTimeout>;

	function handlePlacaInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const value = input.value.trim().toUpperCase();

		// Auto upper-case
		if (input.value !== value) {
			input.value = value;
			$form.placa = value;
		} else {
			$form.placa = value; // Ensure form value is updated even if no case change
		}

		if (checkTimeout) clearTimeout(checkTimeout);

		// Solo validar si cambió respecto al original Y tiene longitud
		if (value.length < 3 || value === originalPlaca) {
			// Clear existing placa unique error if it's the original value or too short
			errors.update((errs: Record<string, string[] | undefined>) => {
				const newErrs = { ...errs };
				if (newErrs.placa && newErrs.placa.includes('Esta placa ya existe.')) {
					delete newErrs.placa;
				}
				return newErrs;
			});
			return;
		}

		checkTimeout = setTimeout(async () => {
			try {
				// Validamos contra la tabla 'vehiculo'
				const isUnique = await invoke<boolean>('check_unique', {
					table: 'vehiculo',
					field: 'placa',
					value,
					excludeId: null // No tenemos vehiculoId fácil, confiamos en originalPlaca check
				});

				if (!isUnique) {
					errors.update((errs: Record<string, string[] | undefined>) => ({
						...errs,
						placa: ['Esta placa ya existe.']
					}));
				} else {
					errors.update((errs: Record<string, string[] | undefined>) => {
						const newErrs = { ...errs };
						if (newErrs.placa && newErrs.placa.includes('Esta placa ya existe.')) {
							delete newErrs.placa;
						}
						return newErrs;
					});
				}
			} catch (e) {
				console.error('Error validando placa:', e);
			}
		}, 400);
	}

	// Efecto para limpiar campos cuando se desmarca (asumiendo que el componente padre
	// o el super refine schema se encargan, pero aquí podemos ayudar visualmente si se requiere
	// aunque sveltekit-superforms maneja el estado globalmente).
	// La lógica de limpieza suele estar mejor en el componente padre o en un efecto global sobre el store.
</script>

<div class="bg-surface-1 p-5 rounded-lg border border-surface">
	<div class="flex items-center justify-between mb-5">
		<h3 class="text-base font-semibold text-primary flex items-center gap-2 m-0 border-0 p-0">
			<Truck size={18} />
			Coche / Vehículo
		</h3>

		<div class="flex items-center gap-3">
			<label class="relative inline-flex items-center cursor-pointer">
				<input
					type="checkbox"
					bind:checked={$form.tieneVehiculo}
					class="sr-only peer"
					disabled={loading || readonly}
				/>
				<div
					class="w-9 h-5 bg-surface-3 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"
				></div>
			</label>
			<span class="text-xs font-medium text-gray-300">Tiene Vehículo</span>
		</div>
	</div>

	{#if $form.tieneVehiculo}
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4 animate-scale-in">
			<div>
				<label for="placa" class={labelClass}>Placa *</label>
				<input
					id="placa"
					name="placa"
					type="text"
					bind:value={$form.placa}
					oninput={handlePlacaInput}
					disabled={loading || readonly}
					class={inputClass}
					placeholder="M 123-456"
					{...$constraints.placa}
				/>
				{#if $errors.placa}<p class={errorClass}>{$errors.placa}</p>{/if}
			</div>

			<div>
				<label for="tipoVehiculo" class={labelClass}>Tipo Vehículo *</label>
				<select
					id="tipoVehiculo"
					name="tipoVehiculo"
					bind:value={$form.tipoVehiculo}
					disabled={loading || readonly}
					class={inputClass}
					{...$constraints.tipoVehiculo}
				>
					<option value="">Seleccione tipo</option>
					<option value="AUTOMOVIL">Automóvil</option>
					<option value="MOTOCICLETA">Motocicleta</option>
					<option value="CAMIONETA">Camioneta</option>
					<option value="CAMION">Camión</option>
					<option value="OTRO">Otro</option>
				</select>
				{#if $errors.tipoVehiculo}<p class={errorClass}>
						{$errors.tipoVehiculo}
					</p>{/if}
			</div>

			<div>
				<label for="marca" class={labelClass}>Marca</label>
				<input
					id="marca"
					name="marca"
					type="text"
					bind:value={$form.marca}
					disabled={loading || readonly}
					class={inputClass}
					placeholder="Toyota"
					{...$constraints.marca}
				/>
				{#if $errors.marca}<p class={errorClass}>{$errors.marca}</p>{/if}
			</div>

			<div>
				<label for="modelo" class={labelClass}>Modelo</label>
				<input
					id="modelo"
					name="modelo"
					type="text"
					bind:value={$form.modelo}
					disabled={loading || readonly}
					class={inputClass}
					placeholder="Hilux"
					{...$constraints.modelo}
				/>
				{#if $errors.modelo}<p class={errorClass}>{$errors.modelo}</p>{/if}
			</div>

			<div>
				<label for="color" class={labelClass}>Color</label>
				<input
					id="color"
					name="color"
					type="text"
					bind:value={$form.color}
					disabled={loading || readonly}
					class={inputClass}
					placeholder="Blanco"
					{...$constraints.color}
				/>
				{#if $errors.color}<p class={errorClass}>{$errors.color}</p>{/if}
			</div>
		</div>
	{/if}
</div>
