<!-- ==========================================
// src/lib/components/contratista/ContratistaListForm.svelte
// Componente visual con SearchBar de Tantivy + AG Grid
// ========================================== -->

<script lang="ts">
  import { fade } from "svelte/transition";
  import {
    RefreshCw,
    AlertCircle,
    CheckCircle2,
    Clock,
    XCircle,
    Filter,
    Download,
  } from "lucide-svelte";
  import type { ContratistaResponse, EstadoContratista } from "$lib/types/contratista";
  import type { SearchResult } from "$lib/types/search.types";
  import type {
    DataTableColumn,
    DataTableAction,
  } from "$lib/types/dataTable";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import DataTable from "$lib/components/common/DataTable.svelte";
  import ContratistaActionModal from "./ContratistaActionModal.svelte";
  import { searchStore } from "$lib/stores/searchStore";

  interface Props {
    contratistas: ContratistaResponse[];
    loading: boolean;
    error: string;
    onRefresh: () => void;
    onBlock: (data: {
      contratistaId: string;
      motivoBloqueo: string;
      observaciones?: string;
    }) => Promise<void>;
    onUnblock: (data: {
      id: string;
      motivoDesbloqueo: string;
      observaciones?: string;
    }) => Promise<void>;
    blockedContratistas: Set<string>;
  }

  let {
    contratistas = [],
    loading = false,
    error = "",
    onRefresh,
    onBlock,
    onUnblock,
    blockedContratistas = new Set(),
  }: Props = $props();

  // Estado local
  let estadoFilter = $state<"todos" | "activo" | "inactivo" | "suspendido">(
    "todos",
  );
  let praindFilter = $state<"todos" | "vigente" | "vencido" | "por-vencer">(
    "todos",
  );

  // Datos filtrados para la tabla
  let filteredData = $derived.by(() => {
    let filtered = contratistas;

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

    // Filtro por búsqueda de Tantivy
    // Si hay resultados de búsqueda, solo mostrar esos IDs
    if ($searchStore.results.length > 0) {
      const searchIds = new Set($searchStore.results.map((r) => r.id));
      filtered = filtered.filter((c) => searchIds.has(c.id));
    }

    return filtered;
  });

  // Estadísticas
  let stats = $derived({
    total: contratistas.length,
    activos: contratistas.filter((c) => c.estado === "activo").length,
    vencidos: contratistas.filter((c) => c.praindVencido).length,
    porVencer: contratistas.filter(
      (c) => !c.praindVencido && c.diasHastaVencimiento <= 30,
    ).length,
  });

  // Definición de columnas para AG Grid
  const columns: DataTableColumn<ContratistaResponse>[] = [
    {
      field: "cedula",
      headerName: "Cédula",
      width: 130,
      pinned: "left",
      cellStyle: { fontFamily: "monospace", fontSize: "13px" },
    },
    {
      field: "nombreCompleto",
      headerName: "Nombre Completo",
      flex: 1,
      minWidth: 200,
      cellStyle: { fontWeight: 500 },
    },
    {
      field: "empresaNombre",
      headerName: "Empresa",
      flex: 1,
      minWidth: 180,
    },
    {
      field: "estado",
      headerName: "Estado",
      width: 130,
      cellRenderer: (params) => {
        const estado = params.value as EstadoContratista;
        const badges: Record<EstadoContratista, { icon: string; class: string }> = {
          activo: {
            icon: "check-circle",
            class: "bg-green-500/10 text-green-400 border-green-500/20",
          },
          inactivo: {
            icon: "clock",
            class: "bg-gray-500/10 text-gray-400 border-gray-500/20",
          },
          suspendido: {
            icon: "x-circle",
            class: "bg-red-500/10 text-red-400 border-red-500/20",
          },
        };
        const badge = badges[estado] || badges.inactivo;
        return `
          <span class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium ${badge.class}">
            ${estado ? estado.charAt(0).toUpperCase() + estado.slice(1) : 'N/A'}
          </span>
        `;
      },
    },
    {
      field: "praindVencido",
      headerName: "PRAIND",
      width: 130,
      cellRenderer: (params) => {
        const row = params.data;
        if (!row) return "";

        let badgeClass = "";
        let text = "";

        if (row.praindVencido) {
          badgeClass = "bg-red-500/10 text-red-400 border-red-500/20";
          text = "Vencido";
        } else if (row.diasHastaVencimiento <= 30) {
          badgeClass = "bg-yellow-500/10 text-yellow-400 border-yellow-500/20";
          text = `${row.diasHastaVencimiento} días`;
        } else {
          badgeClass = "bg-green-500/10 text-green-400 border-green-500/20";
          text = "Vigente";
        }

        return `
          <span class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium ${badgeClass}">
            ${text}
          </span>
        `;
      },
    },
    {
      field: "fechaVencimientoPraind",
      headerName: "Vencimiento",
      width: 130,
      valueFormatter: (params) => {
        if (!params.value) return "";
        const date = new Date(params.value);
        return date.toLocaleDateString("es-PA", {
          year: "numeric",
          month: "short",
          day: "numeric",
        });
      },
    },
    {
      field: "puedeIngresar",
      headerName: "Acceso",
      width: 130,
      cellRenderer: (params) => {
        const canEnter = params.value;
        if (canEnter) {
          return `
            <span class="inline-flex items-center gap-1.5 rounded-full border border-green-500/20 bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-400">
              Permitido
            </span>
          `;
        } else {
          return `
            <span class="inline-flex items-center gap-1.5 rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium text-red-400">
              Denegado
            </span>
          `;
        }
      },
    },
  ];

  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    // Los resultados ya están en searchStore, el filtro se aplica automáticamente
    // No necesitamos hacer nada aquí
  }

  function handleSearchClear() {
    // searchStore se limpia automáticamente desde el SearchBar
    // No necesitamos hacer nada aquí
  }

  function clearAllFilters() {
    estadoFilter = "todos";
    praindFilter = "todos";
    handleSearchClear();
  }
</script>

<div class="flex h-full flex-col bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          Lista de Contratistas
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de todos los contratistas registrados
        </p>
      </div>
      <button
        on:click={onRefresh}
        disabled={loading}
        class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-all hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
      >
        <RefreshCw size={16} class={loading ? "animate-spin" : ""} />
        Actualizar
      </button>
    </div>

    <!-- Stats -->
    <div class="mt-4 grid grid-cols-4 gap-4">
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Total</div>
        <div class="mt-1 text-2xl font-semibold text-white">{stats.total}</div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Activos</div>
        <div class="mt-1 text-2xl font-semibold text-green-400">
          {stats.activos}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">PRAIND Vencido</div>
        <div class="mt-1 text-2xl font-semibold text-red-400">
          {stats.vencidos}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Por Vencer</div>
        <div class="mt-1 text-2xl font-semibold text-yellow-400">
          {stats.porVencer}
        </div>
      </div>
    </div>
  </div>

  <!-- Search & Filters Bar -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex flex-wrap items-center gap-4">
      <!-- Tantivy Search Bar -->
      <div class="flex-1 min-w-[300px]">
        <SearchBar
          placeholder="Buscar por nombre, cédula o empresa..."
          limit={10}
          on:select={handleSearchSelect}
          on:clear={handleSearchClear}
        />
      </div>

      <!-- Estado Filter -->
      <div class="flex items-center gap-2">
        <Filter size={16} class="text-gray-400" />
        <select
          bind:value={estadoFilter}
          class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="todos">Todos los estados</option>
          <option value="activo">Activos</option>
          <option value="inactivo">Inactivos</option>
          <option value="suspendido">Suspendidos</option>
        </select>
      </div>

      <!-- PRAIND Filter -->
      <select
        bind:value={praindFilter}
        class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
      >
        <option value="todos">Todos PRAIND</option>
        <option value="vigente">Vigentes</option>
        <option value="por-vencer">Por vencer (≤30 días)</option>
        <option value="vencido">Vencidos</option>
      </select>

      <!-- Clear Filters -->
      {#if estadoFilter !== "todos" || praindFilter !== "todos" || $searchStore.results.length > 0}
        <button
          on:click={clearAllFilters}
          class="flex items-center gap-2 rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-300"
        >
          <XCircle size={14} />
          Limpiar filtros
        </button>
      {/if}
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden">
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
          <RefreshCw size={32} class="mx-auto animate-spin text-blue-500" />
          <p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
        </div>
      </div>
    {:else if contratistas.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-600" />
          <p class="mt-4 text-lg font-medium text-gray-400">
            No hay contratistas registrados
          </p>
          <p class="mt-2 text-sm text-gray-500">
            Los contratistas aparecerán aquí una vez sean registrados
          </p>
        </div>
      </div>
    {:else}
      <!-- AG Grid DataTable -->
      <DataTable
        data={filteredData}
        {columns}
        storageKey="contratistas-list"
        rowSelection={false}
        pagination={true}
        paginationPageSize={20}
        paginationPageSizeSelector={[10, 20, 30, 50, 100]}
        getRowId={(row) => row.id}
        height="100%"
        toolbarConfig={{
          showColumnSelector: true,
          showExport: true,
          showAutoSize: true,
        }}
        exportConfig={{
          fileName: `contratistas-${new Date().toISOString().split("T")[0]}.csv`,
        }}
      />
    {/if}
  </div>
</div>