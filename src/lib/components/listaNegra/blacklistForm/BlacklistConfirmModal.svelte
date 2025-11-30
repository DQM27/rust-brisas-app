<!-- BlacklistConfirmModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";

  interface Props {
    show: boolean;
    contratistaName: string;
    motivo: string;
    observaciones: string;
    onConfirm: () => void;
    onCancel: () => void;
    onMotivoChange: (value: string) => void;
    onObservacionesChange: (value: string) => void;
  }

  let {
    show,
    contratistaName,
    motivo,
    observaciones,
    onConfirm,
    onCancel,
    onMotivoChange,
    onObservacionesChange,
  }: Props = $props();
</script>

{#if show}
  <div
    class="modal-overlay fixed inset-0 z-50 flex items-center justify-center"
    transition:fade
  >
    <div
      class="card-base max-w-sm w-full p-6"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <h3 class="text-lg font-semibold text-primary mb-4">
        Confirmar Desbloqueo
      </h3>
      <p class="text-sm text-secondary mb-6">
        Está a punto de desactivar el bloqueo para <strong
          >{contratistaName}</strong
        >. El motivo y las observaciones para esta acción serán registradas en
        el historial de la persona.
      </p>

      <div class="space-y-4">
        <div class="space-y-1.5">
          <label for="unblockMotivo" class="text-sm font-medium text-primary">
            Motivo del Desbloqueo <span class="text-error">*</span>
          </label>
          <textarea
            id="unblockMotivo"
            value={motivo}
            oninput={(e) => onMotivoChange(e.currentTarget.value)}
            rows="2"
            placeholder="Motivo de la desactivación del bloqueo (ej: Cumplió sanción, revisión de caso, etc.)"
            class="input-base w-full resize-y"
          ></textarea>
        </div>
        <div class="space-y-1.5">
          <label
            for="unblockObservaciones"
            class="text-sm font-medium text-primary"
          >
            Observaciones (Opcional)
          </label>
          <textarea
            id="unblockObservaciones"
            value={observaciones}
            oninput={(e) => onObservacionesChange(e.currentTarget.value)}
            rows="2"
            placeholder="Notas internas."
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
          onclick={onConfirm}
          disabled={!motivo.trim()}
          class="btn-base bg-success text-white hover:bg-success-hover disabled:opacity-50"
        >
          Confirmar Desbloqueo
        </button>
      </div>
    </div>
  </div>
{/if}
