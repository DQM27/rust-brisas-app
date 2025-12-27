<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { Trash2, RotateCcw } from "lucide-svelte"; // Add RotateCcw for restore icon

  // ... (previous imports)

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let isUpdatingStatus = false;
  let viewMode = $state<"active" | "trash">("active"); // "active" or "trash"

  // ... (rest of simple state)

  // ==========================================
  // DERIVED STATE
  // ==========================================
  const filteredData = $derived.by(() => {
    const _search = $selectedSearchStore;
    // En modo papelera, ignoramos filtros de estado normal
    if (viewMode === "trash") {
      return listLogic.getFilteredData(contratistas).filter((c) => true); // Aplicar solo búsqueda
    }
    listState.estadoFilter = estadoFilter;
    listState.praindFilter = praindFilter;
    return listLogic.getFilteredData(contratistas);
  });

  // ... (labels)

  // Columnas
  const columnDefs = $derived.by((): ColDef<ContratistaResponse>[] => {
    // Si estamos en papelera, columnas simplificadas o con acciones diferentes
    const cols = ContratistaListLogic.getColumns(handleStatusChange);

    if (viewMode === "trash") {
      // En papelera ocultamos estado y acciones normales, agregamos DeletedAt si quisieramos
      return cols
        .filter((c) => c.field !== "estado" && c.colId !== "actions")
        .map((col) => ({
          field: String(col.field) as any,
          headerName: col.headerName,
          width: col.width,
          flex: col.flex,
          sortable: true,
          filter: true,
          resizable: true,
          valueFormatter: col.valueFormatter,
        }));
    }

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

    if (viewMode === "trash") {
      return {
        default: [
          {
            id: "back-to-active",
            label: "Volver a Activos",
            icon: undefined, // Standard back icon maybe?
            onClick: () => toggleViewMode(),
            variant: "default" as const,
          },
          ...COMMON_DEFAULT_BUTTONS.filter((b) =>
            ["autosize-all", "reset-columns"].includes(b.id),
          ).map((b) => ({
            id: b.id,
            label: b.label,
            icon: b.icon,
            tooltip: b.tooltip,
            onClick: undefined,
            useCommonHandler: true,
          })),
        ],
        singleSelect: [
          {
            id: "restore",
            label: "Restaurar",
            icon: RotateCcw,
            onClick: () => handleRestore(selected),
            variant: "default" as const,
          },
        ],
        multiSelect: [],
      };
    }

    return {
      default: [
        createCustomButton.nuevo(() => openModal()),
        {
          id: "view-trash",
          label: "Papelera",
          icon: Trash2,
          onClick: () => toggleViewMode(),
          variant: "ghost" as const, // Subtle button
          tooltip: "Ver contratistas eliminados",
        },
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
      let result;
      if (viewMode === "active") {
        result = await contratistaService.fetchAllContratistas();
      } else {
        result = await contratistaService.getArchivedContratistas();
      }

      if (result.ok) {
        // En list response viene .contratistas, en archived viene directo array
        contratistas =
          viewMode === "active"
            ? (result.data as any).contratistas
            : result.data;
      } else {
        error = result.error;
      }
    } catch (err) {
      console.error("Error al cargar contratistas:", err);
      error = "Error al cargar contratistas";
    }
    loading = false;
  }

  function toggleViewMode() {
    viewMode = viewMode === "active" ? "trash" : "active";
    loadContratistas();
  }

  // ... (modal handlers remain same)

  // ==========================================
  // HANDLERS - ACTIONS
  // ==========================================

  async function handleDelete(contratista: ContratistaResponse) {
    if (
      !confirm(
        `¿Estás seguro de eliminar a ${contratista.nombre_completo}? Se moverá a la papelera.`,
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

  async function handleRestore(contratista: ContratistaResponse) {
    const toastId = toast.loading("Restaurando...");
    const result = await contratistaService.restoreContratista(contratista.id);

    if (result.ok) {
      toast.success("Contratista restaurado", { id: toastId });
      loadContratistas();
    } else {
      toast.error(result.error, { id: toastId });
    }
  }

  // ... (rest of file)

  // HEADER CHANGE
  // <h2 class="text-xl font-semibold text-gray-100">
  //     {viewMode === 'active' ? 'Lista de Contratistas' : 'Papelera de Reciclaje'}
  // </h2>

  // ... (rest of file)

  // ==========================================
  // LIFECYCLE
  // ==========================================
  onMount(() => {
    loadContratistas();
  });
</script>

<svelte:window onclick={handleClickOutside} onkeydown={handleKeydown} />

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
            No hay contratistas
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Crea el primer contratista para comenzar
          </p>
          <button
            onclick={() => openModal()}
            class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Nuevo Contratista
          </button>
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
