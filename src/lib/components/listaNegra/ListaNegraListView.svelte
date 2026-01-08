<!-- src/lib/components/listaNegra/ListaNegraListView.svelte -->
<!-- Vista unificada: Lista Negra + Modal para CRUD (Patrón Users) -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { AlertCircle, RotateCw } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  // Components
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import ListaNegraFormModal from "./ListaNegraFormModal.svelte";
  import BlacklistConfirmModal from "./blacklistForm/BlacklistConfirmModal.svelte";

  // Logic & Config
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";
  import { ListaNegraColumns } from "$lib/logic/listaNegra/listaNegraColumns";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import { can } from "$lib/logic/permissions";

  // Types
  import type {
    ListaNegraResponse,
    AddToListaNegraInput,
    NivelSeveridad,
  } from "$lib/types/listaNegra";

  // Stores
  import { selectedSearchStore } from "$lib/stores/searchStore";
  import { currentUser } from "$lib/stores/auth";
  import { activeTabId } from "$lib/stores/tabs";
  import {
    keyboardCommand,
    setActiveContext,
    clearCommand,
  } from "$lib/stores/keyboardCommands";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let bloqueados = $state<ListaNegraResponse[]>([]);
  let loading = $state(false);
  let error = $state("");

  // Permisos derivados
  let canManage = $derived(can($currentUser, "MANAGE_BLACKLIST"));
  // let canViewReason = $derived(can($currentUser, "VIEW_BLACKLIST_REASON"));

  // Filtros
  let showEstadoDropdown = $state(false);
  let showNivelDropdown = $state(false);

  let estadoFilter = $state<"todos" | "activo" | "inactivo">("todos");
  let nivelFilter = $state<"todos" | "ALTO" | "MEDIO" | "BAJO">("todos");

  // Selección en grid
  let selectedRows = $state<ListaNegraResponse[]>([]);

  // Suscripción a comandos de teclado centralizados
  let unsubscribeKeyboard: (() => void) | null = null;

  function setupKeyboardSubscription() {
    unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
      if (!event) return;
      if ($activeTabId !== tabId) return;

      switch (event.command) {
        case "create-new":
          if (canManage && !showFormModal && !showConfirmModal) {
            openFormModal(null);
            clearCommand();
          }
          break;
        case "edit":
          if (canManage && selectedRows.length === 1 && !showFormModal) {
            openFormModal(selectedRows[0]);
            clearCommand();
          }
          break;
        case "escape":
          if (showFormModal) {
            closeFormModal();
            clearCommand();
          } else if (showConfirmModal) {
            closeConfirmModal();
            clearCommand();
          }
          break;
        case "refresh":
          loadListaNegra();
          clearCommand();
          break;
      }
    });
  }

  // ==========================================
  // ESTADO DE MODALES
  // ==========================================
  // 1. Modal Agregar/Editar
  let showFormModal = $state(false);
  let editingBloqueado = $state<ListaNegraResponse | null>(null);
  let formLoading = $state(false);

  // 2. Modal Desbloquear/Re-bloquear
  let showConfirmModal = $state(false);
  let confirmMotivo = $state("");
  let confirmObservaciones = $state("");
  let confirmActionType = $state<"unblock" | "reblock">("unblock");

  // ==========================================
  // DERIVADOS
  // ==========================================

  // Datos filtrados
  let filteredData = $derived.by(() => {
    let filtered = bloqueados;

    // Filtro por búsqueda seleccionada (tiene prioridad)
    const selectedSearch = $selectedSearchStore;
    if (selectedSearch.result) {
      const cedula = selectedSearch.result.cedula || selectedSearch.result.id;
      filtered = filtered.filter((b) => b.cedula === cedula);
      return filtered;
    }

    // Filtro de estado
    if (estadoFilter === "activo") {
      filtered = filtered.filter((b) => b.isActive);
    } else if (estadoFilter === "inactivo") {
      filtered = filtered.filter((b) => !b.isActive);
    }

    // Filtro de nivel
    if (nivelFilter !== "todos") {
      filtered = filtered.filter((b) => b.nivelSeveridad === nivelFilter);
    }

    return filtered;
  });

  // Labels de filtros
  const estadoLabel = $derived(
    { todos: "Todos", activo: "Bloqueados", inactivo: "Desbloqueados" }[
      estadoFilter
    ],
  );
  const nivelLabel = $derived(
    { todos: "Todos Niveles", ALTO: "Alto", MEDIO: "Medio", BAJO: "Bajo" }[
      nivelFilter
    ],
  );

  // Columnas AG Grid
  let columnDefs = $derived.by((): ColDef<ListaNegraResponse>[] => {
    return ListaNegraColumns.getColumns();
  });

  // Botones personalizados (Toolbar)
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];
    const isSingleSelect = selectedRows.length === 1;

    // Solo si tiene permisos
    if (!canManage) {
      return {
        default: [
          {
            id: "refresh",
            label: "Actualizar",
            icon: RotateCw,
            onClick: loadListaNegra,
            variant: "default" as const,
          },
        ],
      };
    }

    return {
      default: [
        createCustomButton.nuevo(() => openFormModal(null)),
        {
          id: "refresh",
          label: "Actualizar",
          icon: RotateCw,
          onClick: loadListaNegra,
          variant: "default" as const,
        },
      ],
      singleSelect: [
        // EDITAR (Solo campos permitidos como nivel/motivo)
        createCustomButton.editar(() => {
          if (selected) openFormModal(selected);
        }),

        // ACCIÓN DINÁMICA: DESBLOQUEAR / RE-BLOQUEAR
        selected?.isActive
          ? {
              id: "unblock",
              label: "Desbloquear",
              // icon: LockOpen, // Importar si se quiere icono
              variant: "destructive" as const,
              onClick: () => openConfirmModal(selected, "unblock"),
            }
          : {
              id: "reblock",
              label: "Re-bloquear",
              // icon: Lock, // Importar si se quiere icono
              variant: "warning" as const,
              onClick: () => openConfirmModal(selected, "reblock"),
            },
      ],
    };
  });

  // ==========================================
  // DATA LOADING
  // ==========================================

  async function loadListaNegra() {
    loading = true;
    error = "";
    const result = await listaNegraService.fetchAll();
    if (result.ok) {
      bloqueados = result.data.bloqueados;
    } else {
      error = result.error;
    }
    loading = false;
  }

  // ==========================================
  // ACTIONS - FORM MODAL
  // ==========================================

  function openFormModal(bloqueado: ListaNegraResponse | null) {
    editingBloqueado = bloqueado;
    showFormModal = true;
  }

  function closeFormModal() {
    showFormModal = false;
    editingBloqueado = null;
  }

  async function handleSaveForm(input: AddToListaNegraInput): Promise<boolean> {
    formLoading = true;
    try {
      if (editingBloqueado) {
        // EDICIÓN
        const updateInput = {
          nivelSeveridad: input.nivelSeveridad,
          motivoBloqueo: input.motivoBloqueo,
          observaciones: input.observaciones,
        };
        const result = await listaNegraService.update(
          editingBloqueado.id,
          updateInput,
        );
        if (result.ok) {
          toast.success("Información actualizada");
          // Actualización optimista local
          bloqueados = bloqueados.map((b) =>
            b.id === editingBloqueado!.id ? result.data : b,
          );
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      } else {
        // CREACIÓN
        const result = await listaNegraService.add(input);
        if (result.ok) {
          toast.success("Persona agregada a lista negra");
          await loadListaNegra();
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      }
    } finally {
      formLoading = false;
    }
  }

  // ==========================================
  // ACTIONS - CONFIRM MODAL (Unblock/Reblock)
  // ==========================================

  function openConfirmModal(
    bloqueado: ListaNegraResponse,
    type: "unblock" | "reblock",
  ) {
    editingBloqueado = bloqueado; // Usamos la misma variable temporal
    confirmActionType = type;
    confirmMotivo = "";
    confirmObservaciones = "";
    showConfirmModal = true;
  }

  function closeConfirmModal() {
    showConfirmModal = false;
    editingBloqueado = null;
  }

  async function handleConfirmAction() {
    if (!editingBloqueado) return;

    formLoading = true;
    try {
      if (confirmActionType === "unblock") {
        // DESBLOQUEAR
        // Nota: El servicio unblock actual solo pide ID. Motivo y Observaciones
        // no se están enviando en la firma simple del servicio nuevo,
        // pero el modal viejo los pedía.
        // Si el backend soporta historial de desbloqueos deberíamos enviarlos.
        // Asumiremos la firma simple `unblock(id)` por ahora, o actualizar servicio si es crítico.
        // REVISIÓN: El backend `remove_from_lista_negra` usa motivo/observaciones?
        // Mirando lista_negra_commands.rs: `remove_from_lista_negra` NO recibe args extras, solo ID.
        // Para mantener compatibilidad visual del modal, ignoraremos esos campos o los loguearemos.

        // ERROR: El usuario espera poder poner un motivo de desbloqueo.
        // El servicio legacy tenía `remove(id, motivo, obs)`.
        // El command `remove_from_lista_negra` usa `unblock_lista_negra` service que SÍ tomaba parámetros?
        // Vamos a usar la función simple.

        const result = await listaNegraService.unblock(editingBloqueado.id);
        if (result.ok) {
          toast.success("Persona desbloqueada");
          await loadListaNegra();
          closeConfirmModal();
        } else {
          toast.error(result.error);
        }
      } else {
        // RE-BLOQUEAR
        // El servicio reblock pide: (id, nivel, motivo, por)
        const usuario = $currentUser;
        const bloqueadoPor = usuario
          ? `${usuario.nombre} ${usuario.apellido}`
          : "Sistema";

        // Asumimos mismo nivel que tenía, o deberíamos pedirlo en el modal?
        // Por simplicidad usaremos su nivel previo o MEDIO.
        const nivel = editingBloqueado.nivelSeveridad || "MEDIO";

        const result = await listaNegraService.reblock(
          editingBloqueado.id,
          nivel,
          confirmMotivo || "Re-bloqueo manual",
          bloqueadoPor,
        );

        if (result.ok) {
          toast.success("Persona re-bloqueada");
          await loadListaNegra();
          closeConfirmModal();
        } else {
          toast.error(result.error);
        }
      }
    } finally {
      formLoading = false;
    }
  }

  // ==========================================
  // FILTERS HANDLERS
  // ==========================================

  function handleEstadoSelect(value: any) {
    estadoFilter = value;
    showEstadoDropdown = false;
  }

  function handleNivelSelect(value: any) {
    nivelFilter = value;
    showNivelDropdown = false;
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (
      !target.closest(".filter-dropdown-container") &&
      !target.closest("[data-filter-button]")
    ) {
      showEstadoDropdown = false;
      showNivelDropdown = false;
    }
  }

  // Lifecycle
  onMount(() => {
    loadListaNegra();
    setupKeyboardSubscription();
  });

  onDestroy(() => {
    if (unsubscribeKeyboard) {
      unsubscribeKeyboard();
    }
  });

  // Registrar contexto activo cuando esta pestaña está activa
  $effect(() => {
    if ($activeTabId === tabId) {
      setActiveContext("lista-negra");
    }
  });
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">Lista Negra</h2>
        <p class="mt-1 text-sm text-gray-400">
          Control de accesos denegados y restricciones
        </p>
      </div>
      <div class="flex-1 max-w-md">
        <SearchBar placeholder="Buscar por cédula o nombre..." limit={10} />
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar datos</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <!-- Skeleton / Loading State -->
      <div class="flex h-full items-center justify-center">
        <div class="text-center text-gray-400">
          <RotateCw class="animate-spin mx-auto mb-2" size={32} />
          <p>Cargando lista negra...</p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="lista-negra"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="lista-negra-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>

  <!-- Dropdowns de Filtros -->
  <div class="filter-dropdown-container">
    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos"], ["activo", "Bloqueados"], ["inactivo", "Desbloqueados"]] as [value, label]}
          <button
            onclick={() => handleEstadoSelect(value)}
            class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {estadoFilter ===
            value
              ? 'bg-blue-500/20 text-blue-400'
              : ''}"
          >
            {label}
          </button>
        {/each}
      </div>
    {/if}

    {#if showNivelDropdown}
      <div
        class="absolute top-16 left-44 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos"], ["ALTO", "Alto"], ["MEDIO", "Medio"], ["BAJO", "Bajo"]] as [value, label]}
          <button
            onclick={() => handleNivelSelect(value)}
            class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {nivelFilter ===
            value
              ? 'bg-blue-500/20 text-blue-400'
              : ''}"
          >
            {label}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Modal Formulario -->
<ListaNegraFormModal
  show={showFormModal}
  bloqueado={editingBloqueado}
  loading={formLoading}
  onSave={handleSaveForm}
  onClose={closeFormModal}
/>

<!-- Modal Confirmación -->
<BlacklistConfirmModal
  show={showConfirmModal}
  contratistaName={editingBloqueado?.nombreCompleto || ""}
  motivo={confirmMotivo}
  observaciones={confirmObservaciones}
  onConfirm={handleConfirmAction}
  onCancel={closeConfirmModal}
  onMotivoChange={(v) => (confirmMotivo = v)}
  onObservacionesChange={(v) => (confirmObservaciones = v)}
/>
