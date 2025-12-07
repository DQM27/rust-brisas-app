<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { generalSettings } from "$lib/stores/settingsStore";
  import { Sun, Moon } from "lucide-svelte";
  import { fly } from "svelte/transition";

  let time: number = 0; // 0.0 to 23.99
  let interval: NodeJS.Timeout;

  // Configuration
  const SUN_START = 6;
  const SUN_END = 18;

  $: overrideHour = $generalSettings.overrideHour;

  function updateTime() {
    if (overrideHour !== null && overrideHour !== undefined) {
      time = overrideHour;
    } else {
      const now = new Date();
      time = now.getHours() + now.getMinutes() / 60;
    }
  }

  // Calculate position
  // We want an arc.
  // x: 0% -> 100%
  // y: 100% (horizon) -> 20% (zenith) -> 100% (horizon)

  let x = 0;
  let y = 0;
  let isSun = true;
  let opacity = 0;

  $: {
    updateTime();

    // Determine if it's day (Sun) or night (Moon)
    // Day: 06:00 to 18:00
    isSun = time >= SUN_START && time < SUN_END;

    let progress = 0; // 0.0 to 1.0 representing path completion

    if (isSun) {
      // 6 -> 0.0, 12 -> 0.5, 18 -> 1.0
      progress = (time - SUN_START) / (SUN_END - SUN_START);
    } else {
      // Night path: 18:00 (0.0) -> 24:00 (0.5) -> 06:00 (1.0)
      if (time >= SUN_END) {
        // 18 to 24
        progress = (time - SUN_END) / 12;
      } else {
        // 0 to 6
        progress = (time + (24 - SUN_END)) / 12;
      }
    }

    // Calculate Coordinates
    // X: -10% to 110% to ensure entering/leaving screen completely
    x = -10 + progress * 120;

    // Y: Arc. Math.sin(0) = 0, Math.sin(PI/2) = 1, Math.sin(PI) = 0
    // We want Y to go from say 110% (below screen) to 10% (top)
    const arcHeight = Math.sin(progress * Math.PI);
    // Invert because CSS Top 0 is top
    // 100% (bottom) - arcHeight * (amplitude)
    y = 120 - arcHeight * 100;

    // Opacity fade at edges
    if (progress < 0.1) opacity = progress * 10;
    else if (progress > 0.9) opacity = (1 - progress) * 10;
    else opacity = 1;
  }

  onMount(() => {
    updateTime();
    interval = setInterval(updateTime, 1000 * 60); // Update every minute if auto
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<div
  class="pointer-events-none absolute inset-0 overflow-hidden"
  style="z-index: 0;"
>
  <!-- Celestial Body Container -->
  <div
    class="absolute flex items-center justify-center transition-all duration-700 ease-out"
    style="
            left: {x}%;
            top: {y}%;
            opacity: {opacity};
            transform: translate(-50%, -50%);
        "
  >
    {#if isSun}
      <div class="relative">
        <!-- Sun Glow -->
        <div
          class="absolute inset-0 blur-xl bg-orange-400/50 rounded-full scale-150 animate-pulse"
        ></div>
        <div
          class="absolute inset-0 blur-md bg-yellow-300/60 rounded-full scale-110"
        ></div>
        <!-- Sun Icon -->
        <Sun
          size={80}
          class="text-yellow-400 fill-yellow-400 drop-shadow-lg relative z-10"
        />
      </div>
    {:else}
      <div class="relative">
        <!-- Moon Glow -->
        <div
          class="absolute inset-0 blur-xl bg-blue-400/30 rounded-full scale-150 animate-pulse"
        ></div>
        <div
          class="absolute inset-0 blur-md bg-blue-200/40 rounded-full scale-110"
        ></div>
        <!-- Moon Icon -->
        <Moon
          size={80}
          class="text-blue-100 fill-blue-100 drop-shadow-lg relative z-10"
        />
      </div>
    {/if}
  </div>
</div>
