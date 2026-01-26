<!-- BlacklistManualInputs.svelte -->
<script lang="ts">
	interface Props {
		cedula: string;
		nombre: string;
		segundoNombre: string;
		apellido: string;
		segundoApellido: string;
		loading?: boolean;
		onCedulaChange: (value: string) => void;
		onNombreChange: (value: string) => void;
		onSegundoNombreChange: (value: string) => void;
		onApellidoChange: (value: string) => void;
		onSegundoApellidoChange: (value: string) => void;
	}

	let {
		cedula,
		nombre,
		segundoNombre,
		apellido,
		segundoApellido,
		loading = false,
		onCedulaChange,
		onNombreChange,
		onSegundoNombreChange,
		onApellidoChange,
		onSegundoApellidoChange
	}: Props = $props();
</script>

<div class="space-y-5">
	<div class="space-y-1.5">
		<label for="cedula" class="text-sm font-medium text-primary">Cédula</label>
		<input
			id="cedula"
			type="text"
			value={cedula}
			oninput={(e) => onCedulaChange(e.currentTarget.value)}
			placeholder="1-2345-6789"
			disabled={loading}
			class="input-base w-full"
		/>
	</div>

	<div class="grid grid-cols-2 gap-4">
		<div class="space-y-1.5">
			<label for="nombre" class="text-sm font-medium text-primary">Primer Nombre</label>
			<input
				id="nombre"
				type="text"
				value={nombre}
				oninput={(e) => onNombreChange(e.currentTarget.value)}
				placeholder="Juan"
				disabled={loading}
				class="input-base w-full"
			/>
		</div>
		<div class="space-y-1.5">
			<label for="segundoNombre" class="text-sm font-medium text-primary">Segundo Nombre</label>
			<input
				id="segundoNombre"
				type="text"
				value={segundoNombre}
				oninput={(e) => onSegundoNombreChange(e.currentTarget.value)}
				placeholder="(Opcional)"
				disabled={loading}
				class="input-base w-full"
			/>
		</div>
		<div class="space-y-1.5">
			<label for="apellido" class="text-sm font-medium text-primary text-nowrap"
				>Primer Apellido</label
			>
			<input
				id="apellido"
				type="text"
				value={apellido}
				oninput={(e) => onApellidoChange(e.currentTarget.value)}
				placeholder="Pérez"
				disabled={loading}
				class="input-base w-full"
			/>
		</div>
		<div class="space-y-1.5">
			<label for="segundoApellido" class="text-sm font-medium text-primary text-nowrap"
				>Segundo Apellido</label
			>
			<input
				id="segundoApellido"
				type="text"
				value={segundoApellido}
				oninput={(e) => onSegundoApellidoChange(e.currentTarget.value)}
				placeholder="(Opcional)"
				disabled={loading}
				class="input-base w-full"
			/>
		</div>
	</div>
</div>
