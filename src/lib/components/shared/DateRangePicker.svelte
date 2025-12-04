<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Calendar } from 'lucide-svelte';
  import { fade, slide } from 'svelte/transition';

  /**
   * Selector de rango de fechas para toolbar
   * Se muestra como dropdown al hacer click
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  interface Props {
    startDate: string;
    endDate: string;
    label?: string;
  }

  let { startDate, endDate, label = 'Rango de fechas' }: Props = $props();

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let isOpen = $state(false);
  let tempStartDate = $state(startDate);
  let tempEndDate = $state(endDate);

  // ==========================================
  // FUNCIONES AUXILIARES
  // ==========================================

  function formatDateRange(start: string, end: string): string {
    const startFormatted = new Date(start).toLocaleDateString('es-MX', {
      day: '2-digit',
      month: 'short',
    });
    const endFormatted = new Date(end).toLocaleDateString('es-MX', {
      day: '2-digit',
      month: 'short',
    });
    return `${startFormatted} - ${endFormatted}`;
  }

  function handleApply() {
    // Validar que la fecha de inicio no sea mayor que la de fin
    if (tempStartDate > tempEndDate) {
      alert('La fecha de inicio no puede ser mayor que la fecha de fin');
      return;
    }

    dispatch('change', {
      startDate: tempStartDate,
      endDate: tempEndDate,
    });
    isOpen = false;
  }

  function handleCancel() {
    tempStartDate = startDate;
    tempEndDate = endDate;
    isOpen = false;
  }

  function setToday() {
    const today = new Date().toISOString().split('T')[0];
    tempStartDate = today;
    tempEndDate = today;
  }

  function setThisWeek() {
    const today = new Date();
    const firstDay = new Date(today.setDate(today.getDate() - today.getDay()));
    const lastDay = new Date(today.setDate(today.getDate() - today.getDay() + 6));

    tempStartDate = firstDay.toISOString().split('T')[0];
    tempEndDate = lastDay.toISOString().split('T')[0];
  }

  function setThisMonth() {
    const today = new Date();
    const firstDay = new Date(today.getFullYear(), today.getMonth(), 1);
    const lastDay = new Date(today.getFullYear(), today.getMonth() + 1, 0);

    tempStartDate = firstDay.toISOString().split('T')[0];
    tempEndDate = lastDay.toISOString().split('T')[0];
  }

  // ==========================================
  // EFECTOS
  // ==========================================

  $effect(() => {
    tempStartDate = startDate;
    tempEndDate = endDate;
  });

  // Click outside para cerrar
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.date-range-picker')) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });
</script>

<!-- 
  Date Range Picker para toolbar
  Componente de presentación con dropdown
-->

<div class="date-range-picker relative">
  <!-- Botón principal -->
  <button
    type="button"
    onclick={() => (isOpen = !isOpen)}
    class="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors"
  >
    <Calendar class="w-4 h-4" />
    <span class="hidden sm:inline">{label}:</span>
    <span class="font-semibold">{formatDateRange(startDate, endDate)}</span>
  </button>

  <!-- Dropdown -->
  {#if isOpen}
    <div
      transition:fade={{ duration: 150 }}
      class="absolute top-full mt-2 right-0 z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-xl w-80"
    >
      <div class="p-4 space-y-4">
        <!-- Quick filters -->
        <div>
          <p class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2">
            Accesos rápidos
          </p>
          <div class="flex gap-2">
            <button
              type="button"
              onclick={setToday}
              class="px-3 py-1 text-xs font-medium text-gray-700 dark:text-gray-200 bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            >
              Hoy
            </button>
            <button
              type="button"
              onclick={setThisWeek}
              class="px-3 py-1 text-xs font-medium text-gray-700 dark:text-gray-200 bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            >
              Esta semana
            </button>
            <button
              type="button"
              onclick={setThisMonth}
              class="px-3 py-1 text-xs font-medium text-gray-700 dark:text-gray-200 bg-gray-100 dark:bg-gray-700 rounded hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
            >
              Este mes
            </button>
          </div>
        </div>

        <!-- Date inputs -->
        <div class="space-y-3">
          <div>
            <label
              for="start-date"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Fecha inicio
            </label>
            <input
              id="start-date"
              type="date"
              bind:value={tempStartDate}
              class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <div>
            <label
              for="end-date"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
            >
              Fecha fin
            </label>
            <input
              id="end-date"
              type="date"
              bind:value={tempEndDate}
              class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end gap-2 pt-2 border-t border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onclick={handleCancel}
            class="px-3 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded transition-colors"
          >
            Cancelar
          </button>
          <button
            type="button"
            onclick={handleApply}
            class="px-3 py-1.5 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded transition-colors"
          >
            Aplicar
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>