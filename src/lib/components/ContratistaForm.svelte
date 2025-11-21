<script lang="ts">
  export let loading = false;
  export let onSubmit: (data: {
    cedula: string;
    nombre: string;
    apellido: string;
    empresaId: string;
    fechaVencimientoPraind: string;
  }) => void;

  // Lista de empresas pasada desde el padre
  export let empresas: { id: string; nombre: string }[] = [];

  let cedula = "";
  let nombre = "";
  let apellido = "";
  let empresaId = "";
  let fechaVencimientoPraind = "";

  // Método que el padre puede llamar
  export function reset() {
    cedula = "";
    nombre = "";
    apellido = "";
    empresaId = "";
    fechaVencimientoPraind = "";
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    onSubmit({ cedula, nombre, apellido, empresaId, fechaVencimientoPraind });
  }

  const isFormValid = () =>
    cedula.trim() !== "" &&
    nombre.trim() !== "" &&
    apellido.trim() !== "" &&
    empresaId.trim() !== "" &&
    fechaVencimientoPraind.trim() !== "";
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div class="w-full max-w-2xl rounded-lg bg-[#252526] p-8 shadow-xl">
    <h2 class="border-b border-[#007acc] pb-3 text-2xl font-semibold text-gray-200 text-center">
      Registrar Contratista
    </h2>

    <form on:submit={handleSubmit} class="mt-6 space-y-5">
      <!-- Cédula -->
      <div class="space-y-2">
        <label for="cedula" class="block text-sm font-medium text-gray-300">Cédula</label>
        <input
          id="cedula"
          type="text"
          bind:value={cedula}
          placeholder="1-2345-6789"
          disabled={loading}
          required
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white
                 placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        />
      </div>

      <!-- Nombre y Apellido -->
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div class="space-y-2">
          <label for="nombre" class="block text-sm font-medium text-gray-300">Nombre</label>
          <input
            id="nombre"
            type="text"
            bind:value={nombre}
            placeholder="Juan"
            disabled={loading}
            required
            class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white
                   placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
          />
        </div>

        <div class="space-y-2">
          <label for="apellido" class="block text-sm font-medium text-gray-300">Apellido</label>
          <input
            id="apellido"
            type="text"
            bind:value={apellido}
            placeholder="Pérez"
            disabled={loading}
            required
            class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white
                   placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
          />
        </div>
      </div>

      <!-- Empresa (select) -->
      <div class="space-y-2">
        <label for="empresaId" class="block text-sm font-medium text-gray-300">Empresa</label>
        <select
          id="empresaId"
          bind:value={empresaId}
          disabled={loading}
          required
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white
                 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        >
          <option value="" disabled selected>Seleccione una empresa</option>
          {#each empresas as empresa}
            <option value={empresa.id}>{empresa.nombre}</option>
          {/each}
        </select>
      </div>

      <!-- Fecha PRAIND -->
      <div class="space-y-2">
        <label for="fechaVencimientoPraind" class="block text-sm font-medium text-gray-300">Fecha PRAIND</label>
        <input
          id="fechaVencimientoPraind"
          type="date"
          bind:value={fechaVencimientoPraind}
          disabled={loading}
          required
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white
                 placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        />
      </div>

      <!-- Botón -->
      <button
        type="submit"
        disabled={loading || !isFormValid()}
        class="mt-6 w-full rounded bg-[#007acc] px-4 py-2.5 font-medium text-white
               transition-colors hover:bg-[#005a9e] disabled:cursor-not-allowed disabled:opacity-60"
      >
        {loading ? 'Procesando...' : 'Registrar Contratista'}
      </button>
    </form>
  </div>
</div>
