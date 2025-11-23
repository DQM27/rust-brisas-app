<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import {
    Search,
    Filter,
    RefreshCw,
    AlertCircle,
    CheckCircle2,
    Clock,
    XCircle,
  } from "lucide-svelte";
  import type { ContratistaResponse } from "$lib/types/contratista";
  import ContratistaActionModal from "./ContratistaActionModal.svelte";

  export let contratistas: ContratistaResponse[] = [];
  export let loading = false;
  export let error = "";
  export let onRefresh: () => void;
  export let onBlock: (data: {
    contratistaId: string;
    motivoBloqueo: string;
    observaciones?: string;
  }) => Promise<void>;
  export let onUnblock: (data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) => Promise<void>;
  export let blockedContratistas: Set<string> = new Set();

  // Filtros
  let searchTerm = "";
  let estadoFilter: "todos" | "activo" | "inactivo" | "suspendido" = "todos";
  let praindFilter: "todos" | "vigente" | "vencido" | "por-vencer" = "todos";

  // Contratistas filtrados
  $: filteredContratistas = contratistas.filter((c) => {
    // Filtro de búsqueda
    const searchLower = searchTerm.toLowerCase();
    const matchesSearch =
      !searchTerm ||
      c.nombreCompleto.toLowerCase().includes(searchLower) ||
      c.cedula.includes(searchTerm) ||
      c.empresaNombre.toLowerCase().includes(searchLower);

    // Filtro de estado
    const matchesEstado = estadoFilter === "todos" || c.estado === estadoFilter;

    // Filtro de PRAIND
    let matchesPraind = true;
    if (praindFilter === "vigente") {
      matchesPraind = !c.praindVencido && c.diasHastaVencimiento > 30;
    } else if (praindFilter === "vencido") {
      matchesPraind = c.praindVencido;
    } else if (praindFilter === "por-vencer") {
      matchesPraind = !c.praindVencido && c.diasHastaVencimiento <= 30;
    }

    return matchesSearch && matchesEstado && matchesPraind;
  });

  // Estadísticas
  $: stats = {
    total: contratistas.length,
    activos: contratistas.filter((c) => c.estado === "activo").length,
    vencidos: contratistas.filter((c) => c.praindVencido).length,
    porVencer: contratistas.filter(
      (c) => !c.praindVencido && c.diasHastaVencimiento <= 30,
    ).length,
  };

  function getEstadoBadgeClass(estado: string): string {
    switch (estado) {
      case "activo":
        return "bg-green-500/10 text-green-400 border-green-500/20";
      case "inactivo":
        return "bg-gray-500/10 text-gray-400 border-gray-500/20";
      case "suspendido":
        return "bg-red-500/10 text-red-400 border-red-500/20";
      default:
        return "bg-gray-500/10 text-gray-400 border-gray-500/20";
    }
  }

  function getPraindBadgeClass(contratista: ContratistaResponse): string {
    if (contratista.praindVencido) {
      return "bg-red-500/10 text-red-400 border-red-500/20";
    } else if (contratista.diasHastaVencimiento <= 30) {
      return "bg-yellow-500/10 text-yellow-400 border-yellow-500/20";
    }
    return "bg-green-500/10 text-green-400 border-green-500/20";
  }

  function getPraindText(contratista: ContratistaResponse): string {
    if (contratista.praindVencido) {
      return "Vencido";
    } else if (contratista.diasHastaVencimiento <= 30) {
      return `${contratista.diasHastaVencimiento} días`;
    }
    return "Vigente";
  }

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleDateString("es-PA", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
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

  <!-- Filters -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex flex-wrap items-center gap-4">
      <!-- Search -->
      <div class="relative flex-1 min-w-[300px]">
        <Search
          size={18}
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
        />
        <input
          type="text"
          bind:value={searchTerm}
          placeholder="Buscar por nombre, cédula o empresa..."
          class="w-full rounded-lg border border-white/10 bg-[#1e1e1e] py-2 pl-10 pr-4 text-sm text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
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
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-auto p-6">
    {#if error}
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
    {:else if loading}
      <div class="flex items-center justify-center py-12">
        <div class="text-center">
          <RefreshCw size={32} class="mx-auto animate-spin text-blue-500" />
          <p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
        </div>
      </div>
    {:else if filteredContratistas.length === 0}
      <div class="flex items-center justify-center py-12">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-600" />
          <p class="mt-4 text-lg font-medium text-gray-400">
            No se encontraron contratistas
          </p>
          <p class="mt-2 text-sm text-gray-500">
            {searchTerm || estadoFilter !== "todos" || praindFilter !== "todos"
              ? "Intenta ajustar los filtros de búsqueda"
              : "No hay contratistas registrados en el sistema"}
          </p>
        </div>
      </div>
    {:else}
      <div class="overflow-hidden rounded-lg ring-1 ring-white/10">
        <table class="w-full">
          <thead class="bg-[#252526]">
            <tr>
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Cédula</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Nombre Completo</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Empresa</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Estado</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >PRAIND</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Vencimiento</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Acceso</th
              >
              <th
                class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400"
                >Acciones</th
              >
            </tr>
          </thead>
          <tbody class="divide-y divide-white/5 bg-[#1e1e1e]">
            {#each filteredContratistas as contratista (contratista.id)}
              <tr
                class="transition-colors hover:bg-white/5"
                transition:fly={{ y: 10, duration: 200 }}
              >
                <td class="px-4 py-3 text-sm font-mono text-gray-300">
                  {contratista.cedula}
                </td>
                <td class="px-4 py-3 text-sm font-medium text-white">
                  {contratista.nombreCompleto}
                </td>
                <td class="px-4 py-3 text-sm text-gray-400">
                  {contratista.empresaNombre}
                </td>
                <td class="px-4 py-3">
                  <span
                    class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium {getEstadoBadgeClass(
                      contratista.estado,
                    )}"
                  >
                    {#if contratista.estado === "activo"}
                      <CheckCircle2 size={12} />
                    {:else if contratista.estado === "suspendido"}
                      <XCircle size={12} />
                    {:else}
                      <Clock size={12} />
                    {/if}
                    {contratista.estado.charAt(0).toUpperCase() +
                      contratista.estado.slice(1)}
                  </span>
                </td>
                <td class="px-4 py-3">
                  <span
                    class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium {getPraindBadgeClass(
                      contratista,
                    )}"
                  >
                    {#if contratista.praindVencido}
                      <XCircle size={12} />
                    {:else if contratista.diasHastaVencimiento <= 30}
                      <AlertCircle size={12} />
                    {:else}
                      <CheckCircle2 size={12} />
                    {/if}
                    {getPraindText(contratista)}
                  </span>
                </td>
                <td class="px-4 py-3 text-sm text-gray-400">
                  {formatDate(contratista.fechaVencimientoPraind)}
                </td>
                <td class="px-4 py-3">
                  {#if contratista.puedeIngresar}
                    <span
                      class="inline-flex items-center gap-1.5 rounded-full border border-green-500/20 bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-400"
                    >
                      <CheckCircle2 size={12} />
                      Permitido
                    </span>
                  {:else}
                    <span
                      class="inline-flex items-center gap-1.5 rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium text-red-400"
                    >
                      <XCircle size={12} />
                      Denegado
                    </span>
                  {/if}
                </td>
                <td class="px-4 py-3">
                  <ContratistaActionModal
                    {contratista}
                    isBlocked={blockedContratistas.has(contratista.id)}
                    {onBlock}
                    {onUnblock}
                  />
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- Results count -->
      <div class="mt-4 text-center text-sm text-gray-500">
        Mostrando {filteredContratistas.length} de {contratistas.length} contratistas
      </div>
    {/if}
  </div>
</div>
