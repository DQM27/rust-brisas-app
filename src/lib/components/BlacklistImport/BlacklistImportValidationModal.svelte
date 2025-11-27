<script lang="ts">
  import {
    X,
    AlertTriangle,
    Save,
    ChevronLeft,
    ChevronRight,
  } from "lucide-svelte";
  import type { BlacklistImportEntry } from "$lib/types/blacklistImport.types";

  interface Props {
    entries: BlacklistImportEntry[];
    onSave: (correctedEntries: BlacklistImportEntry[]) => void;
    onClose: () => void;
  }

  let { entries, onSave, onClose }: Props = $props();

  // Estado local de las entradas siendo editadas
  let editedEntries = $state<BlacklistImportEntry[]>([]);
  let currentIndex = $state(0);

  $effect(() => {
    editedEntries = entries.map((entry) => ({
      ...entry,
      segundoNombre: entry.segundoNombre,
      segundoApellido: entry.segundoApellido,
      observaciones: entry.observaciones,
    }));
  });

  const currentEntry = $derived(editedEntries[currentIndex]);
  const hasNext = $derived(currentIndex < editedEntries.length - 1);
  const hasPrev = $derived(currentIndex > 0);
  const progress = $derived(`${currentIndex + 1} / ${editedEntries.length}`);
  const progressPercent = $derived(
    ((currentIndex + 1) / editedEntries.length) * 100,
  );

  function handleNext() {
    if (hasNext) {
      currentIndex++;
    }
  }

  function handlePrev() {
    if (hasPrev) {
      currentIndex--;
    }
  }

  function handleSave() {
    onSave(editedEntries);
  }

  function updateCurrentEntry(field: keyof BlacklistImportEntry, value: any) {
    if (currentEntry) {
      editedEntries[currentIndex] = {
        ...currentEntry,
        [field]: value,
      };
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-[1000] animate-in fade-in duration-200"
>
  <div
    class="bg-[#252526] border border-white/10 rounded-xl w-[95%] max-w-4xl max-h-[95vh] flex flex-col shadow-2xl animate-in zoom-in-95 slide-in-from-bottom-4 duration-300"
  >
    <!-- Header -->
    <div class="flex items-start justify-between p-6 border-b border-white/10">
      <div class="flex gap-4">
        <div
          class="flex items-center justify-center w-12 h-12 rounded-lg bg-yellow-500/10 border border-yellow-500/20"
        >
          <AlertTriangle size={24} class="text-yellow-400" />
        </div>
        <div>
          <h2 class="text-xl font-semibold text-white mb-1">
            Validación Manual Requerida
          </h2>
          <p class="text-sm text-gray-400">
            Estos nombres contienen preposiciones y requieren corrección manual
          </p>
        </div>
      </div>
      <button
        onclick={onClose}
        class="flex items-center justify-center w-9 h-9 rounded-lg bg-transparent hover:bg-white/5 text-gray-400 hover:text-white transition-all"
        title="Cerrar"
      >
        <X size={20} />
      </button>
    </div>

    <!-- Progress Bar -->
    <div class="relative h-1 bg-white/5">
      <div
        class="absolute inset-y-0 left-0 bg-gradient-to-r from-blue-500 to-blue-400 transition-all duration-300 ease-out"
        style="width: {progressPercent}%"
      ></div>
    </div>

    <div class="px-6 py-3 bg-[#1e1e1e] border-b border-white/10">
      <p class="text-xs font-medium text-gray-400 text-center">
        Entrada {progress}
      </p>
    </div>

    {#if currentEntry}
      <!-- Form Container -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">
        <!-- Nombre Original - Card destacado -->
        <div
          class="p-5 bg-gradient-to-br from-yellow-500/10 to-orange-500/5 border border-yellow-500/20 rounded-xl"
        >
          <div class="flex items-start gap-3">
            <AlertTriangle
              size={20}
              class="text-yellow-400 mt-0.5 flex-shrink-0"
            />
            <div class="flex-1">
              <p
                class="text-xs font-semibold text-yellow-400 uppercase tracking-wider mb-2"
              >
                Nombre Original Detectado
              </p>
              <p class="text-lg font-semibold text-white mb-3">
                {currentEntry.primerNombre}
                {currentEntry.primerApellido}
              </p>
              {#if currentEntry.validationMessage}
                <p class="text-sm text-yellow-300/90 leading-relaxed">
                  {currentEntry.validationMessage}
                </p>
              {/if}
            </div>
          </div>
        </div>

        <!-- Datos de Solo Lectura -->
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <span
              class="block text-xs font-semibold text-gray-400 uppercase tracking-wider"
            >
              Cédula
            </span>
            <div
              class="px-4 py-3 bg-[#1e1e1e] border border-white/10 rounded-lg"
            >
              <p class="text-sm font-mono text-gray-300">
                {currentEntry.cedula}
              </p>
            </div>
          </div>
          <div class="space-y-2">
            <span
              class="block text-xs font-semibold text-gray-400 uppercase tracking-wider"
            >
              Empresa
            </span>
            <div
              class="px-4 py-3 bg-[#1e1e1e] border border-white/10 rounded-lg"
            >
              <p class="text-sm text-gray-300">{currentEntry.empresa}</p>
            </div>
          </div>
        </div>

        <!-- Sección Editable -->
        <div class="p-6 bg-[#1e1e1e] border border-blue-500/20 rounded-xl">
          <div class="flex items-center gap-2 mb-5">
            <div class="w-1 h-5 bg-blue-500 rounded-full"></div>
            <h3
              class="text-sm font-semibold text-white uppercase tracking-wider"
            >
              Corrija los Nombres
            </h3>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <!-- Primer Nombre -->
            <div class="space-y-2">
              <label
                for="primerNombre"
                class="block text-xs font-medium text-gray-300"
              >
                Primer Nombre <span class="text-red-400">*</span>
              </label>
              <input
                id="primerNombre"
                type="text"
                value={currentEntry.primerNombre}
                oninput={(e) =>
                  updateCurrentEntry("primerNombre", e.currentTarget.value)}
                placeholder="Juan"
                class="w-full px-4 py-2.5 bg-[#252526] border border-white/20 rounded-lg text-white text-sm placeholder:text-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
              />
            </div>

            <!-- Segundo Nombre -->
            <div class="space-y-2">
              <label
                for="segundoNombre"
                class="block text-xs font-medium text-gray-300"
              >
                Segundo Nombre
              </label>
              <input
                id="segundoNombre"
                type="text"
                value={currentEntry.segundoNombre || ""}
                oninput={(e) =>
                  updateCurrentEntry(
                    "segundoNombre",
                    e.currentTarget.value || undefined,
                  )}
                placeholder="Carlos"
                class="w-full px-4 py-2.5 bg-[#252526] border border-white/20 rounded-lg text-white text-sm placeholder:text-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
              />
            </div>

            <!-- Primer Apellido -->
            <div class="space-y-2">
              <label
                for="primerApellido"
                class="block text-xs font-medium text-gray-300"
              >
                Primer Apellido <span class="text-red-400">*</span>
              </label>
              <input
                id="primerApellido"
                type="text"
                value={currentEntry.primerApellido}
                oninput={(e) =>
                  updateCurrentEntry("primerApellido", e.currentTarget.value)}
                placeholder="Pérez"
                class="w-full px-4 py-2.5 bg-[#252526] border border-white/20 rounded-lg text-white text-sm placeholder:text-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
              />
            </div>

            <!-- Segundo Apellido -->
            <div class="space-y-2">
              <label
                for="segundoApellido"
                class="block text-xs font-medium text-gray-300"
              >
                Segundo Apellido
              </label>
              <input
                id="segundoApellido"
                type="text"
                value={currentEntry.segundoApellido || ""}
                oninput={(e) =>
                  updateCurrentEntry(
                    "segundoApellido",
                    e.currentTarget.value || undefined,
                  )}
                placeholder="Gómez"
                class="w-full px-4 py-2.5 bg-[#252526] border border-white/20 rounded-lg text-white text-sm placeholder:text-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-between gap-4 p-6 border-t border-white/10 bg-[#1e1e1e]"
      >
        <!-- Navegación -->
        <div class="flex gap-2">
          <button
            onclick={handlePrev}
            disabled={!hasPrev}
            class="flex items-center gap-2 px-4 py-2.5 bg-transparent border border-white/20 rounded-lg text-white text-sm font-medium hover:bg-white/5 disabled:opacity-40 disabled:cursor-not-allowed transition-all"
          >
            <ChevronLeft size={16} />
            Anterior
          </button>
          <button
            onclick={handleNext}
            disabled={!hasNext}
            class="flex items-center gap-2 px-4 py-2.5 bg-transparent border border-white/20 rounded-lg text-white text-sm font-medium hover:bg-white/5 disabled:opacity-40 disabled:cursor-not-allowed transition-all"
          >
            Siguiente
            <ChevronRight size={16} />
          </button>
        </div>

        <!-- Botón Guardar -->
        <button
          onclick={handleSave}
          class="flex items-center gap-2 px-6 py-2.5 bg-gradient-to-r from-blue-600 to-blue-500 rounded-lg text-white text-sm font-semibold hover:from-blue-500 hover:to-blue-400 shadow-lg shadow-blue-500/20 transition-all"
        >
          <Save size={18} />
          Guardar Correcciones
        </button>
      </div>
    {/if}
  </div>
</div>
