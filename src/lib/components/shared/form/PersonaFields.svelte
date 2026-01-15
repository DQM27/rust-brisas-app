<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Plus, ChevronDown } from 'lucide-svelte';

	interface Props {
		// Aceptamos el objeto 'form' (store), 'errors' (store), y 'constraints' (store)
		// Usamos 'any' genérico porque el esquema varía ligeramente entre Contratista/Proveedor
		// pero los campos base (nombre, apellido, cedula) son coincidentes.
		form: any;
		errors: any;
		constraints: any;
		validate: (path: any, opts?: any) => Promise<any>;
		loading?: boolean;
		isEditMode?: boolean;
		readonly?: boolean;
		empresas?: Array<{ id: string; nombre: string }>;
		showEmpresa?: boolean;
		tableName?: string; // Tabla para validar unicidad (ej: "proveedor", "contratista")
		currentId?: string; // ID para excluir en validación (edición)
		onCreateEmpresa?: () => void; // Callback para crear nueva empresa
	}

	let {
		form,
		errors,
		constraints,
		validate,
		loading = false,
		isEditMode = false,
		readonly = false,
		empresas = [],
		showEmpresa = true,
		tableName,
		currentId,
		onCreateEmpresa
	}: Props = $props();

	let checkTimeout: ReturnType<typeof setTimeout>;
	let cedulaDuplicateError = $state<string | null>(null); // Estado independiente para persistencia
	let showEmpresaDropdown = $state(false);

	// Validación en tiempo real (debounced)
	function handleCedulaInput(event: Event) {
		const input = event.target as HTMLInputElement;
		const value = input.value;

		// Trigger Superforms validation immediately for real-time feedback (regex, length)
		validate('cedula');

		// Normalizar entrada (formato regex básico visual)
		// Dejar que Zod maneje el formato estricto, aquí solo unicidad

		if (checkTimeout) clearTimeout(checkTimeout);

		// Solo validar si tiene longitud decente
		if (value.length < 4 || !tableName) return;

		checkTimeout = setTimeout(async () => {
			console.log(`🔍 Checking uniqueness for ${tableName}.${value}...`);
			try {
				const isUnique = await invoke<boolean>('check_unique', {
					table: tableName,
					field: 'cedula',
					value,
					excludeId: currentId
				});

				console.log(`✅ Uniqueness result: ${isUnique}`);

				if (!isUnique) {
					cedulaDuplicateError = 'Esta cédula ya está registrada.';
					// También forzamos el error en el store para control (opcional)
					errors.update((errs: Record<string, string[] | undefined>) => ({
						...errs,
						cedula: ['Esta cédula ya está registrada.']
					}));
				} else {
					cedulaDuplicateError = null;
					// Limpiamos error solo si era de duplicado
					errors.update((errs: Record<string, string[] | undefined>) => {
						const newErrs = { ...errs };
						if (newErrs.cedula && newErrs.cedula.includes('registrada')) {
							delete newErrs.cedula;
						}
						return newErrs;
					});
				}
			} catch (e) {
				console.error('Error validando unicidad:', e);
			}
		}, 400); // 400ms debounce
	}

	// Lógica reactiva para clases de input

	function getInputClass(
		hasError: boolean,
		isReadonly: boolean,
		value: any,
		isRequired: boolean = false
	) {
		const base =
			'w-full bg-black/20 border rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none disabled:opacity-50';

		if (hasError) return `${base} !border-red-500/50 !ring-1 !ring-red-500/20`;

		// Success state: ONLY for required fields that are not empty and not readonly
		if (isRequired && value && String(value).trim() !== '' && !isReadonly) {
			return `${base} !border-green-500/50 !ring-1 !ring-green-500/20`;
		}

		const standard = isReadonly ? 'border-white/5 opacity-70 bg-gray-800/10' : 'border-white/10';

		return `${base} ${standard}`;
	}

	const labelClass = 'block text-xs font-medium text-secondary mb-1';
	const errorClass = 'text-xs text-red-500 mt-0.5';
</script>

<div>
	<!-- Datos Personales header removed for consistency with Contratista simple grid -->

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
		<!-- Cédula (Full Width) -->
		<div class="col-span-1 lg:col-span-2">
			<label for="cedula" class={labelClass}
				>Cédula <span class="text-red-500 ml-0.5">*</span></label
			>
			<input
				id="cedula"
				name="cedula"
				type="text"
				bind:value={$form.cedula}
				oninput={handleCedulaInput}
				aria-invalid={$errors.cedula || cedulaDuplicateError ? 'true' : undefined}
				disabled={loading || isEditMode || readonly}
				class={getInputClass(
					!!($errors.cedula || cedulaDuplicateError),
					isEditMode || readonly,
					$form.cedula,
					true // isRequired
				)}
				placeholder="Ej: 001-010203-0001A"
				{...$constraints.cedula}
			/>
			{#if $errors.cedula || cedulaDuplicateError}
				<p class={errorClass}>{$errors.cedula || cedulaDuplicateError}</p>
			{/if}
		</div>
	</div>

	<!-- Fila 2: Nombres (Nombre + Segundo Nombre) -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
		<div>
			<label for="nombre" class={labelClass}
				>Nombre <span class="text-red-500 ml-0.5">*</span></label
			>
			<input
				id="nombre"
				name="nombre"
				type="text"
				bind:value={$form.nombre}
				oninput={() => validate('nombre')}
				disabled={loading || readonly}
				class={getInputClass(!!$errors.nombre, readonly, $form.nombre, true)}
				placeholder="Juan"
				{...$constraints.nombre}
			/>
			{#if $errors.nombre}<p class={errorClass}>{$errors.nombre}</p>{/if}
		</div>

		<div>
			<label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
			<input
				id="segundoNombre"
				name="segundoNombre"
				type="text"
				bind:value={$form.segundoNombre}
				oninput={() => validate('segundoNombre')}
				disabled={loading || readonly}
				class={getInputClass(
					!!$errors.segundoNombre,
					readonly,
					$form.segundoNombre,
					false // isRequired
				)}
				placeholder="Carlos"
				{...$constraints.segundoNombre}
			/>
			{#if $errors.segundoNombre}<p class={errorClass}>
					{$errors.segundoNombre}
				</p>{/if}
		</div>
	</div>

	<!-- Fila 3: Apellidos (Apellido + Segundo Apellido) -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
		<div>
			<label for="apellido" class={labelClass}
				>Apellido <span class="text-red-500 ml-0.5">*</span></label
			>
			<input
				id="apellido"
				name="apellido"
				type="text"
				bind:value={$form.apellido}
				oninput={() => validate('apellido')}
				disabled={loading || readonly}
				class={getInputClass(!!$errors.apellido, readonly, $form.apellido, true)}
				placeholder="Pérez"
				{...$constraints.apellido}
			/>
			{#if $errors.apellido}<p class={errorClass}>{$errors.apellido}</p>{/if}
		</div>

		<div>
			<label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
			<input
				id="segundoApellido"
				name="segundoApellido"
				type="text"
				bind:value={$form.segundoApellido}
				oninput={() => validate('segundoApellido')}
				disabled={loading || readonly}
				class={getInputClass(
					!!$errors.segundoApellido,
					readonly,
					$form.segundoApellido,
					false // isRequired
				)}
				placeholder="González"
				{...$constraints.segundoApellido}
			/>
			{#if $errors.segundoApellido}<p class={errorClass}>
					{$errors.segundoApellido}
				</p>{/if}
		</div>
	</div>

	<!-- Fila 4: Empresa (siempre visible si showEmpresa) -->
	{#if showEmpresa}
		<div class="mb-4">
			<label for="empresaId" class={labelClass}>
				Empresa <span class="text-red-500 ml-0.5">*</span>
			</label>
			<div class="flex gap-2 relative">
				<div class="relative flex-1">
					<button
						type="button"
						id="empresaId"
						disabled={loading || readonly || empresas.length === 0}
						onclick={() => (showEmpresaDropdown = !showEmpresaDropdown)}
						class="{getInputClass(
							!!$errors.empresaId,
							readonly,
							$form.empresaId,
							true // isRequired
						)} flex items-center justify-between cursor-pointer w-full text-left {showEmpresaDropdown
							? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
							: ''}"
					>
						<span class="truncate">
							{empresas.find((e) => e.id === $form.empresaId)?.nombre || 'Seleccione una empresa'}
						</span>
						<ChevronDown size={16} class="text-secondary" />
					</button>

					<!-- Dropdown Options -->
					{#if showEmpresaDropdown && !readonly}
						<!-- Backdrop -->
						<div
							class="fixed inset-0 z-[60]"
							onclick={() => (showEmpresaDropdown = false)}
							role="presentation"
							aria-hidden="true"
						></div>

						<div
							class="absolute z-[70] w-full mt-1 bg-surface-2 border border-surface rounded-lg shadow-xl overflow-hidden p-1 origin-top max-h-60 overflow-y-auto"
						>
							{#if empresas.length === 0}
								<div class="px-3 py-2 text-sm text-gray-500">No hay empresas</div>
							{:else}
								{#each empresas as emp}
									<button
										type="button"
										onclick={() => {
											$form.empresaId = emp.id;
											showEmpresaDropdown = false;
										}}
										class="w-full text-left px-3 py-1.5 text-sm rounded-md transition-colors flex items-center justify-between group {$form.empresaId ===
										emp.id
											? 'bg-white/5 text-white font-medium'
											: 'text-secondary hover:bg-white/10'}"
									>
										<span>{emp.nombre}</span>
										{#if $form.empresaId === emp.id}
											<svg
												xmlns="http://www.w3.org/2000/svg"
												class="h-4 w-4 text-white"
												viewBox="0 0 20 20"
												fill="currentColor"
											>
												<path
													fill-rule="evenodd"
													d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
													clip-rule="evenodd"
												/>
											</svg>
										{/if}
									</button>
								{/each}
							{/if}
						</div>
					{/if}
				</div>
				{#if onCreateEmpresa && !readonly}
					<button
						type="button"
						onclick={onCreateEmpresa}
						disabled={loading}
						class="px-2 h-[34px] rounded-lg border-2 border-surface text-secondary hover:border-accent hover:text-accent transition-all flex items-center justify-center text-xs disabled:opacity-50"
						title="Crear nueva empresa"
					>
						<Plus size={16} />
					</button>
				{/if}
			</div>
			{#if $errors.empresaId}<p class={errorClass}>{$errors.empresaId}</p>{/if}
		</div>
	{/if}
</div>

<style>
	/* Standardized input focus style */
	input:focus,
	button#empresaId:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}

	/* Autofill Fix for Dark Theme */
	input:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}
</style>
