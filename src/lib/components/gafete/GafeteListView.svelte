<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";

  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { createCustomButton } from "$lib/config/agGridConfigs";

  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import * as alertaGafeteService from "$lib/logic/alertaGafete/alertaGafeteService";
  import { GafeteColumns } from "$lib/logic/gafete/gafeteColumns";

  import type { GafeteResponse } from "$lib/types/gafete";
  import { Plus } from "lucide-svelte";
  import { currentUser } from "$lib/stores/auth";

  // Modales
  import GafeteFormModal from "./GafeteFormModal.svelte";
  import ResolveAlertModal from "./modals/ResolveAlertModal.svelte";
  import BulkCreateGafeteModal from "./modals/BulkCreateGafeteModal.svelte";

  // Estado
  let gafetes = $state<GafeteResponse[]>([]);
  let loading = $state(false);
  let showModal = $state(false);
  let showBulkModal = $state(false);
  let selectedGafete = $state<GafeteResponse | null>(null);
  let formLoading = $state(false);

  // Estado para modal de resolución de alertas
  let showResolveModal = $state(false);
  let selectedAlertGafete = $state<GafeteResponse | null>(null);

  // Definición de columnas usando la clase especializada
  const columnDefs = $derived(
    GafeteColumns.getColumns({
      onResolve: (data) => handleResolve(data),
      onRecover: (data) => changeStatus(data, "activo"),
      onLost: (data) => changeStatus(data, "extraviado"),
      onDamage: (data) => changeStatus(data, "danado"),
      onDelete: (data) => handleDelete(data),
      onEdit: (data) => handleEdit(data),
    }),
  );

  // Custom buttons para la toolbar
  const customButtons = $derived.by(() => {
    try {
      if (!createCustomButton) return {};
      return {
        default: [
          createCustomButton.nuevo(() => handleNew()),
          {
            id: "bulk-create",
            label: "Generar Lote",
            icon: Plus,
            onClick: () => {
              showBulkModal = true;
            },
            classes: "bg-blue-600 hover:bg-blue-700 text-white",
            tooltip: "Generar múltiples gafetes",
          },
        ],
      };
    } catch (err) {
      console.error("Error generating customButtons:", err);
      return {};
    }
  });

  // ==========================================
  // CARGA DE DATOS
  // ==========================================
  async function loadGafetes() {
    loading = true;
    try {
      const result = await gafeteService.fetchAll();
      if (result.ok) {
        gafetes = result.data.gafetes;
      } else {
        toast.error(result.error);
      }
    } catch (err: any) {
      toast.error("Error inesperado al cargar gafetes");
    } finally {
      loading = false;
    }
  }

  // ==========================================
  // MANEJADORES DE ACCIONES
  // ==========================================
  function handleNew() {
    selectedGafete = null;
    showModal = true;
  }

  function handleEdit(gafete: GafeteResponse) {
    selectedGafete = gafete;
    showModal = true;
  }

  function handleResolve(gafete: GafeteResponse) {
    selectedAlertGafete = gafete;
    showResolveModal = true;
  }

  async function changeStatus(data: GafeteResponse, newStatus: string) {
    try {
      if (!data) return;
      loading = true;
      const userId = $currentUser?.id;
      const result = await gafeteService.updateStatus(
        data.numero,
        data.tipo,
        newStatus,
        userId,
      );
      if (result.ok) {
        toast.success(`Estado actualizado a ${newStatus}`);
        await loadGafetes();
      } else {
        toast.error(result.error);
      }
    } catch (error: any) {
      toast.error(error.message || "Error al cambiar el estado del gafete.");
    } finally {
      loading = false;
    }
  }

  async function handleResolveSubmit(notas: string) {
    if (!selectedAlertGafete?.alertaId) {
      toast.error("No se encontró el ID de la alerta");
      return;
    }

    formLoading = true;
    const userId = $currentUser?.id;
    const result = await alertaGafeteService.resolverAlerta(
      selectedAlertGafete.alertaId,
      notas,
      userId,
    );

    if (result.ok) {
      toast.success("Alerta resuelta correctamente");
      showResolveModal = false;
      selectedAlertGafete = null;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  async function handleDelete(gafete: GafeteResponse) {
    if (
      !confirm(
        `¿Estás seguro de eliminar PERMANENTEMENTE el gafete ${gafete.numero}?`,
      )
    )
      return;

    const result = await gafeteService.remove(gafete.numero, gafete.tipo);
    if (result.ok) {
      toast.success("Gafete eliminado");
      loadGafetes();
    } else {
      toast.error(result.error);
    }
  }

  async function handleFormSubmit(data: any) {
    formLoading = true;
    let result;

    if (selectedGafete) {
      result = await gafeteService.update(
        selectedGafete.numero,
        selectedGafete.tipo,
        data,
      );
    } else {
      result = await gafeteService.create(data);
    }

    if (result.ok) {
      toast.success(selectedGafete ? "Gafete actualizado" : "Gafete creado");
      showModal = false;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  async function handleBulkSubmit(data: any) {
    formLoading = true;
    const result = await gafeteService.createRange(data);

    if (result.ok) {
      toast.success(`Se generaron ${result.data.length} gafetes correctamente`);
      showBulkModal = false;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  onMount(() => {
    loadGafetes();
  });
</script>

<!-- ========================================== -->
<!-- LAYOUT -->
<!-- ========================================== -->
<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">Gestión de Gafetes</h2>
        <p class="mt-1 text-sm text-gray-400">
          Administración de inventario, estado físico y alertas
        </p>
      </div>

      <div class="flex gap-4 text-xs text-gray-400">
        <div class="flex items-center">
          <span class="w-2 h-2 rounded-full bg-emerald-500 mr-1"></span> Disponibles
        </div>
        <div class="flex items-center">
          <span class="w-2 h-2 rounded-full bg-blue-500 mr-1"></span> En Uso
        </div>
        <div class="flex items-center">
          <span class="w-2 h-2 rounded-full bg-rose-500 mr-1"></span> Dañados
        </div>
      </div>
    </div>
  </div>

  <div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
    <AGGridWrapper
      gridId="badges-list"
      rowData={gafetes}
      {columnDefs}
      {customButtons}
      getRowId={(params) => params.data.numero}
    />
  </div>
</div>

<!-- ========================================== -->
<!-- MODALES -->
<!-- ========================================== -->
{#if showModal}
  <GafeteFormModal
    show={showModal}
    gafete={selectedGafete}
    loading={formLoading}
    onSave={handleFormSubmit}
    onClose={() => {
      showModal = false;
      selectedGafete = null;
    }}
  />
{/if}

{#if showBulkModal}
  <BulkCreateGafeteModal
    show={showBulkModal}
    loading={formLoading}
    onSave={handleBulkSubmit}
    onClose={() => {
      showBulkModal = false;
    }}
  />
{/if}

{#if showResolveModal && selectedAlertGafete}
  <ResolveAlertModal
    show={showResolveModal}
    gafeteNumero={selectedAlertGafete.numero}
    nombrePersona={selectedAlertGafete.quienPerdio || "Desconocido"}
    fechaReporte={selectedAlertGafete.fechaPerdido || new Date().toISOString()}
    loading={formLoading}
    onResolve={handleResolveSubmit}
    onCancel={() => {
      showResolveModal = false;
      selectedAlertGafete = null;
    }}
  />
{/if}
