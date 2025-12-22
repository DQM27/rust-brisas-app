<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { AlertCircle, Filter } from "lucide-svelte";
  import type {
    ContratistaResponse,
    CreateContratistaInput,
    UpdateContratistaInput,
  } from "$lib/types/contratista";
  import type { ColDef } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";

  // Components
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import ContratistaFormModal from "./ContratistaFormModal.svelte";

  // Services
  import * as contratistaService from "$lib/logic/contratista/contratistaService";
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";
  import { reindexAllContratistas } from "$lib/api/searchService";

  // Logic
  import {
    createContratistaListLogic,
    ContratistaListLogic,
  } from "$lib/logic/contratista/contratistaColumns";
  import {
    createCustomButton,
    COMMON_DEFAULT_BUTTONS,
  } from "$lib/config/agGridConfigs";
  import { selectedSearchStore } from "$lib/stores/searchStore";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let blockedContratistas = $state<Set<string>>(new Set());
  let isUpdatingStatus = false;

  // Modal state
  let showModal = $state(false);
  let editingContratista = $state<ContratistaResponse | null>(null);
  let modalLoading = $state(false);

  // Lógica de presentación
  const listLogic = createContratistaListLogic();
  const listState = listLogic.getState();

  // Filtros
  let estadoFilter = $state<"todos" | "activo" | "inactivo" | "suspendido">(
    "todos",
  );
  let praindFilter = $state<"todos" | "vigente" | "por-vencer" | "vencido">(
    "todos",
  );

  // Dropdowns
  let showEstadoDropdown = $state(false);
  let showPraindDropdown = $state(false);

  // Selección
  let selectedRows = $state<ContratistaResponse[]>([]);

  // ==========================================
  // DERIVED STATE
  // ==========================================
  const filteredData = $derived.by(() => {
    const _search = $selectedSearchStore;
    listState.estadoFilter = estadoFilter;
    listState.praindFilter = praindFilter;
    return listLogic.getFilteredData(contratistas);
  });

  const estadoLabel = $derived(
    {
      todos: "Todos",
      activo: "Activos",
      inactivo: "Inactivos",
      suspendido: "Suspendidos",
    }[estadoFilter],
  );

  const praindLabel = $derived(
    {
      todos: "Todos",
      vigente: "Vigentes",
      "por-vencer": "Por vencer",
      vencido: "Vencidos",
    }[praindFilter],
  );

  const hasActiveFilters = $derived(
    estadoFilter !== "todos" || praindFilter !== "todos",
  );

  // Columnas
  const columnDefs = $derived.by((): ColDef<ContratistaResponse>[] => {
    const cols = ContratistaListLogic.getColumns(handleStatusChange);
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
    try {
      const result = await contratistaService.fetchAllContratistas();
      if (result.ok) {
        contratistas = result.data.contratistas;
        await loadBlockedContratistas();
      } else {
        error = result.error;
      }
    } catch (err) {
      console.error("Error al cargar contratistas:", err);
      error = "Error al cargar contratistas";
    }
    loading = false;
  }

  async function loadBlockedContratistas() {
    const result = await listaNegraService.fetchAll();
    if (result.ok) {
      const blocked = new Set<string>();
      result.data.bloqueados.forEach((b) => {
        if (b.isActive && b.contratistaId) {
          blocked.add(b.contratistaId);
        }
      });
      blockedContratistas = blocked;
    }
  }

  // ==========================================
  // HANDLERS - MODAL
  // ==========================================
  function openModal(contratista?: ContratistaResponse) {
    editingContratista = contratista || null;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    editingContratista = null;
  }

  async function handleSaveContratista(
    data: CreateContratistaInput | UpdateContratistaInput,
  ): Promise<boolean> {
    modalLoading = true;
    try {
      if (editingContratista) {
        // Modo edición
        const result = await contratistaService.updateContratista(
          editingContratista.id,
          {
            id: editingContratista.id,
            ...data,
          } as UpdateContratistaInput,
        );
        if (result.ok) {
          toast.success("Contratista actualizado");
          await loadContratistas();
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      } else {
        // Modo creación
        const result = await contratistaService.createContratista(
          data as CreateContratistaInput,
        );
        if (result.ok) {
          toast.success("Contratista registrado");
          await loadContratistas();
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      }
    } catch (err: any) {
      toast.error("Error inesperado");
      return false;
    } finally {
      modalLoading = false;
    }
  }

  // ==========================================
  // HANDLERS - STATUS
  // ==========================================
  async function handleStatusChange(id: string, currentStatus: string) {
    if (loading || isUpdatingStatus) return;
    try {
      isUpdatingStatus = true;
      const newStatus = currentStatus === "activo" ? "inactivo" : "activo";

      // Actualización optimista
      const oldContratistas = [...contratistas];
      contratistas = contratistas.map((c) =>
        c.id === id ? { ...c, estado: newStatus as any } : c,
      );

      const toastId = toast.loading("Actualizando estado...");
      const result = await contratistaService.changeEstado(
        id,
        newStatus as any,
      );

      if (result.ok) {
        toast.success("Estado actualizado", { id: toastId });
      } else {
        contratistas = oldContratistas;
        toast.error(result.error, { id: toastId });
      }
    } finally {
      isUpdatingStatus = false;
    }
  }

  // ==========================================
  // HANDLERS - FILTERS
  // ==========================================
  function handleEstadoSelect(value: string) {
    estadoFilter = value as any;
    showEstadoDropdown = false;
  }

  function handlePraindSelect(value: string) {
    praindFilter = value as any;
    showPraindDropdown = false;
  }

  function handleClearAllFilters() {
    estadoFilter = "todos";
    praindFilter = "todos";
    showEstadoDropdown = false;
    showPraindDropdown = false;
  }

  async function handleReindex() {
    const toastId = toast.loading("Reindexando...");
    try {
      await reindexAllContratistas();
      toast.success("Índice actualizado", { id: toastId });
    } catch (err) {
      toast.error("Error al reindexar", { id: toastId });
    }
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (
      !target.closest(".filter-dropdown-container") &&
      !target.closest("[data-filter-button]")
    ) {
      showEstadoDropdown = false;
      showPraindDropdown = false;
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================
  onMount(() => {
    loadContratistas();
  });
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          Lista de Contratistas
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de contratistas registrados
        </p>
      </div>
      <div class="flex-1 max-w-md">
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
            No hay contratistas registrados
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Los contratistas aparecerán aquí una vez sean registrados
          </p>
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
  </div>

  <!-- Filter Dropdowns -->
  <div class="filter-dropdown-container">
    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos los estados"], ["activo", "Activos"], ["inactivo", "Inactivos"], ["suspendido", "Suspendidos"]] as [value, label]}
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

    {#if showPraindDropdown}
      <div
        class="absolute top-16 left-52 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos PRAIND"], ["vigente", "Vigentes"], ["por-vencer", "Por vencer (≤30 días)"], ["vencido", "Vencidos"]] as [value, label]}
          <button
            onclick={() => handlePraindSelect(value)}
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

<!-- Modal -->
<ContratistaFormModal
  show={showModal}
  contratista={editingContratista}
  loading={modalLoading}
  onSave={handleSaveContratista}
  onClose={closeModal}
/>
