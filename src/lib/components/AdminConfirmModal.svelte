<script lang="ts">
  import { preventDefault } from "svelte/legacy";

  interface Props {
    isOpen: boolean;
    onConfirm: (password: string) => void;
    onCancel: () => void;
  }

  let { isOpen, onConfirm, onCancel }: Props = $props();

  let password = $state("");
  let inputRef = $state<HTMLInputElement>();

  $effect(() => {
    if (isOpen && inputRef) {
      inputRef.focus();
    }
  });

  function handleSubmit() {
    if (password) {
      onConfirm(password);
      password = ""; // Reset
    }
  }

  function handleCancel() {
    password = "";
    onCancel();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
  >
    <div
      class="w-full max-w-md bg-surface-2 rounded-lg shadow-xl border border-surface-tertiary p-6 animate-scale-in"
    >
      <h3 class="text-xl font-bold text-primary mb-2">Confirmar Acción</h3>
      <p class="text-tertiary text-sm mb-6">
        Por favor ingresa tu contraseña de administrador para continuar.
      </p>

      <form onsubmit={preventDefault(handleSubmit)} class="space-y-4">
        <div>
          <label
            for="admin-pass"
            class="block text-sm font-medium text-primary mb-1"
            >Contraseña</label
          >
          <input
            bind:this={inputRef}
            id="admin-pass"
            type="password"
            bind:value={password}
            class="w-full rounded border border-emphasis bg-surface-1 px-3 py-2 text-primary focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent"
            placeholder="Tu contraseña..."
          />
        </div>

        <div class="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onclick={handleCancel}
            class="px-4 py-2 text-sm font-medium text-tertiary hover:text-primary transition-colors"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={!password}
            class="px-4 py-2 text-sm font-medium text-white bg-accent rounded hover:bg-accent-hover transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Confirmar
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  @keyframes scale-in {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  .animate-scale-in {
    animation: scale-in 0.2s ease-out forwards;
  }
</style>
