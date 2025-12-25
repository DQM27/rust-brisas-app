<!-- src/lib/components/gafete/modals/ResolveAlertModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { CheckCircle, X, AlertCircle, Calendar, User } from "lucide-svelte";

  interface Props {
    show: boolean;
    gafeteNumero: string;
    nombrePersona: string;
    fechaReporte: string;
    loading?: boolean;
    onResolve: (notas: string) => void;
    onCancel: () => void;
  }

  let {
    show,
    gafeteNumero,
    nombrePersona,
    fechaReporte,
    loading = false,
    onResolve,
    onCancel,
  }: Props = $props();

  let notas = $state("");

  // Estilos
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-green-500 disabled:opacity-60 transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
  >
    <div class="absolute inset-0"></div>

    <div
      class="relative z-10 w-full max-w-md bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
      transition:fly={{ y: 20, duration: 250 }}
    >
      <!-- Header -->
      <div
        class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between bg-white dark:bg-[#0d1117]"
      >
        <h2
          class="text-xl font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2"
        >
          <CheckCircle size={22} class="text-green-500" />
          Resolver Alerta
        </h2>
        <button
          onclick={onCancel}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <div class="p-6 space-y-6">
        <!-- Context Card -->
        <div
          class="p-4 bg-amber-50 dark:bg-amber-900/10 rounded-lg border border-amber-100 dark:border-amber-900/30"
        >
          <div class="flex items-start gap-3">
            <AlertCircle size={18} class="text-amber-500 mt-0.5 shrink-0" />
            <div class="space-y-2">
              <p class="text-sm text-gray-700 dark:text-gray-300 leading-tight">
                Se está resolviendo el reporte de pérdida para el gafete <strong
                  class="text-gray-900 dark:text-white">{gafeteNumero}</strong
                >.
              </p>

              <div class="grid grid-cols-1 gap-2 pt-1">
                <div
                  class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400"
                >
                  <User size={14} />
                  <span
                    >Persona: <strong class="text-gray-700 dark:text-gray-300"
                      >{nombrePersona}</strong
                    ></span
                  >
                </div>
                <div
                  class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400"
                >
                  <Calendar size={14} />
                  <span
                    >Reportado el: <strong
                      class="text-gray-700 dark:text-gray-300"
                      >{new Date(fechaReporte).toLocaleDateString()}</strong
                    ></span
                  >
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Notas Form -->
        <div class="space-y-4">
          <div>
            <label for="notasResolucion" class={labelClass}>
              Notas de Resolución <span class="text-gray-400 font-normal"
                >(Obligatorio)</span
              >
            </label>
            <textarea
              id="notasResolucion"
              bind:value={notas}
              rows="3"
              disabled={loading}
              placeholder="Ej: El visitante devolvió el gafete después de encontrarlo en su auto / Se procedió con el cobro por extravío."
              class={inputClass}
            ></textarea>
            <p class="mt-1 text-xs text-gray-400">
              Describa brevemente cómo se resolvió la pérdida (devolución
              física, pago, reposición, etc).
            </p>
          </div>
        </div>

        <!-- Botones -->
        <div class="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onclick={onCancel}
            disabled={loading}
            class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 border border-transparent rounded-md transition-colors"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={() => onResolve(notas)}
            disabled={loading || !notas.trim()}
            class="inline-flex justify-center px-6 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md shadow-sm hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            {#if loading}
              <svg
                class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                ></circle>
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
              Resolviendo...
            {:else}
              Marcar como Resuelto
            {/if}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
