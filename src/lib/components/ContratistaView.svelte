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

  // --- Campos del formulario ---
  let cedula = "";
  let nombre = "";
  let apellido = "";
  let empresaId = "";
  let fechaVencimientoPraind = "";

  // Permite resetear el formulario desde el padre
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
</script>

<form on:submit={handleSubmit} class="space-y-2">
  <input bind:value={cedula} placeholder="Cédula" />
  <input bind:value={nombre} placeholder="Nombre" />
  <input bind:value={apellido} placeholder="Apellido" />
  <select bind:value={empresaId}>
    <option value="" disabled selected>Seleccione una empresa</option>
    {#each empresas as empresa}
      <option value={empresa.id}>{empresa.nombre}</option>
    {/each}
  </select>
  <input bind:value={fechaVencimientoPraind} type="date" placeholder="Fecha Vencimiento Praind" />

  <button type="submit" disabled={loading}>
    {loading ? "Cargando..." : "Registrar"}
  </button>
</form>
