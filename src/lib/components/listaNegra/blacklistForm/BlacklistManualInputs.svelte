<!-- BlacklistManualInputs.svelte -->
<script lang="ts">
  interface Props {
    cedula: string;
    nombre: string;
    apellido: string;
    loading?: boolean;
    onCedulaChange: (value: string) => void;
    onNombreChange: (value: string) => void;
    onApellidoChange: (value: string) => void;
  }

  let {
    cedula,
    nombre,
    apellido,
    loading = false,
    onCedulaChange,
    onNombreChange,
    onApellidoChange,
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
      <label for="nombre" class="text-sm font-medium text-primary">Nombre</label
      >
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
      <label for="apellido" class="text-sm font-medium text-primary"
        >Apellido</label
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
  </div>
</div>
