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
    onSubmit({ cedula, nombre, apellido, empresaId, fechaVencimientoPraind });
  }
</script>

<form onsubmit={handleSubmit} class="flex flex-col gap-4">
  
  <label>
    Cédula
    <input bind:value={cedula} required />
  </label>

  <label>
    Nombre
    <input bind:value={nombre} required />
  </label>

  <label>
    Apellido
    <input bind:value={apellido} required />
  </label>

  <label>
    Empresa
    <select bind:value={empresaId} required>
      
    </select>
  </label>

  <label>
    Fecha PRAIND
    <input type="date" bind:value={fechaVencimientoPraind} required />
  </label>

  <button type="submit" disabled={loading}>
    {loading ? "Procesando..." : "Registrar"}
  </button>
</form>
