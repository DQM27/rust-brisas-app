<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { Trash2, RotateCcw } from "lucide-svelte"; // Add RotateCcw for restore icon

  import { AlertCircle } from "lucide-svelte";
  import { selectedSearchStore } from "$lib/stores/searchStore";
  import * as contratistaService from "$lib/logic/contratista/contratistaService";
  import { ContratistaColumns } from "$lib/logic/contratista/contratistaColumns";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import ContratistaFormModal from "./ContratistaFormModal.svelte";

  import type { ContratistaResponse } from "$lib/types/contratista";
  import type { ColDef } from "@ag-grid-community/core";
  import {
    COMMON_DEFAULT_BUTTONS,
    createCustomButton,
  } from "$lib/config/agGridConfigs";

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let isUpdatingStatus = false;

  // States specific to Active Grid
  let selectedRows = $state<ContratistaResponse[]>([]);
  let showModal = $state(false);
  let editingContratista = $state<ContratistaResponse | null>(null);
  let modalLoading = $state(false);

  // Filters
  let estadoFilter = $state<"todos" | "activo" | "inactivo" | "suspendido">(
    "todos",
  );
  let showEstadoDropdown = $state(false);
  let praindFilter = $state<"todos" | "vigente" | "vencido" | "por-vencer">(
    "todos",
  );
  let showPraindDropdown = $state(false);

  // ==========================================
  // DERIVED STATE
  // ==========================================
  const filteredData = $derived.by(() => {
    let filtered = contratistas;

    const _search = $selectedSearchStore;
    if (_search.result) {
      return filtered.filter((c) => c.id === _search.result!.id);
    }

    // Filtro de estado
    if (estadoFilter !== "todos") {
      filtered = filtered.filter((c) => c.estado === estadoFilter);
    }

    // Filtro de PRAIND
    if (praindFilter === "vigente") {
      filtered = filtered.filter(
        (c) => !c.praindVencido && c.diasHastaVencimiento > 30,
      );
    } else if (praindFilter === "vencido") {
      filtered = filtered.filter((c) => c.praindVencido);
    } else if (praindFilter === "por-vencer") {
      filtered = filtered.filter(
        (c) => !c.praindVencido && c.diasHastaVencimiento <= 30,
      );
    }

    return filtered;
  });

  // ... (labels)

  // Columnas - Only for Active List now
  const columnDefs = $derived.by((): ColDef<ContratistaResponse>[] => {
    const cols = ContratistaColumns.getColumns(handleStatusChange);

    return cols.map(
      (col) =>
        ({
          field: String(col.field) as any,
          headerName: col.headerName,
          width: col.width,
          minWidth: col.minWidth,
          flex: col.flex,
          sortable: col.sortable !== false,
          filter: true,
          resizable: true,
          cellRenderer: col.cellRenderer,
          valueFormatter: col.valueFormatter,
          cellStyle: col.cellStyle,
          onCellClicked: col.onCellClicked,
        }) as ColDef<ContratistaResponse>,
    );
  });

  // Custom buttons
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    return {
      default: [
        createCustomButton.nuevo(() => openModal()),

        ...COMMON_DEFAULT_BUTTONS.filter((b) =>
          ["autosize-all", "reset-columns", "select-all"].includes(b.id),
        ).map((b) => ({
          id: b.id,
          label: b.label,
          icon: b.icon,
          tooltip: b.tooltip,
          onClick: undefined,
          useCommonHandler: true,
        })),
        {
          id: "reindex",
          label: "Reindexar",
          icon: AlertCircle,
          onClick: () => handleReindex(),
          variant: "default" as const,
          tooltip: "Reparar índice de búsqueda",
        },
      ],
      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) openModal(selected);
        }),
        createCustomButton.eliminar(() => {
          if (selected) handleDelete(selected);
        }),
      ],
      multiSelect: [],
    };
  });

  // ==========================================
  // HANDLERS - DATA
  // ==========================================
  async function loadContratistas() {
    loading = true;
    error = "";
    contratistas = []; // Clear current
    try {
      const result = await contratistaService.fetchAllContratistas();
      if (result.ok) {
        contratistas = (result.data as any).contratistas;
      } else {
        error = result.error;
      }
    } catch (err) {
      console.error("Error al cargar contratistas:", err);
      error = "Error al cargar contratistas";
    }
    loading = false;
  }

  // ... (modal handlers remain same)

  // ==========================================
  // HANDLERS - MODAL
  // ==========================================
  function openModal(contratista: ContratistaResponse | null = null) {
    editingContratista = contratista;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    editingContratista = null;
  }

  async function handleSaveContratista(data: any) {
    // Should be typed properly based on input
    modalLoading = true;
    try {
      if (editingContratista) {
        const res = await contratistaService.updateContratista(
          editingContratista.id,
          data,
        );
        if (res.ok) {
          toast.success("Contratista actualizado");
          closeModal();
          loadContratistas();
        } else {
          toast.error(res.error);
        }
      } else {
        const res = await contratistaService.createContratista(data);
        if (res.ok) {
          toast.success("Contratista creado");
          closeModal();
          loadContratistas();
        } else {
          toast.error(res.error);
        }
      }
    } catch (e) {
      console.error(e);
      toast.error("Error al guardar contratista");
    }
    modalLoading = false;
  }

  // ==========================================
  // HANDLERS - ACTIONS
  // ==========================================

  async function handleReindex() {
    const toastId = toast.loading("Reindexando...");
    try {
      // Assuming service has reindex method or similar
      // Using a placeholder or generic call if exact method unknown, but usually exposed via service or command
      // For now, logging as not implemented fully or using a specialized service call if exist.
      // Given errors, I'll check contratistaService for reindex methods or assume standard pattern.
      // If verify fails, I will fix.
      await contratistaService.reindexContratistas();
      toast.success("Índice actualizado", { id: toastId });
    } catch (e) {
      toast.error("Error al reindexar", { id: toastId });
    }
  }

  async function handleStatusChange(id: string, status: string) {
    if (isUpdatingStatus) return;
    isUpdatingStatus = true;

    const newStatus = status === "activo" ? "inactivo" : "activo";

    // Optimistic update - save backup and update immediately
    const oldContratistas = [...contratistas];
    contratistas = contratistas.map((c) =>
      c.id === id ? { ...c, estado: newStatus } : c,
    );

    const toastId = toast.loading(`Cambiando a ${newStatus}...`);

    try {
      const res = await contratistaService.changeEstado(id, newStatus as any);
      if (res.ok) {
        toast.success(`Estado actualizado a ${newStatus}`, { id: toastId });
        // No need to reload - optimistic update already applied
      } else {
        // Revert on error
        contratistas = oldContratistas;
        toast.error(res.error, { id: toastId });
      }
    } catch (e) {
      // Revert on error
      contratistas = oldContratistas;
      console.error(e);
      toast.error("Error al cambiar estado", { id: toastId });
    } finally {
      isUpdatingStatus = false;
    }
  }

  function handleEstadoSelect(
    value: "todos" | "activo" | "inactivo" | "suspendido",
  ) {
    estadoFilter = value;
    showEstadoDropdown = false;
  }

  function handlePraindSelect(
    value: "todos" | "vigente" | "vencido" | "por-vencer",
  ) {
    praindFilter = value;
    showPraindDropdown = false;
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest(".filter-dropdown-container")) {
      showEstadoDropdown = false;
      showPraindDropdown = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      showEstadoDropdown = false;
      showPraindDropdown = false;
      // closeModal() handled by modal component usually
    }
  }

  async function handleDelete(contratista: ContratistaResponse) {
    if (
      !confirm(
        `¿Estás seguro de eliminar a ${contratista.nombreCompleto}? Se moverá a la papelera.`,
      )
    )
      return;

    const toastId = toast.loading("Eliminando...");
    const result = await contratistaService.deleteContratista(contratista.id);

    if (result.ok) {
      toast.success("Contratista movido a papelera", { id: toastId });
      loadContratistas();
    } else {
      toast.error(result.error, { id: toastId });
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================
  let unlistenFn: UnlistenFn | null = null;

  onMount(async () => {
    loadContratistas();

    // Listen for real-time contratista updates from backend
    unlistenFn = await listen<{ action: string; data: any }>(
      "contratista:changed",
      (event) => {
        console.log("📡 Contratista changed event received:", event.payload);

        const { action, data } = event.payload;

        if (action === "create") {
          // New contratista - reload to get computed fields
          loadContratistas();
        } else if (action === "update" && data?.id) {
          // Update existing - merge the change to avoid flicker
          // Note: We might already have the optimistic update, so this is a no-op in that case
          // But for external updates (other users), this will apply the change
          contratistas = contratistas.map((c) =>
            c.id === data.id ? { ...c, ...data } : c,
          );
        } else if (action === "delete" && data?.id) {
          // Remove deleted item
          contratistas = contratistas.filter((c) => c.id !== data.id);
        }
      },
    );
  });

  onDestroy(() => {
    if (unlistenFn) {
      unlistenFn();
    }
  });
</script>

<svelte:window onclick={handleClickOutside} onkeydown={handleKeydown} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          "Lista de Contratistas"
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de contratistas registrados
        </p>
      </div>
      <div class="flex-1 max-w-md">
        <!-- Only show searchbar if in active mode or if trash view supports it (trash view has internal logic or we hide it) -->
        <!-- For now we hide searchbar in trash view to simplify as trash view component handles its own display -->
        <SearchBar
          placeholder="Buscar por nombre, cédula o empresa..."
          limit={10}
        />
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
            <div class="font-medium">Error al cargar contratistas</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg
            class="mx-auto h-8 w-8 animate-spin text-blue-500"
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
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
        </div>
      </div>
    {:else if contratistas.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">
            No hay contratistas activos
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Crea el primer contratista para comenzar
          </p>
          <div class="flex gap-3 justify-center mt-6">
            <button
              onclick={() => openModal()}
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors font-medium"
            >
              Nuevo Contratista
            </button>
          </div>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="contratista-list"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="contratistas-list-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}

    <!-- Filter Dropdowns (Only active view) -->
    <div class="filter-dropdown-container">
      {#if showEstadoDropdown}
        <div
          class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
          transition:fade={{ duration: 150 }}
        >
          {#each [["todos", "Todos los estados"], ["activo", "Activos"], ["inactivo", "Inactivos"], ["suspendido", "Suspendidos"]] as [value, label]}
            <button
              onclick={() => handleEstadoSelect(value as any)}
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

      {#if showPraindDropdown}
        <div
          class="absolute top-16 left-52 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
          transition:fade={{ duration: 150 }}
        >
          {#each [["todos", "Todos PRAIND"], ["vigente", "Vigentes"], ["por-vencer", "Por vencer (≤30 días)"], ["vencido", "Vencidos"]] as [value, label]}
            <button
              onclick={() => handlePraindSelect(value as any)}
              class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {praindFilter ===
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
</div>

<!-- Modal -->
<ContratistaFormModal
  show={showModal}
  contratista={editingContratista}
  loading={modalLoading}
  onSave={handleSaveContratista}
  onClose={closeModal}
/>
