<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import { citaService } from "$lib/services/citaService";
  import type { CitaPopulated } from "$lib/types/cita";
  import { toast } from "svelte-5-french-toast";
  import { Clock, User, Building, ArrowRight, UserCheck } from "lucide-svelte";
  import { currentUser } from "$lib/stores/auth";

  const dispatch = createEventDispatcher();

  export let refreshTrigger = 0; // Prop para forzar recarga

  let citas: CitaPopulated[] = [];
  let loading = false;
  let processingId: string | null = null;
  let gafeteInput = "";
  let selectedCita: CitaPopulated | null = null;

  $: if (refreshTrigger) loadCitas();

  onMount(() => {
    loadCitas();
  });

  async function loadCitas() {
    loading = true;
    try {
      citas = await citaService.getCitasHoy();
    } catch (e) {
      console.error(e);
      // Silent error or toast?
    } finally {
      loading = false;
    }
  }

  function handleSelectCita(cita: CitaPopulated) {
    selectedCita = cita;
    gafeteInput = "";
    // Focus next tick?
  }

  function handleCancelSelection() {
    selectedCita = null;
    gafeteInput = "";
  }

  async function handleProcesarIngreso() {
    if (!selectedCita || !gafeteInput) return;
    if (!$currentUser) return;

    processingId = selectedCita.id;
    try {
      await citaService.procesarIngresoCita(
        selectedCita.id,
        gafeteInput,
        $currentUser.id,
      );
      toast.success(`Ingreso registrado para ${selectedCita.visitante_nombre}`);
      selectedCita = null;
      loadCitas(); // Recargar lista
      dispatch("ingresoRealizado"); // Avisar al padre para recargar lista de ingresos
    } catch (e: any) {
      toast.error(e.message || "Error al procesar ingreso");
    } finally {
      processingId = null;
    }
  }
</script>

<div class="h-full flex flex-col">
  {#if selectedCita}
    <!-- Modal/Vista de Confirmación Rápida -->
    <div
      class="flex-1 flex flex-col items-center justify-center p-6 bg-gray-50 dark:bg-black/20 rounded-lg animate-in fade-in zoom-in-95 duration-200"
    >
      <div class="w-full max-w-md space-y-6">
        <div class="text-center space-y-2">
          <div
            class="mx-auto w-16 h-16 bg-green-500/10 rounded-full flex items-center justify-center mb-4"
          >
            <UserCheck class="w-8 h-8 text-green-500" />
          </div>
          <h3 class="text-xl font-bold text-gray-900 dark:text-gray-100">
            Confirmar Ingreso
          </h3>
          <p class="text-gray-500 dark:text-gray-400">
            {selectedCita.visitante_nombre_completo}
          </p>
          <div
            class="text-sm bg-white dark:bg-[#252526] p-3 rounded border border-gray-200 dark:border-white/10 mt-2"
          >
            <div class="flex justify-between">
              <span class="text-gray-500">Visita a:</span>
              <span class="font-medium">{selectedCita.anfitrion}</span>
            </div>
            <div class="flex justify-between mt-1">
              <span class="text-gray-500">Hora Cita:</span>
              <span class="font-medium text-blue-500"
                >{new Date(selectedCita.fecha_cita).toLocaleTimeString([], {
                  hour: "2-digit",
                  minute: "2-digit",
                })}</span
              >
            </div>
          </div>
        </div>

        <div class="space-y-4">
          <div class="space-y-2">
            <label
              for="gafeteInput"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            >
              Asignar Gafete #
            </label>
            <input
              id="gafeteInput"
              type="text"
              bind:value={gafeteInput}
              placeholder="Escanee o escriba gafete"
              class="w-full text-center text-2xl font-mono p-3 rounded-lg border-2 border-primary/50 focus:border-primary focus:ring-4 focus:ring-primary/20 bg-white dark:bg-[#1e1e1e]"
            />
          </div>

          <div class="grid grid-cols-2 gap-3">
            <button
              class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-white/5"
              on:click={handleCancelSelection}
            >
              Cancelar
            </button>
            <button
              class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded font-medium disabled:opacity-50 disabled:cursor-not-allowed"
              disabled={!gafeteInput || !!processingId}
              on:click={handleProcesarIngreso}
            >
              {processingId ? "Procesando..." : "Confirmar Ingreso"}
            </button>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- Lista de Citas -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-gray-700 dark:text-gray-200">
        Citas Programadas para Hoy
      </h3>
      <button
        class="p-1 hover:bg-gray-100 dark:hover:bg-white/5 rounded"
        on:click={loadCitas}
        title="Actualizar"
      >
        🔄
      </button>
    </div>

    {#if loading}
      <div class="p-8 text-center text-gray-500">Cargando citas...</div>
    {:else if citas.length === 0}
      <div
        class="flex flex-col items-center justify-center p-8 text-gray-400 border-2 border-dashed border-gray-200 dark:border-white/5 rounded-lg"
      >
        <Clock size={32} class="mb-2 opacity-50" />
        <p>No hay citas pendientes para hoy</p>
      </div>
    {:else}
      <div class="space-y-3 overflow-y-auto pr-1">
        {#each citas as cita}
          <button
            class="w-full text-left bg-white dark:bg-[#1e1e1e] border border-gray-200 dark:border-white/5 hover:border-blue-500 dark:hover:border-blue-500/50 p-3 rounded-lg shadow-sm transition-all duration-200 group relative overflow-hidden"
            on:click={() => handleSelectCita(cita)}
          >
            <div
              class="absolute left-0 top-0 bottom-0 w-1 bg-blue-500 group-hover:w-1.5 transition-all"
            ></div>

            <div class="pl-3 flex justify-between items-center">
              <div>
                <div class="flex items-center gap-2 mb-1">
                  <span class="font-bold text-gray-900 dark:text-white text-lg"
                    >{cita.visitante_nombre_completo}</span
                  >
                  {#if cita.visitante_empresa}
                    <span
                      class="text-xs px-2 py-0.5 bg-gray-100 dark:bg-white/10 rounded text-gray-500"
                      >{cita.visitante_empresa}</span
                    >
                  {/if}
                </div>
                <div
                  class="flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400"
                >
                  <span class="flex items-center gap-1"
                    ><Clock size={14} />
                    {new Date(cita.fecha_cita).toLocaleTimeString([], {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}</span
                  >
                  <span class="flex items-center gap-1"
                    ><User size={14} /> Visita a: {cita.anfitrion}</span
                  >
                </div>
              </div>
              <div
                class="text-blue-500 opacity-0 group-hover:opacity-100 transition-opacity transform translate-x-2 group-hover:translate-x-0"
              >
                <ArrowRight />
              </div>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  {/if}
</div>
