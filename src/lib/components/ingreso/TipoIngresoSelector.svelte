<script lang="ts">
  // ==========================================
  // TipoIngresoSelector.svelte
  // ==========================================
  // Selector de tipo de ingreso (Contratista/Visita/Proveedor)

  import { UserCircle, Users, Building2 } from 'lucide-svelte';

  export let tipoSeleccionado: 'contratista' | 'visita' | 'proveedor' = 'contratista';
  export let onChange: (tipo: 'contratista' | 'visita' | 'proveedor') => void = () => {};

  const tipos = [
    {
      id: 'contratista' as const,
      label: 'Contratista',
      icon: UserCircle,
      descripcion: 'Trabajador con PRAIND'
    },
    {
      id: 'visita' as const,
      label: 'Visita',
      icon: Users,
      descripcion: 'Visitante con anfitrión'
    },
    {
      id: 'proveedor' as const,
      label: 'Proveedor',
      icon: Building2,
      descripcion: 'Proveedor de empresa'
    }
  ];

  function handleSelect(tipo: typeof tipoSeleccionado) {
    tipoSeleccionado = tipo;
    onChange(tipo);
  }
</script>

<div class="tipo-ingreso-selector">
  <label class="label">Tipo de Ingreso</label>
  <div class="grid grid-cols-3 gap-3">
    {#each tipos as tipo}
      <button
        type="button"
        class="tipo-card"
        class:active={tipoSeleccionado === tipo.id}
        on:click={() => handleSelect(tipo.id)}
      >
        <svelte:component this={tipo.icon} size={24} class="icon" />
        <div class="tipo-info">
          <span class="tipo-label">{tipo.label}</span>
          <span class="tipo-desc">{tipo.descripcion}</span>
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .tipo-ingreso-selector {
    margin-bottom: 1.5rem;
  }

  .label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
  }

  .tipo-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    border: 2px solid var(--border);
    border-radius: 8px;
    background: var(--bg-primary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tipo-card:hover {
    border-color: var(--primary);
    background: var(--bg-secondary);
  }

  .tipo-card.active {
    border-color: var(--primary);
    background: var(--primary-alpha-10);
  }

  .tipo-card :global(.icon) {
    color: var(--text-tertiary);
    transition: color 0.2s ease;
  }

  .tipo-card.active :global(.icon) {
    color: var(--primary);
  }

  .tipo-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
  }

  .tipo-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .tipo-desc {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    text-align: center;
  }
</style>
