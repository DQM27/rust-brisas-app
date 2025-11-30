<!-- src/lib/components/layout/StatusBar.svelte -->
<script lang="ts">
  import { Wifi, WifiOff } from "lucide-svelte";
  import InspectionToggle from "../layout/InspectionToggle.svelte";
  import { createEventDispatcher } from "svelte";
  import { online } from "$lib/stores/network";

  // Props
  export let inspectionPanelVisible: boolean = false;

  // Dispatcher
  const dispatch = createEventDispatcher<{
    inspectionToggle: { visible: boolean };
  }>();

  function handleInspectionToggle(
    event: CustomEvent<{ visible: boolean }>,
  ): void {
    dispatch("inspectionToggle", event.detail);
  }

  // Clases reactivas para el estado de conexión
  $: connectionClasses = $online ? "text-success" : "text-error";

  $: connectionIcon = $online ? Wifi : WifiOff;
</script>

<div
  class="flex h-6 items-center justify-between bg-surface-3 px-3
            text-xs text-text-primary select-none font-sans
            max-md:py-1 border-t border-surface"
>
  <!-- Sección izquierda - Estado de conexión -->
  <div class="flex items-center gap-4">
    <div
      class="flex items-center gap-1.5 whitespace-nowrap rounded-sm px-2 py-0.5
             transition-colors duration-200 {connectionClasses}"
      role="status"
      aria-live="polite"
    >
      <svelte:component
        this={connectionIcon}
        size={14}
        class="transition-all duration-300 {$online
          ? 'animate-pulse'
          : 'animate-bounce'}"
      />
      <span class="font-medium">
        {$online ? "En línea" : "Sin conexión"}
      </span>
    </div>
  </div>

  <!-- Sección derecha - Inspección -->
  <div class="flex items-center gap-4">
    <InspectionToggle
      visible={inspectionPanelVisible}
      variant="compact"
      on:toggle={handleInspectionToggle}
    />
  </div>
</div>
