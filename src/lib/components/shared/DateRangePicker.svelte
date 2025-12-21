<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import { createEventDispatcher } from "svelte";
  import { Calendar, ChevronDown, X } from "lucide-svelte";
  import { fade, slide } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  /**
   * Selector de rango de fechas para toolbar
   * Tema oscuro, minimalista
   */

  const dispatch = createEventDispatcher<{
    change: { startDate: string; endDate: string };
  }>();

  // ==========================================
  // PROPS
  // ==========================================

  interface Props {
    startDate: string;
    endDate: string;
    label?: string;
    disabled?: boolean;
  }

  let {
    startDate,
    endDate,
    label = "Período",
    disabled = false,
  }: Props = $props();

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  let isOpen = $state(false);
  let tempStartDate = $state("");
  let tempEndDate = $state("");
  let error = $state<string | null>(null);

  // ==========================================
  // CONSTANTES - Presets de fechas
  // ==========================================

  type PresetKey =
    | "today"
    | "yesterday"
    | "thisWeek"
    | "lastWeek"
    | "thisMonth"
    | "lastMonth"
    | "last7"
    | "last30";

  interface Preset {
    label: string;
    getValue: () => { start: string; end: string };
  }

  const presets: Record<PresetKey, Preset> = {
    today: {
      label: "Hoy",
      getValue: () => {
        const today = toLocalDateString(new Date());
        return { start: today, end: today };
      },
    },
    yesterday: {
      label: "Ayer",
      getValue: () => {
        const yesterday = new Date();
        yesterday.setDate(yesterday.getDate() - 1);
        const date = toLocalDateString(yesterday);
        return { start: date, end: date };
      },
    },
    thisWeek: {
      label: "Esta semana",
      getValue: () => {
        const today = new Date();
        const first = new Date(today);
        first.setDate(today.getDate() - today.getDay());
        const last = new Date(first);
        last.setDate(first.getDate() + 6);
        return {
          start: toLocalDateString(first),
          end: toLocalDateString(last),
        };
      },
    },
    lastWeek: {
      label: "Semana pasada",
      getValue: () => {
        const today = new Date();
        const first = new Date(today);
        first.setDate(today.getDate() - today.getDay() - 7);
        const last = new Date(first);
        last.setDate(first.getDate() + 6);
        return {
          start: toLocalDateString(first),
          end: toLocalDateString(last),
        };
      },
    },
    thisMonth: {
      label: "Este mes",
      getValue: () => {
        const today = new Date();
        const first = new Date(today.getFullYear(), today.getMonth(), 1);
        const last = new Date(today.getFullYear(), today.getMonth() + 1, 0);
        return {
          start: toLocalDateString(first),
          end: toLocalDateString(last),
        };
      },
    },
    lastMonth: {
      label: "Mes pasado",
      getValue: () => {
        const today = new Date();
        const first = new Date(today.getFullYear(), today.getMonth() - 1, 1);
        const last = new Date(today.getFullYear(), today.getMonth(), 0);
        return {
          start: toLocalDateString(first),
          end: toLocalDateString(last),
        };
      },
    },
    last7: {
      label: "Últimos 7 días",
      getValue: () => {
        const today = new Date();
        const past = new Date(today);
        past.setDate(today.getDate() - 6);
        return {
          start: toLocalDateString(past),
          end: toLocalDateString(today),
        };
      },
    },
    last30: {
      label: "Últimos 30 días",
      getValue: () => {
        const today = new Date();
        const past = new Date(today);
        past.setDate(today.getDate() - 29);
        return {
          start: toLocalDateString(past),
          end: toLocalDateString(today),
        };
      },
    },
  };

  // ==========================================
  // FUNCIONES AUXILIARES
  // ==========================================

  function toLocalDateString(date: Date): string {
    return date.toISOString().split("T")[0];
  }

  function formatDateRange(start: string, end: string): string {
    if (start === end) {
      return new Date(start + "T00:00:00").toLocaleDateString("es-CR", {
        day: "2-digit",
        month: "short",
      });
    }

    const startDate = new Date(start + "T00:00:00");
    const endDate = new Date(end + "T00:00:00");

    const startFormatted = startDate.toLocaleDateString("es-CR", {
      day: "2-digit",
      month: "short",
    });
    const endFormatted = endDate.toLocaleDateString("es-CR", {
      day: "2-digit",
      month: "short",
    });

    return `${startFormatted} — ${endFormatted}`;
  }

  function applyPreset(key: PresetKey) {
    const { start, end } = presets[key].getValue();
    tempStartDate = start;
    tempEndDate = end;
    error = null;
  }

  function validate(): boolean {
    if (!tempStartDate || !tempEndDate) {
      error = "Ambas fechas son requeridas";
      return false;
    }
    if (tempStartDate > tempEndDate) {
      error = "La fecha inicial no puede ser posterior a la final";
      return false;
    }
    error = null;
    return true;
  }

  function handleApply() {
    if (!validate()) return;

    dispatch("change", {
      startDate: tempStartDate,
      endDate: tempEndDate,
    });
    isOpen = false;
  }

  function handleCancel() {
    tempStartDate = startDate;
    tempEndDate = endDate;
    error = null;
    isOpen = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      handleCancel();
    } else if (event.key === "Enter" && isOpen) {
      handleApply();
    }
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
    if (!target.closest(".date-range-picker")) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener("click", handleClickOutside);
      document.addEventListener("keydown", handleKeydown);
      return () => {
        document.removeEventListener("click", handleClickOutside);
        document.removeEventListener("keydown", handleKeydown);
      };
    }
  });
</script>

<div class="date-range-picker relative">
  <!-- Botón principal -->
  <button
    type="button"
    onclick={() => !disabled && (isOpen = !isOpen)}
    {disabled}
    class="flex items-center gap-2 px-3 py-2 text-sm rounded-lg border transition-colors
      bg-[#1e1e1e] border-white/10
      {disabled
      ? 'opacity-50 cursor-not-allowed'
      : 'hover:border-white/20 hover:bg-white/5'}"
  >
    <Calendar class="w-4 h-4 text-gray-500" />
    <span class="text-gray-400">{label}:</span>
    <span class="text-white font-medium"
      >{formatDateRange(startDate, endDate)}</span
    >
    <ChevronDown
      class="w-4 h-4 text-gray-500 transition-transform {isOpen
        ? 'rotate-180'
        : ''}"
    />
  </button>

  <!-- Dropdown -->
  {#if isOpen}
    <div
      transition:slide={{ duration: 150, easing: cubicOut }}
      class="absolute top-full mt-2 right-0 z-50 w-72
        bg-[#1e1e1e] border border-white/10 rounded-lg shadow-xl overflow-hidden"
    >
      <!-- Presets -->
      <div class="p-3 border-b border-white/5">
        <p
          class="text-[10px] font-medium text-gray-500 uppercase tracking-wider mb-2"
        >
          Acceso rápido
        </p>
        <div class="grid grid-cols-2 gap-1.5">
          {#each Object.entries(presets) as [key, preset]}
            <button
              type="button"
              onclick={() => applyPreset(key as PresetKey)}
              class="px-2 py-1.5 text-xs text-gray-300 rounded
                bg-white/5 hover:bg-white/10 transition-colors text-left"
            >
              {preset.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Date inputs -->
      <div class="p-3 space-y-3">
        <div>
          <label for="start-date" class="block text-xs text-gray-500 mb-1">
            Desde
          </label>
          <input
            id="start-date"
            type="date"
            bind:value={tempStartDate}
            class="w-full px-3 py-2 text-sm rounded-lg
              bg-[#252526] border border-white/10 text-white
              focus:outline-none focus:border-white/30
              [color-scheme:dark]"
          />
        </div>

        <div>
          <label for="end-date" class="block text-xs text-gray-500 mb-1">
            Hasta
          </label>
          <input
            id="end-date"
            type="date"
            bind:value={tempEndDate}
            class="w-full px-3 py-2 text-sm rounded-lg
              bg-[#252526] border border-white/10 text-white
              focus:outline-none focus:border-white/30
              [color-scheme:dark]"
          />
        </div>

        <!-- Error -->
        {#if error}
          <p transition:fade={{ duration: 100 }} class="text-xs text-red-400">
            {error}
          </p>
        {/if}
      </div>

      <!-- Actions -->
      <div
        class="flex justify-end gap-2 px-3 py-2.5 border-t border-white/5 bg-black/20"
      >
        <button
          type="button"
          onclick={handleCancel}
          class="px-3 py-1.5 text-sm text-gray-400 hover:text-white transition-colors"
        >
          Cancelar
        </button>
        <button
          type="button"
          onclick={handleApply}
          class="px-3 py-1.5 text-sm font-medium text-white
            bg-blue-600 hover:bg-blue-500 rounded transition-colors"
        >
          Aplicar
        </button>
      </div>
    </div>
  {/if}
</div>
