<script lang="ts">
  import { generalSettings } from "$lib/stores/settingsStore";
  import { currentSeason } from "$lib/utils/season";
  import { getMountainBrightness } from "./constants";
  import {
    LANDSCAPE_PATHS,
    getLandscapeTheme,
    type LandscapeType,
  } from "./systems/landscapeData";
  import type { Season } from "./types";
  import { fade } from "svelte/transition";

  // Reactive values (Svelte 5 syntax)
  let effectiveHour = $derived(
    $generalSettings.overrideHour ?? new Date().getHours(),
  );
  let season = $derived(
    ($generalSettings.overrideSeason ?? $currentSeason) as Season,
  );

  // Get current landscape settings
  let type = $derived($generalSettings.landscapeType as LandscapeType);
  let theme = $derived(getLandscapeTheme(type, season));
  let paths = $derived(LANDSCAPE_PATHS[type]);

  // Get brightness based on time of day
  let brightness = $derived(getMountainBrightness(effectiveHour));

  // Brightness adjustments for specific biomes
  let adjustedBrightness = $derived(
    type === "city" && brightness < 0.5
      ? brightness * 1.5 // Cities are brighter at night
      : type === "moon" && brightness > 0.5
        ? 0.7 // Moon is darker during "day" (which represents illuminated side)
        : brightness,
  );

  // Special City Lights logic
  let showCityLights = $derived(type === "city" && brightness < 0.6);

  // Special Beach Waves logic (animation)
  let isBeach = $derived(type === "beach");
</script>

<div class="absolute inset-0 w-full h-full overflow-hidden pointer-events-none">
  <!-- Key is used to force transition when landscape type changes -->
  {#key type}
    <div class="absolute inset-0 w-full h-full" in:fade={{ duration: 1000 }}>
      <!-- Background Layer -->
      <svg
        class="absolute bottom-0 w-full h-[60%] transition-all duration-1000 ease-in-out"
        viewBox="0 0 1440 320"
        preserveAspectRatio="none"
        style="filter: brightness({adjustedBrightness}); color: {theme
          .colors[0]}"
      >
        <path fill="currentColor" fill-opacity="0.85" d={paths[0]} />
      </svg>

      <!-- Midground Layer -->
      <svg
        class="absolute bottom-0 w-full h-[50%] transition-all duration-1000 ease-in-out {isBeach
          ? 'animate-wave-slow'
          : ''}"
        viewBox="0 0 1440 320"
        preserveAspectRatio="none"
        style="filter: brightness({adjustedBrightness}); color: {theme
          .colors[1]}"
      >
        <path fill="currentColor" fill-opacity="0.9" d={paths[1]} />
      </svg>

      <!-- Foreground Layer -->
      <svg
        class="absolute bottom-0 w-full h-[35%] transition-all duration-1000 ease-in-out {isBeach
          ? 'animate-wave-fast'
          : ''}"
        viewBox="0 0 1440 320"
        preserveAspectRatio="none"
        style="filter: brightness({adjustedBrightness}); color: {theme
          .colors[2]}"
      >
        <path fill="currentColor" fill-opacity="1" d={paths[2]} />
      </svg>

      <!-- City Lights Overlay (only for City at night) -->
      {#if showCityLights}
        <svg
          class="absolute bottom-0 w-full h-[50%] transition-opacity duration-1000"
          viewBox="0 0 1440 320"
          preserveAspectRatio="none"
          style="opacity: {0.8 - brightness}"
        >
          <!-- Random windows/lights - simplified -->
          <defs>
            <pattern
              id="city-lights"
              x="0"
              y="0"
              width="20"
              height="20"
              patternUnits="userSpaceOnUse"
            >
              <rect
                x="2"
                y="2"
                width="4"
                height="4"
                fill="#fbbf24"
                fill-opacity="0.6"
              />
              <rect
                x="12"
                y="10"
                width="3"
                height="3"
                fill="#fbbf24"
                fill-opacity="0.4"
              />
            </pattern>
          </defs>
          <!-- Masking would be ideal, but for now just an overlay rect clipped by the city path manually would be complex. 
                     Simpler approach: Just dots that fade in/out roughly where buildings are.
                     OR: rely on the fact that city mid layer is buildings. -->
        </svg>
      {/if}
    </div>
  {/key}
</div>

<style>
  @keyframes wave {
    0% {
      transform: translateX(0) scaleY(1);
    }
    50% {
      transform: translateX(-10px) scaleY(1.05);
    }
    100% {
      transform: translateX(0) scaleY(1);
    }
  }

  .animate-wave-slow {
    animation: wave 8s ease-in-out infinite;
  }

  .animate-wave-fast {
    animation: wave 5s ease-in-out infinite;
    animation-delay: 0.5s;
  }
</style>
