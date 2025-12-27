<!-- src/lib/components/proveedor/ProveedorListView.svelte -->
<script lang="ts">
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import ProveedorFormModal from "$lib/components/proveedor/ProveedorFormModal.svelte";
  import {
    fetchAllProveedores,
    fetchActiveProveedores,
    createProveedor,
    updateProveedor,
    deleteProveedor,
    changeStatus,
    getArchivedProveedores,
    restoreProveedor,
  } from "$lib/logic/proveedor/proveedorService";
  import { PROVEEDOR_COLUMNS } from "$lib/logic/proveedor/proveedorColumns";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import type {
    ProveedorResponse,
    CreateProveedorInput,
    UpdateProveedorInput,
  } from "$lib/types/proveedor";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";
  import { activeTabId } from "$lib/stores/tabs";

  interface Props {
    tabId?: string;
  }
  let { tabId = "proveedor-list" }: Props = $props();

  // Estado del Grid
  let proveedores = $state<ProveedorResponse[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  // Estado local de filtros
  // Toggle entre ver todos, solo activos, o papelera
  let activeFilter = $state<"todos" | "activos" | "papelera">("activos");

  // Estado del Modal
  let showModal = $state(false);
  let selectedProveedor = $state<ProveedorResponse | null>(null);
  let modalLoading = $state(false);

  // Selección
  let selectedRows = $state<ProveedorResponse[]>([]);

  // Keyboard shortcut handler for Ctrl+N
  function handleKeydown(e: KeyboardEvent) {
    if ($activeTabId !== tabId) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "n") {
      const target = e.target as HTMLElement;
      if (target.tagName === "TEXTAREA" || target.isContentEditable) return;
      e.preventDefault();
      openFormModal(null);
    }
  }

  // Carga inicial
  const loadData = async () => {
    loading = true;
    error = null;

    // Si el filtro es 'activos', usamos el servicio optimizado
    // Si es 'todos', traemos todo
    let res;
    if (activeFilter === "activos") {
      res = await fetchActiveProveedores();
    } else if (activeFilter === "papelera") {
      res = await getArchivedProveedores();
    } else {
      res = await fetchAllProveedores();
    }

    if (res.ok) {
      proveedores = res.data;
    } else {
      error = res.error;
      toast.error(res.error);
    }
    loading = false;
  };

  // Manejadores del Grid
  const handleRefresh = () => loadData();

  function openFormModal(proveedor: ProveedorResponse | null) {
    selectedProveedor = proveedor;
    showModal = true;
  }

  // Creación / Edición
  async function handleSave(data: CreateProveedorInput | UpdateProveedorInput) {
    modalLoading = true;
    let result;

    if (selectedProveedor) {
      result = await updateProveedor(
        selectedProveedor.id,
        data as UpdateProveedorInput,
      );
    } else {
      result = await createProveedor(data as CreateProveedorInput);
    }

    modalLoading = false;

    if (result.ok) {
      toast.success(
        selectedProveedor ? "Proveedor actualizado" : "Proveedor creado",
      );
      loadData(); // Recargar grid
      showModal = false;
      return true;
    } else {
      toast.error(result.error);
      return false;
    }
  }

  // Cambio de estado (usado en columna toggle)
  // Nota: PROVEEDOR_COLUMNS usa un formatter, pero la acción real debería ser via menu o botón custom.
  // Sin embargo, si queremos toggle desde toolbar o menu context:
  async function toggleStatus(proveedor: ProveedorResponse) {
    const newStatus = proveedor.estado === "ACTIVO" ? "INACTIVO" : "ACTIVO";
    const toastId = toast.loading(`Cambiando estado a ${newStatus}...`);

    const res = await changeStatus(proveedor.id, newStatus);

    if (res.ok) {
      toast.success("Estado actualizado", { id: toastId });
      // Optimistic update local
      proveedores = proveedores.map((p) =>
        p.id === proveedor.id
          ? {
              ...p,
              estado: res.data.estado,
              puedeIngresar: res.data.puedeIngresar,
            }
          : p,
      );
    } else {
      toast.error(res.error, { id: toastId });
    }
  }

  // Eliminar
  async function confirmDelete(proveedor: ProveedorResponse) {
    if (
      !confirm(`¿Estás seguro de eliminar al proveedor "${proveedor.nombre}"?`)
    )
      return;

    const res = await deleteProveedor(proveedor.id);
    if (res.ok) {
      toast.success("Proveedor eliminado");
      loadData();
    } else {
      toast.error(res.error);
    }
  }

  // Restaurar
  async function confirmRestore(proveedor: ProveedorResponse) {
    if (
      !confirm(`¿Estás seguro de restaurar al proveedor "${proveedor.nombre}"?`)
    )
      return;

    const res = await restoreProveedor(proveedor.id);
    if (res.ok) {
      toast.success("Proveedor restaurado");
      loadData();
    } else {
      toast.error(res.error);
    }
  }
  // Botones Custom
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    if (activeFilter === "papelera") {
      return {
        default: [
          {
            id: "filter-active",
            label: "Ver Activos", // Return to main view
            category: "ui",
            onClick: () => {
              activeFilter = "activos";
              loadData();
            },
            tooltip: "Volver a la lista de activos",
          },
          {
            id: "refresh",
            label: "Actualizar",
            category: "data",
            onClick: loadData,
            tooltip: "Recargar lista",
          },
        ],
        singleSelect: [
          {
            id: "restore",
            label: "Restaurar",
            category: "action",
            onClick: () => {
              if (selected) confirmRestore(selected);
            },
            tooltip: "Restaurar proveedor seleccionado",
          },
        ],
        multiSelect: [],
      };
    }

    return {
      default: [
        createCustomButton.nuevo(() => openFormModal(null)),
        {
          id: "refresh",
          label: "Actualizar",
          category: "data",
          onClick: loadData,
          tooltip: "Recargar lista",
        },
        {
          id: "filter-trash",
          label: "Papelera",
          category: "ui",
          onClick: () => {
            activeFilter = "papelera";
            loadData();
          },
          tooltip: "Ver proveedores eliminados",
        },
      ],
      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) openFormModal(selected);
        }),
        createCustomButton.eliminar(() => {
          if (selected) confirmDelete(selected);
        }),
      ],
      multiSelect: [],
    };
  });

  // Definición de columnas
  const columnDefs = $derived([
    ...PROVEEDOR_COLUMNS,
  ] as ColDef<ProveedorResponse>[]);

  // Effect para cargar datos al montar
  $effect(() => {
    loadData();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-full flex flex-col space-y-4 p-4 animate-fade-in bg-[#1e1e1e]">
  {#if loading}
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
          ></circle>
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
        <p class="mt-4 text-sm text-gray-400">Cargando proveedores...</p>
      </div>
    </div>
  {:else}
    <AGGridWrapper
      gridId="proveedor-list"
      rowData={proveedores}
      {columnDefs}
      {customButtons}
      onSelectionChanged={(rows) => {
        selectedRows = rows;
      }}
      getRowId={(params) => params.data.id}
    />
  {/if}
</div>

<ProveedorFormModal
  show={showModal}
  proveedor={selectedProveedor}
  loading={modalLoading}
  onSave={handleSave}
  onClose={() => (showModal = false)}
/>
