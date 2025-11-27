<script lang="ts">
  import { fade, fly } from "svelte/transition";

  // --- PROPS (solo datos y callbacks) ---
  export let bloqueados: any[] = [];
  export let loading = false;
  export let searchTerm = "";
  export let filterStatus: "todos" | "activos" | "inactivos" = "activos";
  export let filterTipo: "todos" | "permanente" | "temporal" = "todos";
  
  // Callbacks
  export let onSearchChange: (value: string) => void;
  export let onFilterStatusChange: (value: "todos" | "activos" | "inactivos") => void;
  export let onFilterTipoChange: (value: "todos" | "permanente" | "temporal") => void;
  export let onUnblockOrReblock: ((bloqueado: any) => void) | undefined = undefined; // ACTUALIZADO
  export let onEdit: ((bloqueado: any) => void) | undefined = undefined;
  export let onViewDetails: ((bloqueado: any) => void) | undefined = undefined;

  // Computed (pasado desde afuera)
  export let stats: {
    total: number;
    activos: number;
    permanentes: number;
    temporales: number;
  };

  function formatDate(dateStr: string): string {
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString('es-ES', { 
        year: 'numeric', 
        month: 'short', 
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    } catch {
      return dateStr;
    }
  }

  function getDaysColor(days: number): string {
    if (days < 30) return "text-yellow-400";
    if (days < 90) return "text-orange-400";
    return "text-red-400";
  }
</script>

<div class="space-y-6">
  
  <!-- Header con Estadísticas -->
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
    <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-gray-500/10">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
        </div>
        <div>
          <p class="text-sm text-gray-400">Total</p>
          <p class="text-2xl font-semibold text-white">{stats.total}</p>
        </div>
      </div>
    </div>

    <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-red-500/10">
          <svg class="h-5 w-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
          </svg>
        </div>
        <div>
          <p class="text-sm text-gray-400">Activos</p>
          <p class="text-2xl font-semibold text-red-400">{stats.activos}</p>
        </div>
      </div>
    </div>

    <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-orange-500/10">
          <svg class="h-5 w-5 text-orange-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
          </svg>
        </div>
        <div>
          <p class="text-sm text-gray-400">Permanentes</p>
          <p class="text-2xl font-semibold text-orange-400">{stats.permanentes}</p>
        </div>
      </div>
    </div>

    <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-yellow-500/10">
          <svg class="h-5 w-5 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
          </svg>
        </div>
        <div>
          <p class="text-sm text-gray-400">Temporales</p>
          <p class="text-2xl font-semibold text-yellow-400">{stats.temporales}</p>
        </div>
      </div>
    </div>
  </div>

  <!-- Filtros y Búsqueda -->
  <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-4">
    <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
      
      <!-- Búsqueda -->
      <div class="relative flex-1 max-w-md">
        <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
        </div>
        <input
          type="text"
          value={searchTerm}
          on:input={(e) => onSearchChange(e.currentTarget.value)}
          placeholder="Buscar por nombre, cédula o motivo..."
          class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] py-2 pl-10 pr-3 text-sm text-white placeholder-gray-500 focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500"
        />
      </div>

      <!-- Filtros -->
      <div class="flex flex-wrap gap-2">
        <select value={filterStatus} on:change={(e) => onFilterStatusChange(e.currentTarget.value as any)}
          class="rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500">
          <option value="todos">Todos los estados</option>
          <option value="activos">Solo Activos</option>
          <option value="inactivos">Solo Inactivos</option>
        </select>

        <select value={filterTipo} on:change={(e) => onFilterTipoChange(e.currentTarget.value as any)}
          class="rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500">
          <option value="todos">Todos los tipos</option>
          <option value="permanente">Permanentes</option>
          <option value="temporal">Temporales</option>
        </select>
      </div>
    </div>
  </div>

  <!-- Lista de Bloqueados -->
  <div class="space-y-3">
    {#if loading}
      <div class="flex items-center justify-center py-12">
        <svg class="animate-spin h-8 w-8 text-red-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
      </div>
    {:else if bloqueados.length === 0}
      <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-12 text-center">
        <svg class="mx-auto h-12 w-12 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"/>
        </svg>
        <h3 class="mt-4 text-sm font-medium text-gray-400">No se encontraron resultados</h3>
        <p class="mt-1 text-sm text-gray-500">
          {searchTerm ? 'Intenta con otros términos de búsqueda' : 'No hay personas bloqueadas con estos filtros'}
        </p>
      </div>
    {:else}
      {#each bloqueados as bloqueado (bloqueado.id)}
        <div class="rounded-lg bg-[#1e1e1e] border border-white/10 p-5 transition-all hover:border-white/20" in:fly={{ y: 20, duration: 300 }}>
          <div class="flex items-start justify-between gap-4">
            
            <!-- Info Principal -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-3 mb-2">
                <div class="flex h-10 w-10 items-center justify-center rounded-full bg-red-500/10">
                  <svg class="h-5 w-5 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                  </svg>
                </div>
                <div class="flex-1 min-w-0">
                  <h3 class="text-base font-semibold text-white truncate">{bloqueado.nombreCompleto}</h3>
                  <p class="text-sm text-gray-400">Cédula: {bloqueado.cedula}</p>
                </div>
              </div>

              <div class="mt-3 grid grid-cols-1 gap-2 sm:grid-cols-2">
                <div class="flex items-center gap-2 text-sm">
                  <svg class="h-4 w-4 text-gray-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                  </svg>
                  <span class="text-gray-400 truncate">{bloqueado.motivoBloqueo}</span>
                </div>

                {#if bloqueado.empresaNombre}
                  <div class="flex items-center gap-2 text-sm">
                    <svg class="h-4 w-4 text-gray-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                    </svg>
                    <span class="text-gray-400 truncate">{bloqueado.empresaNombre}</span>
                  </div>
                {/if}
              </div>

              <div class="mt-3 flex flex-wrap items-center gap-2">
                <!-- Badge Estado -->
                {#if bloqueado.isActive}
                  <span class="inline-flex items-center gap-1 rounded-full bg-red-500/10 px-2.5 py-1 text-xs font-medium text-red-400">
                    <span class="h-1.5 w-1.5 rounded-full bg-red-500"></span>
                    Bloqueado
                  </span>
                {:else}
                  <span class="inline-flex items-center gap-1 rounded-full bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-400">
                    <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
                    Desbloqueado
                  </span>
                {/if}

                <!-- Badge Tipo -->
                {#if bloqueado.esBloqueoPermanente}
                  <span class="inline-flex items-center rounded-full bg-orange-500/10 px-2.5 py-1 text-xs font-medium text-orange-400">
                    Permanente
                  </span>
                {:else}
                  <span class="inline-flex items-center rounded-full bg-yellow-500/10 px-2.5 py-1 text-xs font-medium text-yellow-400">
                    Temporal
                  </span>
                {/if}

                <!-- Días transcurridos -->
                <span class="inline-flex items-center gap-1 text-xs {getDaysColor(bloqueado.diasTranscurridos)}">
                  <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                  {bloqueado.diasTranscurridos} días
                </span>
              </div>

              <div class="mt-2 text-xs text-gray-500">
                Bloqueado por: <span class="text-gray-400">{bloqueado.bloqueadoPor}</span> • {formatDate(bloqueado.fechaInicioBloqueo)}
              </div>
            </div>

            <!-- Acciones -->
            <div class="flex flex-col gap-2">
              {#if onViewDetails}
                <button on:click={() => onViewDetails(bloqueado)} type="button"
                  class="flex items-center justify-center rounded-lg bg-[#2d2d2d] p-2 text-gray-400 transition-colors hover:bg-[#3d3d3d] hover:text-white"
                  title="Ver Detalles">
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                  </svg>
                </button>
              {/if}

              {#if onEdit && bloqueado.isActive}
                <button on:click={() => onEdit(bloqueado)} type="button"
                  class="flex items-center justify-center rounded-lg bg-[#2d2d2d] p-2 text-gray-400 transition-colors hover:bg-[#3d3d3d] hover:text-blue-400"
                  title="Editar">
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                  </svg>
                </button>
              {/if}

              <!-- ACTUALIZADO: Botón que maneja desbloqueo Y re-bloqueo -->
              {#if onUnblockOrReblock}
                {#if bloqueado.isActive}
                  <!-- Persona bloqueada → Mostrar botón DESBLOQUEAR (verde) -->
                  <button on:click={() => onUnblockOrReblock(bloqueado)} type="button"
                    class="flex items-center justify-center rounded-lg bg-[#2d2d2d] p-2 text-gray-400 transition-colors hover:bg-green-500/10 hover:text-green-400"
                    title="Desbloquear">
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"/>
                    </svg>
                  </button>
                {:else}
                  <!-- Persona desbloqueada → Mostrar botón RE-BLOQUEAR (rojo) -->
                  <button on:click={() => onUnblockOrReblock(bloqueado)} type="button"
                    class="flex items-center justify-center rounded-lg bg-[#2d2d2d] p-2 text-gray-400 transition-colors hover:bg-red-500/10 hover:text-red-400"
                    title="Re-bloquear">
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                    </svg>
                  </button>
                {/if}
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>