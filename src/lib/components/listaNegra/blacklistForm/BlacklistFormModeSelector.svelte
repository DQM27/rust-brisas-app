<!-- BlacklistFormModeSelector.svelte -->
<script lang="ts">
  interface Props {
    modoRegistro: "existente" | "manual";
    loading?: boolean;
    onModeChange: (mode: "existente" | "manual") => void;
  }

  let { modoRegistro, loading = false, onModeChange }: Props = $props();
</script>

<div class="mb-6 space-y-3">
  <span class="block text-sm font-medium text-primary">Modo de Registro</span>
  <div class="grid grid-cols-2 gap-3">
    <button
      type="button"
      onclick={() => onModeChange("existente")}
      disabled={loading}
      class="btn-base {modoRegistro === 'existente'
        ? 'border-error bg-error text-error'
        : 'btn-secondary'}"
    >
      <svg
        class="h-5 w-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
        />
      </svg>
      <span>Contratista Existente</span>
    </button>
    <button
      type="button"
      onclick={() => onModeChange("manual")}
      disabled={loading}
      class="btn-base {modoRegistro === 'manual'
        ? 'border-error bg-error text-error'
        : 'btn-secondary'}"
    >
      <svg
        class="h-5 w-5"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
        />
      </svg>
      <span>Registro Manual</span>
    </button>
  </div>
</div>
