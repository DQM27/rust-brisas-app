<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import {
    Search,
    Filter,
    RefreshCw,
    AlertCircle,
    UserX,
    Plus,
  } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";
  import Listanegraform from "./Listanegraform.svelte";
  import ListaNegraTable from "./ListaNegraTable.svelte";

  export let bloqueados: ListaNegraResponse[] = [];
  export let loading = false;
  export let error = "";
  export let onRefresh: () => void;
  export let onAddToBlacklist: (data: any) => Promise<void>;
  export let onUnblock: (data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) => Promise<void>;

  // Modal state
  let showAddModal = false;
  let showUnblockModal = false;
  let selectedBloqueado: ListaNegraResponse | null = null;
  let addFormLoading = false;

  // Filtros
  let searchTerm = "";
  let estadoFilter: "todos" | "activo" | "inactivo" = "todos";
  let tipoFilter: "todos" | "permanente" | "temporal" = "todos";

  // Bloqueados filtrados
  $: filteredBloqueados = bloqueados.filter((b) => {
    const searchLower = searchTerm.toLowerCase();
    const matchesSearch =
      !searchTerm ||
      b.nombreCompleto.toLowerCase().includes(searchLower) ||
      b.cedula.includes(searchTerm) ||
      (b.empresaNombre && b.empresaNombre.toLowerCase().includes(searchLower));

    const matchesEstado =
      estadoFilter === "todos" ||
      (estadoFilter === "activo" && b.isActive) ||
      (estadoFilter === "inactivo" && !b.isActive);

    let matchesTipo = true;
    if (tipoFilter === "permanente") {
      matchesTipo = b.esBloqueoPermanente;
    } else if (tipoFilter === "temporal") {
      matchesTipo = !b.esBloqueoPermanente;
    }

    return matchesSearch && matchesEstado && matchesTipo;
  });

  // Estadísticas
  $: stats = {
    total: bloqueados.length,
    activos: bloqueados.filter((b) => b.isActive).length,
    permanentes: bloqueados.filter((b) => b.esBloqueoPermanente).length,
    temporales: bloqueados.filter((b) => !b.esBloqueoPermanente).length,
  };

  async function handleAddSubmit(data: any) {
    addFormLoading = true;
    await onAddToBlacklist(data);
    addFormLoading = false;
    closeModal();
  }

  async function handleUnblockSubmit(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    addFormLoading = true;
    await onUnblock(data);
    addFormLoading = false;
    closeUnblockModal();
  }

  function handleUnblockClick(bloqueado: ListaNegraResponse) {
    selectedBloqueado = bloqueado;
    showUnblockModal = true;
  }

  $: if (!showAddModal) {
    addFormLoading = false;
  }

  let formRef: any;

  function resetForm() {
    if (formRef && formRef.reset) {
      formRef.reset();
    }
  }

  function closeModal() {
    showAddModal = false;
    resetForm();
  }

  function closeUnblockModal() {
    showUnblockModal = false;
    selectedBloqueado = null;
  }
</script>

<div class="flex h-full flex-col bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">Lista Negra</h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión de personas bloqueadas del acceso a las instalaciones
        </p>
      </div>
      <div class="flex gap-3">
        <button
          on:click={() => (showAddModal = true)}
          class="flex items-center gap-2 rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white transition-all hover:bg-red-500"
        >
          <Plus size={16} />
          Agregar a Lista Negra
        </button>
        <button
          on:click={onRefresh}
          disabled={loading}
          class="flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-all hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
        >
          <RefreshCw size={16} class={loading ? "animate-spin" : ""} />
          Actualizar
        </button>
      </div>
    </div>

    <!-- Stats -->
    <div class="mt-4 grid grid-cols-4 gap-4">
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Total</div>
        <div class="mt-1 text-2xl font-semibold text-white">{stats.total}</div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Activos</div>
        <div class="mt-1 text-2xl font-semibold text-red-400">
          {stats.activos}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Permanentes</div>
        <div class="mt-1 text-2xl font-semibold text-purple-400">
          {stats.permanentes}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Temporales</div>
        <div class="mt-1 text-2xl font-semibold text-yellow-400">
          {stats.temporales}
        </div>
      </div>
    </div>
  </div>

  <!-- Filters -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex flex-wrap items-center gap-4">
      <div class="relative flex-1 min-w-[300px]">
        <Search
          size={18}
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
        />
        <input
          type="text"
          bind:value={searchTerm}
          placeholder="Buscar por nombre, cédula o empresa..."
          class="w-full rounded-lg border border-white/10 bg-[#1e1e1e] py-2 pl-10 pr-4 text-sm text-white placeholder-gray-500 focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500"
        />
      </div>

      <div class="flex items-center gap-2">
        <Filter size={16} class="text-gray-400" />
        <select
          bind:value={estadoFilter}
          class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500"
        >
          <option value="todos">Todos los estados</option>
          <option value="activo">Bloqueados activos</option>
          <option value="inactivo">Desbloqueados</option>
        </select>
      </div>

      <select
        bind:value={tipoFilter}
        class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500"
      >
        <option value="todos">Todos los tipos</option>
        <option value="permanente">Permanentes</option>
        <option value="temporal">Temporales</option>
      </select>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden">
    {#if error}
      <div
        class="m-6 flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
        transition:fade
      >
        <AlertCircle size={20} />
        <div>
          <div class="font-medium">Error al cargar lista negra</div>
          <div class="text-sm opacity-90">{error}</div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <RefreshCw size={32} class="mx-auto animate-spin text-red-500" />
          <p class="mt-4 text-sm text-gray-400">Cargando lista negra...</p>
        </div>
      </div>
    {:else if filteredBloqueados.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <UserX size={48} class="mx-auto text-gray-600" />
          <p class="mt-4 text-lg font-medium text-gray-400">
            No se encontraron registros
          </p>
          <p class="mt-2 text-sm text-gray-500">
            {searchTerm || estadoFilter !== "todos" || tipoFilter !== "todos"
              ? "Intenta ajustar los filtros de búsqueda"
              : "No hay personas bloqueadas en el sistema"}
          </p>
        </div>
      </div>
    {:else}
      <ListaNegraTable
        data={filteredBloqueados}
        onUnblock={handleUnblockClick}
      />
    {/if}
  </div>
</div>

<!-- Modal para agregar a lista negra -->
{#if showAddModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={closeModal}
    ></div>
    <div
      class="relative z-10 w-full max-w-4xl"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <Listanegraform
        bind:this={formRef}
        loading={addFormLoading}
        onSubmit={handleAddSubmit}
        onUnblock={handleUnblockSubmit}
      />
    </div>
  </div>
{/if}

<!-- Modal para desbloquear -->
{#if showUnblockModal && selectedBloqueado}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={closeUnblockModal}
    ></div>
    <div
      class="relative z-10 w-full max-w-2xl"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Aquí va tu componente de desbloqueo -->
      <div class="rounded-lg bg-[#252526] p-6">
        <h3 class="text-lg font-semibold text-white">Desbloquear persona</h3>
        <p class="mt-2 text-sm text-gray-400">
          {selectedBloqueado.nombreCompleto} - {selectedBloqueado.cedula}
        </p>
        <!-- Formulario de desbloqueo aquí -->
      </div>
    </div>
  </div>
{/if}
