<script lang="ts">
  import { fade, fly } from "svelte/transition";

  interface Props {
    show: boolean;
    gafeteNumero: string;
    nombrePersona: string;
    fechaReporte: string;
    onResolve: (notas: string, fechaDevolucion: string) => void;
    onCancel: () => void;
  }

  let {
    show,
    gafeteNumero,
    nombrePersona,
    fechaReporte,
    onResolve,
    onCancel,
  }: Props = $props();

  let notas = $state("");
  let fechaDevolucion = $state(new Date().toISOString().split("T")[0]);
</script>

{#if show}
  <div
    class="modal-overlay fixed inset-0 z-50 flex items-center justify-center"
    transition:fade
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      onclick={onCancel}
    ></div>
    <div
      class="card-base max-w-md w-full p-6 relative z-10"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <h3 class="text-lg font-semibold text-primary mb-2">
        Resolver Alerta de Gafete
      </h3>
      <p class="text-sm text-secondary mb-6">
        Registre la devolución o pago del gafete <strong
          class="font-mono text-accent">{gafeteNumero}</strong
        >
        que fue perdido por
        <strong>{nombrePersona}</strong> el {new Date(
          fechaReporte,
        ).toLocaleDateString("es-ES")}.
      </p>

      <div class="space-y-4">
        <div class="space-y-1.5">
          <label for="fechaDevolucion" class="text-sm font-medium text-primary">
            Fecha de Devolución/Pago <span class="text-error">*</span>
          </label>
          <input
            id="fechaDevolucion"
            type="date"
            bind:value={fechaDevolucion}
            max={new Date().toISOString().split("T")[0]}
            class="input-base w-full"
          />
        </div>

        <div class="space-y-1.5">
          <label for="notasResolucion" class="text-sm font-medium text-primary">
            Notas/Observaciones
          </label>
          <textarea
            id="notasResolucion"
            bind:value={notas}
            rows="3"
            placeholder="Detalles sobre la devolución o pago (ej: Devolvió gafete en buen estado, Pagó $X por gafete perdido, etc.)"
            class="input-base w-full resize-y"
          ></textarea>
        </div>
      </div>

      <div class="mt-6 flex justify-end gap-3">
        <button type="button" onclick={onCancel} class="btn-ghost btn-base">
          Cancelar
        </button>
        <button
          type="button"
          onclick={() => onResolve(notas, fechaDevolucion)}
          class="btn-base bg-success text-white hover:bg-success-hover"
        >
          Marcar como Resuelto
        </button>
      </div>
    </div>
  </div>
{/if}
