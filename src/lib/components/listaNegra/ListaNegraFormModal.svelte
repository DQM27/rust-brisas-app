<!-- src/lib/components/listaNegra/ListaNegraFormModal.svelte -->
<!-- Modal para agregar/editar personas en lista negra (Validación Zod) -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Search, AlertTriangle, User, Building2 } from "lucide-svelte";
  import { toast } from "svelte-5-french-toast";
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";
  import { currentUser } from "$lib/stores/auth";
  import type {
    ListaNegraResponse,
    AddToListaNegraInput,
    PersonaSearchResult,
    NivelSeveridad,
  } from "$lib/types/listaNegra";
  import {
    AddToListaNegraSchema,
    type AddToListaNegraForm,
  } from "$lib/schemas/listaNegraSchema";

  interface Props {
    show: boolean;
    bloqueado?: ListaNegraResponse | null;
    loading?: boolean;
    onSave: (data: AddToListaNegraInput) => Promise<boolean | void>;
    onClose: () => void;
  }

  let {
    show,
    bloqueado = null,
    loading = false,
    onSave,
    onClose,
  }: Props = $props();

  // Modo derivado
  const isEditMode = $derived(!!bloqueado);
  const modalTitle = $derived(
    isEditMode
      ? `Editar: ${bloqueado?.nombreCompleto}`
      : "Agregar a Lista Negra",
  );

  // Estado del formulario
  let formData = $state<AddToListaNegraForm>({
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
    segundoApellido: "",
    empresaId: "",
    empresaNombre: "",
    nivelSeveridad: "MEDIO",
    motivoBloqueo: "",
    observaciones: "",
  });

  // Estado de búsqueda
  let searchQuery = $state("");
  let searchResults = $state<PersonaSearchResult[]>([]);
  let isSearching = $state(false);
  let showResults = $state(false);
  let selectedPersona = $state<PersonaSearchResult | null>(null);

  // Errores
  let errors = $state<Record<string, string>>({});

  // Cargar datos en modo edición
  $effect(() => {
    if (show && bloqueado) {
      formData = {
        cedula: bloqueado.cedula || "",
        nombre: bloqueado.nombre || "",
        segundoNombre: bloqueado.segundoNombre || "",
        apellido: bloqueado.apellido || "",
        segundoApellido: bloqueado.segundoApellido || "",
        empresaId: bloqueado.empresaId || "",
        empresaNombre: bloqueado.empresaNombre || "",
        nivelSeveridad: bloqueado.nivelSeveridad || "MEDIO",
        motivoBloqueo: bloqueado.motivoBloqueo || "",
        observaciones: bloqueado.observaciones || "",
      };
      errors = {};
      selectedPersona = null;
    } else if (show && !bloqueado) {
      // Reset para creación
      formData = {
        cedula: "",
        nombre: "",
        segundoNombre: "",
        apellido: "",
        segundoApellido: "",
        empresaId: "",
        empresaNombre: "",
        nivelSeveridad: "MEDIO",
        motivoBloqueo: "",
        observaciones: "",
      };
      errors = {};
      searchQuery = "";
      searchResults = [];
      selectedPersona = null;
    }
  });

  // Validación reactiva
  $effect(() => {
    if (Object.values(formData).some((v) => v !== "")) {
      // Solo validamos si hay datos
      const result = AddToListaNegraSchema.safeParse(formData);
      if (!result.success) {
        const newErrors: Record<string, string> = {};
        result.error.issues.forEach((issue) => {
          if (issue.path[0]) {
            newErrors[String(issue.path[0])] = issue.message;
          }
        });
        errors = newErrors;
      } else {
        errors = {};
      }
    }
  });

  // Búsqueda de personas
  async function handleSearch() {
    if (searchQuery.trim().length < 2) {
      searchResults = [];
      return;
    }

    isSearching = true;
    const result = await listaNegraService.searchPersonas(searchQuery);
    isSearching = false;

    if (result.ok) {
      searchResults = result.data;
      showResults = true;
    } else {
      toast.error(result.error);
    }
  }

  function selectPersona(persona: PersonaSearchResult) {
    if (persona.yaBloqueado) {
      toast.error("Esta persona ya está bloqueada");
      return;
    }

    selectedPersona = persona;
    formData = {
      ...formData,
      cedula: persona.cedula,
      nombre: persona.nombre,
      segundoNombre: persona.segundoNombre || "",
      apellido: persona.apellido,
      segundoApellido: persona.segundoApellido || "",
      empresaId: persona.empresaId || "",
      empresaNombre: persona.empresaNombre || "",
    };
    showResults = false;
    searchQuery = "";
  }

  function clearSelection() {
    selectedPersona = null;
    formData = {
      cedula: "",
      nombre: "",
      segundoNombre: "",
      apellido: "",
      segundoApellido: "",
      empresaId: "",
      empresaNombre: "",
      nivelSeveridad: formData.nivelSeveridad,
      motivoBloqueo: formData.motivoBloqueo,
      observaciones: formData.observaciones,
    };
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();

    // Validación final con Zod
    const result = AddToListaNegraSchema.safeParse(formData);

    if (!result.success) {
      const newErrors: Record<string, string> = {};
      result.error.issues.forEach((issue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
      return;
    }

    const usuario = $currentUser;
    const bloqueadoPor = usuario
      ? `${usuario.nombre} ${usuario.apellido}`
      : "Sistema";

    const input: AddToListaNegraInput = {
      ...result.data,
      bloqueadoPor,
    };

    const success = await onSave(input);
    if (success) {
      onClose();
    }
  }

  // Colores de nivel
  const nivelColors = {
    ALTO: "bg-red-500/20 text-red-400 border-red-500/50",
    MEDIO: "bg-yellow-500/20 text-yellow-400 border-yellow-500/50",
    BAJO: "bg-gray-500/20 text-gray-400 border-gray-500/50",
  };

  const tipoPersonaIcons = {
    contratista: "👷",
    proveedor: "📦",
    visita: "👤",
  };

  // Styles
  const inputClass =
    "w-full rounded-md border border-gray-600 bg-[#0d1117] px-3 py-2 text-sm text-gray-100 placeholder:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";
  const labelClass = "block text-sm font-medium text-gray-300 mb-1";
  const errorClass = "text-xs text-red-400 mt-1";

  // Handler para Ctrl+S
  function handleKeydown(e: KeyboardEvent) {
    if (!show || loading) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      const form = document.querySelector("form") as HTMLFormElement;
      if (form) {
        form.requestSubmit();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
      onclick={onClose}
      aria-label="Cerrar modal"
    ></button>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-2xl max-h-[90vh] overflow-auto rounded-lg bg-[#0d1117] shadow-2xl border border-gray-700"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-20 flex items-center justify-between px-6 py-4 bg-[#0d1117] border-b border-gray-700"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-red-500/10">
            <AlertTriangle size={20} class="text-red-400" />
          </div>
          <h2 class="text-xl font-semibold text-white">{modalTitle}</h2>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-200 hover:bg-gray-800 transition-colors"
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="p-6 space-y-5">
        <!-- Búsqueda de persona (solo en creación) -->
        {#if !isEditMode}
          <div class="space-y-2">
            <label for="search" class={labelClass}>Buscar persona</label>

            {#if selectedPersona}
              <!-- Persona seleccionada -->
              <div
                class="flex items-center gap-3 p-3 rounded-lg bg-blue-500/10 border border-blue-500/30"
              >
                <User size={20} class="text-blue-400" />
                <div class="flex-1">
                  <div class="font-medium text-white">
                    {selectedPersona.nombreCompleto}
                  </div>
                  <div class="text-sm text-gray-400">
                    {selectedPersona.cedula} •
                    {tipoPersonaIcons[selectedPersona.tipoPersona]}
                    {selectedPersona.tipoPersona}
                    {#if selectedPersona.empresaNombre}
                      • {selectedPersona.empresaNombre}
                    {/if}
                  </div>
                </div>
                <button
                  type="button"
                  onclick={clearSelection}
                  class="p-1 text-gray-400 hover:text-white"
                >
                  <X size={16} />
                </button>
              </div>
            {:else}
              <!-- Campo de búsqueda -->
              <div class="relative">
                <input
                  id="search"
                  type="text"
                  bind:value={searchQuery}
                  oninput={() => handleSearch()}
                  placeholder="Buscar por cédula o nombre..."
                  class={inputClass}
                />
                <div class="absolute right-3 top-1/2 -translate-y-1/2">
                  {#if isSearching}
                    <div
                      class="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"
                    ></div>
                  {:else}
                    <Search size={16} class="text-gray-500" />
                  {/if}
                </div>
              </div>

              <!-- Resultados de búsqueda (Resto igual) -->
              {#if showResults && searchResults.length > 0}
                <div
                  class="absolute z-30 w-full max-h-60 overflow-auto rounded-lg bg-[#161b22] border border-gray-700 shadow-xl"
                >
                  {#each searchResults as persona}
                    <button
                      type="button"
                      onclick={() => selectPersona(persona)}
                      disabled={persona.yaBloqueado}
                      class="w-full px-4 py-3 text-left hover:bg-gray-700/50 transition-colors border-b border-gray-700/50 last:border-0 disabled:opacity-50"
                    >
                      <div class="flex items-center gap-3">
                        <span class="text-lg"
                          >{tipoPersonaIcons[persona.tipoPersona]}</span
                        >
                        <div class="flex-1">
                          <div class="font-medium text-white">
                            {persona.nombreCompleto}
                            {#if persona.yaBloqueado}
                              <span class="ml-2 text-xs text-red-400"
                                >(Ya bloqueado)</span
                              >
                            {/if}
                          </div>
                          <div class="text-sm text-gray-400">
                            {persona.cedula}
                            {#if persona.empresaNombre}
                              • {persona.empresaNombre}
                            {/if}
                          </div>
                        </div>
                      </div>
                    </button>
                  {/each}
                </div>
              {/if}

              <!-- Entrada manual -->
              <p class="text-xs text-gray-500">
                O ingresa los datos manualmente si la persona no está registrada
              </p>
            {/if}
          </div>
        {/if}

        <!-- Datos de la persona -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="cedula" class={labelClass}>Cédula *</label>
            <input
              id="cedula"
              type="text"
              bind:value={formData.cedula}
              disabled={loading || isEditMode}
              class={inputClass}
              placeholder="1-1234-5678"
            />
            {#if errors.cedula}<p class={errorClass}>{errors.cedula}</p>{/if}
          </div>

          <div>
            <label for="empresaNombre" class={labelClass}>Empresa</label>
            <div class="relative">
              <Building2
                size={16}
                class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
              />
              <input
                id="empresaNombre"
                type="text"
                bind:value={formData.empresaNombre}
                disabled={loading}
                class="{inputClass} pl-9"
                placeholder="Nombre de empresa"
              />
            </div>
            {#if errors.empresaNombre}<p class={errorClass}>
                {errors.empresaNombre}
              </p>{/if}
          </div>

          <div>
            <label for="nombre" class={labelClass}>Nombre *</label>
            <input
              id="nombre"
              type="text"
              bind:value={formData.nombre}
              disabled={loading}
              class={inputClass}
              placeholder="Juan"
            />
            {#if errors.nombre}<p class={errorClass}>{errors.nombre}</p>{/if}
          </div>

          <div>
            <label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
            <input
              id="segundoNombre"
              type="text"
              bind:value={formData.segundoNombre}
              disabled={loading}
              class={inputClass}
              placeholder="Carlos"
            />
            {#if errors.segundoNombre}<p class={errorClass}>
                {errors.segundoNombre}
              </p>{/if}
          </div>

          <div>
            <label for="apellido" class={labelClass}>Apellido *</label>
            <input
              id="apellido"
              type="text"
              bind:value={formData.apellido}
              disabled={loading}
              class={inputClass}
              placeholder="Pérez"
            />
            {#if errors.apellido}<p class={errorClass}>
                {errors.apellido}
              </p>{/if}
          </div>

          <div>
            <label for="segundoApellido" class={labelClass}
              >Segundo Apellido</label
            >
            <input
              id="segundoApellido"
              type="text"
              bind:value={formData.segundoApellido}
              disabled={loading}
              class={inputClass}
              placeholder="González"
            />
            {#if errors.segundoApellido}<p class={errorClass}>
                {errors.segundoApellido}
              </p>{/if}
          </div>
        </div>

        <!-- Nivel de Severidad -->
        <div>
          <span class={labelClass}>Nivel de Severidad *</span>
          <div class="grid grid-cols-3 gap-3">
            {#each ["ALTO", "MEDIO", "BAJO"] as const as nivel}
              <button
                type="button"
                onclick={() => (formData.nivelSeveridad = nivel)}
                class="p-3 rounded-lg border-2 transition-all {formData.nivelSeveridad ===
                nivel
                  ? nivelColors[nivel]
                  : 'border-gray-700 text-gray-400 hover:border-gray-600'}"
              >
                <div class="font-medium">{nivel}</div>
                <div class="text-xs opacity-70">
                  {nivel === "ALTO"
                    ? "Crítico"
                    : nivel === "MEDIO"
                      ? "Moderado"
                      : "Bajo riesgo"}
                </div>
              </button>
            {/each}
          </div>
          {#if errors.nivelSeveridad}<p class={errorClass}>
              {errors.nivelSeveridad}
            </p>{/if}
        </div>

        <!-- Motivo -->
        <div>
          <label for="motivoBloqueo" class={labelClass}
            >Motivo del Bloqueo *</label
          >
          <textarea
            id="motivoBloqueo"
            bind:value={formData.motivoBloqueo}
            disabled={loading}
            class={inputClass}
            rows="3"
            placeholder="Describa el motivo del bloqueo..."
          ></textarea>
          {#if errors.motivoBloqueo}<p class={errorClass}>
              {errors.motivoBloqueo}
            </p>{/if}
        </div>

        <!-- Observaciones -->
        <div>
          <label for="observaciones" class={labelClass}>Observaciones</label>
          <textarea
            id="observaciones"
            bind:value={formData.observaciones}
            disabled={loading}
            class={inputClass}
            rows="2"
            placeholder="Notas adicionales (opcional)"
          ></textarea>
          {#if errors.observaciones}<p class={errorClass}>
              {errors.observaciones}
            </p>{/if}
        </div>

        <!-- Buttons -->
        <div class="flex gap-3 pt-4 border-t border-gray-700">
          <button
            type="button"
            onclick={onClose}
            class="flex-1 py-2.5 px-4 rounded-md border border-gray-600 text-gray-300 hover:bg-gray-800 transition-colors"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading}
            class="flex-1 py-2.5 px-4 rounded-md bg-red-600 text-white font-medium hover:bg-red-700 disabled:opacity-50 transition-colors"
          >
            {loading
              ? "Guardando..."
              : isEditMode
                ? "Guardar Cambios"
                : "Agregar a Lista Negra"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
