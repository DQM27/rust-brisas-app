<script lang="ts">
  export interface Props {
    loading?: boolean;
    onSubmit: (data: {
      cedula: string;
      nombre: string;
      apellido: string;
      empresaId: string;
      fechaVencimientoPraind: string;
    }) => void;
  }

  let { loading = false, onSubmit }: Props = $props();

  let cedula = $state("");
  let nombre = $state("");
  let apellido = $state("");
  let empresaId = $state("");
  let fechaVencimientoPraind = $state("");

  function handleSubmit(event: Event) {
    event.preventDefault();
    onSubmit({ cedula, nombre, apellido, empresaId, fechaVencimientoPraind });
  }

  export function reset() {
    cedula = "";
    nombre = "";
    apellido = "";
    empresaId = "";
    fechaVencimientoPraind = "";
  }
</script>

<form
  on:submit={handleSubmit}
  class="w-full max-w-lg mx-auto mt-8 p-6 bg-gray-800 rounded-lg shadow-lg flex flex-col gap-4"
>
  <h2 class="text-xl font-semibold text-white text-center mb-4">Registrar Contratista</h2>

  <div class="flex flex-col gap-1">
    <label for="cedula" class="text-sm font-medium text-gray-300">Cédula</label>
    <input
      id="cedula"
      type="text"
      bind:value={cedula}
      placeholder="1-2345-6789"
      disabled={loading}
      required
      class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="nombre" class="text-sm font-medium text-gray-300">Nombre</label>
    <input
      id="nombre"
      type="text"
      bind:value={nombre}
      placeholder="Juan"
      disabled={loading}
      required
      class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="apellido" class="text-sm font-medium text-gray-300">Apellido</label>
    <input
      id="apellido"
      type="text"
      bind:value={apellido}
      placeholder="Pérez"
      disabled={loading}
      required
      class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="empresa" class="text-sm font-medium text-gray-300">Empresa</label>
    <input
      id="empresa"
      type="text"
      bind:value={empresaId}
      placeholder="ID Empresa"
      disabled={loading}
      required
      class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <div class="flex flex-col gap-1">
    <label for="fecha" class="text-sm font-medium text-gray-300">Fecha PRAIND</label>
    <input
      id="fecha"
      type="date"
      bind:value={fechaVencimientoPraind}
      disabled={loading}
      required
      class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
  </div>

  <button
    type="submit"
    disabled={loading}
    class="mt-4 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
  >
    {loading ? "Procesando..." : "Registrar"}
  </button>
</form>
