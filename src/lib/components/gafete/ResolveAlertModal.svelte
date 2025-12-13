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

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[#2da44e] disabled:opacity-60 transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
    transition:fade
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="absolute inset-0" onclick={onCancel}></div>
    <div
      class="bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 w-full max-w-md p-6 relative z-10"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <h3
        class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2 border-b border-gray-200 dark:border-gray-700 pb-2"
      >
        Resolver Alerta de Gafete
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-6">
        Registre la devolución o pago del gafete <strong
          class="font-mono text-gray-900 dark:text-gray-100 bg-gray-100 dark:bg-gray-800 px-1.5 py-0.5 rounded"
          >{gafeteNumero}</strong
        >
        que fue perdido por
        <strong class="text-gray-900 dark:text-gray-100">{nombrePersona}</strong
        >
        el {new Date(fechaReporte).toLocaleDateString("es-ES")}.
      </p>

      <div class="space-y-4">
        <div>
          <label for="fechaDevolucion" class={labelClass}>
            Fecha de Devolución/Pago <span class="text-red-500">*</span>
          </label>
          <input
            id="fechaDevolucion"
            type="date"
            bind:value={fechaDevolucion}
            max={new Date().toISOString().split("T")[0]}
            class={inputClass}
          />
        </div>

        <div>
          <label for="notasResolucion" class={labelClass}>
            Notas/Observaciones
          </label>
          <textarea
            id="notasResolucion"
            bind:value={notas}
            rows="3"
            placeholder="Detalles sobre la devolución o pago (ej: Devolvió gafete en buen estado, Pagó $X por gafete perdido, etc.)"
            class={inputClass}
          ></textarea>
        </div>
      </div>

      <div
        class="mt-6 flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
      >
        <button
          type="button"
          onclick={onCancel}
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 border border-transparent rounded-md transition-colors"
        >
          Cancelar
        </button>
        <button
          type="button"
          onclick={() => onResolve(notas, fechaDevolucion)}
          class="px-4 py-2 text-sm font-medium text-white bg-[#2da44e] border border-transparent rounded-md shadow-sm hover:bg-[#2c974b] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#2da44e] transition-all"
        >
          Marcar como Resuelto
        </button>
      </div>
    </div>
  </div>
{/if}
