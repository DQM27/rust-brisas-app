<script lang="ts">
  import { fly } from "svelte/transition";
  import {
    ChevronLeft,
    ChevronRight,
    Calendar as CalendarIcon,
  } from "lucide-svelte";
  import { onMount } from "svelte";

  interface Props {
    value: string | undefined | null; // Allow flexible types
    label?: string;
    id?: string;
    disabled?: boolean;
    placeholder?: string;
  }

  let {
    value = $bindable(""),
    label = "",
    id = Math.random().toString(36).substring(7),
    disabled = false,
    placeholder = "Seleccionar fecha",
  }: Props = $props();

  // State
  let showCalendar = $state(false);
  let currentMonth = $state(new Date().getMonth());
  let currentYear = $state(new Date().getFullYear());

  // Initialize view based on value
  $effect(() => {
    if (value) {
      const [y, m] = value.split("-").map(Number);
      if (!isNaN(y) && !isNaN(m)) {
        // Only update if not already viewing this month/year to avoid jumping while navigating
        // But for initial load/external update, we want to sync.
        // For simplicity, we sync if the menu is closed, or just rely on manual navigation.
        // Let's only sync on initial mount or if value changes externally while closed.
        if (!showCalendar) {
          currentMonth = m - 1;
          currentYear = y;
        }
      }
    }
  });

  // Derived
  let daysInMonth = $derived(
    new Date(currentYear, currentMonth + 1, 0).getDate(),
  );
  let startDay = $derived(new Date(currentYear, currentMonth, 1).getDay()); // 0 = Sun

  // Format helpers
  const months = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
  ];

  const weekDays = ["DO", "LU", "MA", "MI", "JU", "VI", "SA"];

  // Logic
  function toggleCalendar() {
    if (!disabled) showCalendar = !showCalendar;
  }

  function prevMonth() {
    if (currentMonth === 0) {
      currentMonth = 11;
      currentYear--;
    } else {
      currentMonth--;
    }
  }

  function nextMonth() {
    if (currentMonth === 11) {
      currentMonth = 0;
      currentYear++;
    } else {
      currentMonth++;
    }
  }

  function selectDate(day: number) {
    const m = String(currentMonth + 1).padStart(2, "0");
    const d = String(day).padStart(2, "0");
    value = `${currentYear}-${m}-${d}`;
    showCalendar = false;
  }

  function selectToday() {
    const now = new Date();
    currentMonth = now.getMonth();
    currentYear = now.getFullYear();
    const m = String(now.getMonth() + 1).padStart(2, "0");
    const d = String(now.getDate()).padStart(2, "0");
    value = `${now.getFullYear()}-${m}-${d}`;
    showCalendar = false;
  }

  function clearDate() {
    value = "";
    showCalendar = false;
  }

  // Formatting for display
  function formatDate(iso: string) {
    if (!iso) return "";
    const [y, m, d] = iso.split("-");
    return `${d}/${m}/${y}`;
  }

  // Click outside handler
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (showCalendar && !target.closest(".datepicker-container")) {
      showCalendar = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative datepicker-container w-full">
  {#if label}
    <label for={id} class="block text-xs font-medium text-secondary mb-1"
      >{label}</label
    >
  {/if}

  <button
    type="button"
    {id}
    {disabled}
    onclick={toggleCalendar}
    class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-left flex items-center justify-between transition-all focus:outline-none {showCalendar
      ? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
      : ''} {disabled
      ? 'opacity-50 cursor-not-allowed'
      : 'cursor-pointer hover:border-white/20'}"
  >
    <span class={value ? "text-white" : "text-gray-500"}>
      {value ? formatDate(value) : placeholder}
    </span>
    <CalendarIcon size={14} class="text-secondary" />
  </button>

  {#if showCalendar && !disabled}
    <div
      class="absolute z-50 mt-1 p-3 w-[280px] bg-[#1c2128] border border-white/10 rounded-lg shadow-xl origin-top"
      transition:fly={{ y: -10, duration: 300 }}
    >
      <!-- Header -->
      <div class="flex items-center justify-between mb-3 text-white">
        <button
          type="button"
          onclick={prevMonth}
          class="p-1 hover:bg-white/10 rounded-md transition-colors text-secondary hover:text-white"
        >
          <ChevronLeft size={16} />
        </button>
        <span class="text-sm font-semibold capitalize"
          >{months[currentMonth]} {currentYear}</span
        >
        <button
          type="button"
          onclick={nextMonth}
          class="p-1 hover:bg-white/10 rounded-md transition-colors text-secondary hover:text-white"
        >
          <ChevronRight size={16} />
        </button>
      </div>

      <!-- Grid -->
      <div class="grid grid-cols-7 gap-1 mb-2">
        {#each weekDays as day}
          <div class="text-[10px] font-medium text-gray-500 text-center py-1">
            {day}
          </div>
        {/each}

        <!-- Empty slots -->
        {#each Array(startDay) as _, i}
          <div></div>
        {/each}

        <!-- Days -->
        {#each Array(daysInMonth) as _, i}
          {@const day = i + 1}
          {@const isSelected =
            value ===
            `${currentYear}-${String(currentMonth + 1).padStart(2, "0")}-${String(day).padStart(2, "0")}`}
          {@const isToday =
            day === new Date().getDate() &&
            currentMonth === new Date().getMonth() &&
            currentYear === new Date().getFullYear()}

          <button
            type="button"
            onclick={() => selectDate(day)}
            class="h-8 w-8 text-sm rounded-md flex items-center justify-center transition-all
                    {isSelected
              ? 'bg-blue-600 text-white font-bold shadow-md'
              : 'text-gray-300 hover:bg-white/10 hover:text-white'}
                    {isToday && !isSelected
              ? 'border border-blue-500/50 text-blue-400'
              : ''}"
          >
            {day}
          </button>
        {/each}
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-between pt-2 border-t border-white/10"
      >
        <button
          type="button"
          onclick={clearDate}
          class="text-xs text-red-400 hover:text-red-300 px-2 py-1 hover:bg-white/5 rounded transition-colors"
        >
          Borrar
        </button>
        <button
          type="button"
          onclick={selectToday}
          class="text-xs text-blue-400 hover:text-blue-300 px-2 py-1 hover:bg-white/5 rounded transition-colors font-medium"
        >
          Hoy
        </button>
      </div>
    </div>
  {/if}
</div>
