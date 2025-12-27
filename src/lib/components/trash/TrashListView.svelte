<script lang="ts">
  import { onMount, tick } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { RotateCcw, ArrowLeft, AlertCircle } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { COMMON_DEFAULT_BUTTONS } from "$lib/config/agGridConfigs";
  import type { GridId } from "$lib/types/agGrid";
  import type { TrashService, TrashItem } from "$lib/logic/trash/trashService";
  import TrashFormModal from "./TrashFormModal.svelte";

  interface Props<T extends TrashItem> {
    title?: string;
    service: TrashService<T>;
    columnDefs: ColDef<T>[];
    gridId: GridId;
    onBack: () => void;
    rowIdField?: string;
    entityName?: string;
  }

  let {
    title = "Papelera de Reciclaje",
    service,
    columnDefs,
    gridId,
    onBack,
    rowIdField = "id",
    entityName = "Elemento",
  }: Props<any> = $props();

  // State
  let items = $state<any[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selectedRows = $state<any[]>([]);

  // Modal State
  let showModal = $state(false);
  let modalLoading = $state(false);
  let modalAction = $state<"restore" | "delete" | null>(null);
  let itemToProcess = $state<any | null>(null);

  // Derived Buttons
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];
    return {
      default: [
        {
          id: "back-to-active",
          label: "Volver",
          icon: ArrowLeft,
          onClick: onBack,
          variant: "default" as const,
        },
        ...COMMON_DEFAULT_BUTTONS.filter((b) =>
          ["autosize-all", "reset-columns"].includes(b.id),
        ).map((b) => ({
          id: b.id,
          label: b.label,
          icon: b.icon,
          tooltip: b.tooltip,
          onClick: undefined, // Handled by AGGridWrapper common handlers
          useCommonHandler: true,
        })),
      ],
      singleSelect: [
        {
          id: "restore",
          label: "Restaurar",
          icon: RotateCcw,
          onClick: () => confirmRestore(selected),
          variant: "default" as const,
        },
      ],
      multiSelect: [],
    };
  });

  // Actions
  async function loadArchived() {
    loading = true;
    error = "";
    try {
      const result = await service.getArchived();
      if (result.ok) {
        items = result.data;
      } else {
        error = result.error || "Error desconocido";
      }
    } catch (err) {
      console.error(err);
      error = "Error al cargar elementos eliminados";
    }
    loading = false;
  }

  function confirmRestore(item: any) {
    if (!item) return;
    itemToProcess = item;
    modalAction = "restore";
    showModal = true;
  }

  async function handleConfirmAction() {
    if (!itemToProcess || !modalAction) return;

    const id = itemToProcess[rowIdField];
    modalLoading = true;

    try {
      if (modalAction === "restore") {
        const result = await service.restore(id);
        if (result.ok) {
          toast.success(`${entityName} restaurado`);
          await loadArchived();
          showModal = false;
          itemToProcess = null;
        } else {
          toast.error(result.error || "Error al restaurar");
        }
      }
      // Future expansion for delete
    } catch (e) {
      toast.error("Error inesperado");
    }

    modalLoading = false;
  }

  onMount(() => {
    loadArchived();
  });
</script>

<div class="h-full flex flex-col">
  <div class="flex-1 overflow-hidden relative">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading && items.length === 0}
      <!-- Loading state only when no items initially to avoid flickering on reloads if not desired, or just keep generic-->
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <div
            class="animate-spin rounded-full h-8 w-8 border-b-2 border-white mx-auto mb-4"
          ></div>
          <p class="text-gray-400">Cargando eliminados...</p>
        </div>
      </div>
    {:else if items.length === 0 && !loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <p class="mt-4 text-lg font-medium text-gray-300">Papelera vacía</p>
          <p class="mt-2 text-sm text-gray-400">
            No hay {entityName.toLowerCase()}s eliminados recientemente
          </p>
          <button
            onclick={onBack}
            class="mt-6 px-4 py-2 bg-[#2d2d2d] border border-white/10 text-gray-300 rounded-md hover:bg-[#353535] transition-colors flex items-center gap-2 mx-auto"
          >
            <ArrowLeft size={16} />
            Volver
          </button>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        {gridId}
        {columnDefs}
        rowData={items}
        {customButtons}
        getRowId={(params) => params.data[rowIdField]}
        persistenceKey={`${gridId}-columns`}
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>
</div>

<TrashFormModal
  show={showModal}
  item={itemToProcess}
  action={modalAction}
  {entityName}
  loading={modalLoading}
  onConfirm={handleConfirmAction}
  onClose={() => (showModal = false)}
/>
