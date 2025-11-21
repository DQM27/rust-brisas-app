<script lang="ts">
  interface Props {
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

    onSubmit({
      cedula,
      nombre,
      apellido,
      empresaId,
      fechaVencimientoPraind,
    });
  }

  // Reset desde el padre si lo necesitan
  export function reset() {
    cedula = "";
    nombre = "";
    apellido = "";
    empresaId = "";
    fechaVencimientoPraind = "";
  }
</script>

<form
  onsubmit={handleSubmit}
  class="flex w-full max-w-md flex-col gap-4 rounded-lg bg-[#252526] p-8 shadow-xl"
>
  <h1 class="mb-2 text-center text-2xl font-semibold text-gray-200">
    Registrar Contratista
  </h1>

  <!-- CÉDULA -->
  <div class="flex flex-col gap-1">
    <label class="text-sm font-medium text-gray-300">Cédula</label>
    <input
      bind:value={cedula}
      placeholder="1-2345-6789"
      disabled={loading}
      required
      class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200
             placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
             focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
    />
  </div>

  <!-- NOMBRE -->
  <div class="flex flex-col gap-1">
    <label class="text-sm font-medium text-gray-300">Nombre</label>
    <input
      bind:value={nombre}
      placeholder="Juan"
      disabled={loading}
      required
      class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200
             placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
             focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
    />
  </div>

  <!-- APELLIDO -->
  <div class="flex flex-col gap-1">
    <label class="text-sm font-medium text-gray-300">Apellido</label>
    <input
      bind:value={apellido}
      placeholder="Pérez"
      disabled={loading}
      required
      class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200
             placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
             focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
    />
  </div>

  <!-- EMPRESA -->
  <div class="flex flex-col gap-1">
    <label class="text-sm font-medium text-gray-300">Empresa</label>
    <slot name="empresas" />
    <!-- 
      Aquí NO cargamos empresas.
      El padre provee el <select> usando un <slot name="empresas"> 
    -->
  </div>

  <!-- FECHA PRAIND -->
  <div class="flex flex-col gap-1">
    <label class="text-sm font-medium text-gray-300">Fecha PRAIND</label>
    <input
      type="date"
      bind:value={fechaVencimientoPraind}
      disabled={loading}
      required
      class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200
             placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
             focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
    />
  </div>

  <button
    type="submit"
    disabled={loading}
    class="mt-2 rounded bg-[#007acc] px-4 py-2.5 font-medium text-white 
           transition-colors hover:bg-[#005a9e] disabled:cursor-not-allowed 
           disabled:opacity-60"
  >
    {loading ? "Procesando..." : "Registrar"}
  </button>
</form>
