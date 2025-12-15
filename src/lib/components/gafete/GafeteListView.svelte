<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";

  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { createCustomButton } from "$lib/config/agGridConfigs";

  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import * as alertaGafeteService from "$lib/logic/alertaGafete/alertaGafeteService";

  import type { GafeteResponse } from "$lib/types/gafete";
  import { Plus } from "lucide-svelte";
  import { currentUser } from "$lib/stores/auth";
  import GafeteForm from "./GafeteForm.svelte";
  import ResolveAlertModal from "./ResolveAlertModal.svelte";
  import BulkCreateGafeteModal from "./BulkCreateGafeteModal.svelte";

  // Estado
  let gafetes = $state<GafeteResponse[]>([]);
  let loading = $state(false);
  let showModal = $state(false);
  let showBulkModal = $state(false); // Estado para modal masivo
  let selectedGafete = $state<GafeteResponse | null>(null);
  let formLoading = $state(false);

  // Estado para modal de resolución de alertas
  let showResolveModal = $state(false);
  let selectedAlertGafete = $state<GafeteResponse | null>(null);

  // Debug check
  $effect(() => {
    console.log("GafeteListView: mounted");
    console.log("createCustomButton:", createCustomButton);
  });

  // Custom buttons para la toolbar
  const customButtons = $derived.by(() => {
    try {
      if (!createCustomButton) {
        console.error("createCustomButton is undefined!");
        return {};
      }
      return {
        default: [
          createCustomButton.nuevo(() => {
            handleNew();
          }),
          {
            id: "bulk-create",
            label: "Generar Lote",
            icon: Plus, // Using Plus as a placeholder or import a specific one like 'Layers' or 'Copy' if available, but Plus is safe
            onClick: () => {
              showBulkModal = true;
            },
            classes: "bg-blue-600 hover:bg-blue-700 text-white",
            tooltip: "Generar múltiples gafetes",
            // Note: 'action' was wrong, should be 'onClick'. 'name' -> 'label'. Added 'id'.
          },
        ],
      };
    } catch (err) {
      console.error("Error generating customButtons:", err);
      return {};
    }
  });

  // ==========================================
  // COLUMNAS AG GRID
  // ==========================================
  const columnDefs: ColDef<GafeteResponse>[] = [
    {
      field: "numero",
      headerName: "Número",
      sortable: true,
      filter: true,
      cellStyle: { fontWeight: "bold" },
    },

    // ========= TIPO (con badges estilo GitHub) =========
    {
      field: "tipoDisplay",
      headerName: "Tipo",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const tipo = params.data.tipo;
        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";
        let colorClass = "";

        switch (tipo) {
          case "contratista":
            colorClass =
              "bg-indigo-50 text-indigo-700 border-indigo-200 dark:bg-indigo-900/30 dark:text-indigo-300 dark:border-indigo-800";
            break;
          case "proveedor":
            colorClass =
              "bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800";
            break;
          case "visita":
            colorClass =
              "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
            break;
          default:
            colorClass =
              "bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700";
        }

        return `<span class="${baseClass} ${colorClass}">${params.value}</span>`;
      },
    },

    // ========= ESTADO =========
    {
      field: "status",
      headerName: "Estado",
      sortable: true,
      filter: true,
      width: 140,
      cellRenderer: (params: any) => {
        const status = params.value;
        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border uppercase tracking-wide";

        let classes = "";
        let icon = "";
        let label = "";

        switch (status) {
          case "disponible":
            classes =
              "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
            icon = "✔";
            label = "Disponible";
            break;
          case "en_uso":
            classes =
              "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800";
            icon = "◉";
            label = "En Uso";
            break;
          case "perdido":
            classes =
              "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
            icon = "✖";
            label = "Perdido";
            break;
          case "danado":
            classes =
              "bg-rose-100 text-rose-800 border-rose-300 dark:bg-rose-900/50 dark:text-rose-200 dark:border-rose-700";
            icon = "⚡";
            label = "Dañado";
            break;
          case "extraviado":
            classes =
              "bg-amber-100 text-amber-800 border-amber-300 dark:bg-amber-900/50 dark:text-amber-200 dark:border-amber-700";
            icon = "❓";
            label = "Extraviado";
            break;
          default:
            classes =
              "bg-gray-100 text-gray-800 border-gray-300 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600";
            icon = "-";
            label = status;
        }

        return `<span class="${baseClass} ${classes}"><span class="mr-1 opacity-75">${icon}</span> ${label}</span>`;
      },
    },

    // ========= FECHA PERDIDO =========
    {
      field: "fechaPerdido",
      headerName: "Fecha Perdido",
      sortable: true,
      filter: true,
      width: 130,
      valueFormatter: (params: any) => {
        if (!params.value) return "";
        return new Date(params.value).toLocaleDateString();
      },
    },
    {
      field: "fechaPerdido",
      headerName: "Hora Reporte",
      sortable: true,
      filter: true,
      width: 120,
      valueFormatter: (params: any) => {
        if (!params.value) return "";
        return new Date(params.value).toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        });
      },
    },
    {
      field: "quienPerdio",
      headerName: "Persona que Perdió",
      sortable: true,
      filter: true,
      width: 180,
    },
    {
      field: "reportadoPorNombre",
      headerName: "Reportado Por (Usuario)",
      sortable: true,
      filter: true,
      width: 160,
    },

    {
      field: "resueltoPorNombre",
      headerName: "Resuelto Por",
      sortable: true,
      filter: true,
      width: 150,
    },
    {
      field: "fechaResolucion",
      headerName: "Fecha Resolución",
      sortable: true,
      filter: true,
      width: 160,
      valueFormatter: (params: any) => {
        if (!params.value) return "";
        return new Date(params.value).toLocaleDateString();
      },
    },
    {
      field: "fechaResolucion",
      headerName: "Hora Resolución",
      sortable: true,
      filter: true,
      width: 120,
      valueFormatter: (params: any) => {
        if (!params.value) return "";
        return new Date(params.value).toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        });
      },
    },
    {
      field: "notas", // Notas de la alerta
      headerName: "Notas Alerta",
      sortable: true,
      filter: true,
      width: 200,
    },

    // ========= ACCIONES =========
    {
      headerName: "Acciones",
      width: 250,
      cellRenderer: (params: any) => {
        const status = params.data.status;
        const numero = params.data.numero;

        let buttons = "";

        // Botón Resolver Alerta (si está perdido por alerta)
        if (status === "perdido") {
          buttons += `
            <button class="mr-2 px-2 py-1 bg-green-100 text-green-700 rounded hover:bg-green-200 text-xs font-medium resolve-btn">
              ✓ Resolver
            </button>
          `;
        }

        // Botones de estado físico (si no tiene alerta activa)
        if (status !== "perdido") {
          // Si está extraviado -> Recuperar
          if (status === "extraviado") {
            buttons += `
              <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn">
                ↻ Recuperar
              </button>
            `;
          } else if (status !== "danado") {
            // Si NO está dañado -> Marcar Extraviado
            buttons += `
              <button class="mr-2 px-2 py-1 bg-amber-100 text-amber-700 rounded hover:bg-amber-200 text-xs font-medium lost-btn" title="Marcar como Extraviado">
                ❓ Extraviado
              </button>
            `;
            // Si NO está dañado -> Marcar Dañado
            buttons += `
              <button class="mr-2 px-2 py-1 bg-rose-100 text-rose-700 rounded hover:bg-rose-200 text-xs font-medium damage-btn" title="Marcar como Dañado">
                ⚡ Dañado
              </button>
            `;
          }

          // Si está dañado -> Eliminar o Recuperar (Reparado)
          if (status === "danado") {
            buttons += `
              <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn" title="Marcar como Reparado/Activo">
                ↻ Reparado
              </button>
            `;
          }

          if (status === "danado" || status === "disponible") {
            // Permitir eliminar disponibles también si fue error
            buttons += `
                <button class="px-2 py-1 bg-gray-100 text-gray-700 rounded hover:bg-red-100 hover:text-red-700 text-xs font-medium delete-btn" title="Eliminar del inventario">
                  🗑️
                </button>
              `;
          }
        }

        return buttons || `<span class="text-xs text-gray-400">-</span>`;
      },
      onCellClicked: (params: any) => {
        const event = params.event;
        const data = params.data;

        if (event.target.classList.contains("resolve-btn")) {
          handleResolve(data);
        } else if (event.target.classList.contains("recover-btn")) {
          changeStatus(data, "activo");
        } else if (event.target.classList.contains("lost-btn")) {
          changeStatus(data, "extraviado");
        } else if (event.target.classList.contains("damage-btn")) {
          changeStatus(data, "danado");
        } else if (event.target.classList.contains("delete-btn")) {
          handleDelete(data);
        }
      },
    },
  ];

  // ==========================================
  // Cargar datos
  // ==========================================
  async function loadGafetes() {
    loading = true;
    try {
      const result = await gafeteService.fetchAll();
      if (result.ok) {
        gafetes = result.data.gafetes;
      } else {
        console.error("loadGafetes failed:", result.error);
        toast.error(result.error);
      }
    } catch (err) {
      console.error("loadGafetes exception:", err);
      toast.error("Error inesperado al cargar gafetes");
    } finally {
      loading = false;
    }
  }

  // ==========================================
  // Manejadores
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
      // Pass userId to capture who resolved the alert (if applicable)
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
      console.error("Error changing status:", error);
      toast.error(error.message || "Error al cambiar el estado del gafete.");
    } finally {
      loading = false;
    }
  }

  async function handleResolveSubmit(notas: string, fechaDevolucion: string) {
    if (!selectedAlertGafete?.alertaId) {
      toast.error("No se encontró el ID de la alerta");
      return;
    }

    formLoading = true;

    const userId = $currentUser?.id;
    // Nota: El usuario pidió remover la fecha redundante de la nota
    // porque ya existen columnas de fecha. Solo guardamos las notas.
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

  async function handleFormSubmit(event: CustomEvent) {
    formLoading = true;
    const data = event.detail;
    let result;

    if (selectedGafete) {
      // Para update, pasamos el tipo ACTUAL (selectedGafete.tipo) Y el payload (que puede traer nuevo tipo)
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

  async function handleBulkSubmit(event: CustomEvent) {
    formLoading = true;
    const data = event.detail;
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
      <!-- Stats rápidos (Opcional) -->
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
<!-- MODAL SINGLE -->
<!-- ========================================== -->
{#if showModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade
  >
    <GafeteForm
      initialData={selectedGafete}
      loading={formLoading}
      on:submit={handleFormSubmit}
      on:cancel={() => {
        showModal = false;
      }}
    />
  </div>
{/if}

<!-- ========================================== -->
<!-- MODAL BULK -->
<!-- ========================================== -->
{#if showBulkModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade
  >
    <BulkCreateGafeteModal
      loading={formLoading}
      on:submit={handleBulkSubmit}
      on:cancel={() => {
        showBulkModal = false;
      }}
    />
  </div>
{/if}

<!-- ========================================== -->
<!-- MODAL RESOLVER ALERTA -->
<!-- ========================================== -->
{#if showResolveModal && selectedAlertGafete}
  <ResolveAlertModal
    show={showResolveModal}
    gafeteNumero={selectedAlertGafete.numero}
    nombrePersona={selectedAlertGafete.quienPerdio || "Desconocido"}
    fechaReporte={selectedAlertGafete.fechaPerdido || new Date().toISOString()}
    onResolve={handleResolveSubmit}
    onCancel={() => (showResolveModal = false)}
  />
{/if}
