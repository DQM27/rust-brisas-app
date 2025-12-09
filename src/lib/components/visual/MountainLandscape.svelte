<script lang="ts">
  import { generalSettings } from "$lib/stores/settingsStore";
  import { currentSeason } from "$lib/utils/season";
  import { onMount } from "svelte";

  let hour: number = new Date().getHours();

  // Color palettes based on time of day
  // [Stop1, Stop2, Stop3] for sky gradient
  const SKY_PALETTES = {
    dawn: ["#0f2027", "#203a43", "#2c5364"], // Dark blueish
    morning: ["#4facfe", "#00f2fe", "#fff"], // Bright Blue
    day: ["#2980b9", "#6dd5fa", "#ffffff"], // Standard Blue
    dusk: ["#ff7e5f", "#feb47b", "#2c3e50"], // Orange/Purple
    night: ["#000000", "#1a2a6c", "#b21f1f"], // Deep Night (Stars handled by CSS maybe)
  };

  // Mountain colors based on season and time
  // Light, Mid, Dark (Foreground)
  const MOUNTAIN_THEMES = {
    spring: ["#a8e063", "#56ab2f", "#2f5c1c"], // Greens
    summer: ["#d4fc79", "#96e6a1", "#429321"], // Lush Greens
    autumn: ["#f12711", "#f5af19", "#8e44ad"], // Oranges/Red
    winter: ["#2d4059", "#1b262c", "#0f4c75"], // Reduced brightness: Darker Blue/Greys
  };

  // Determine current palette state
  $: effectiveHour = $generalSettings.overrideHour ?? new Date().getHours();

  $: isNight = effectiveHour >= 18 || effectiveHour < 5;
  $: isDawn = effectiveHour >= 5 && effectiveHour < 6;
  $: isDay = effectiveHour >= 6 && effectiveHour < 17;
  $: isDusk = effectiveHour >= 17 && effectiveHour < 18;

  $: currentSky = isNight
    ? SKY_PALETTES.night
    : isDawn
      ? SKY_PALETTES.dawn
      : isDay
        ? SKY_PALETTES.day
        : isDusk
          ? SKY_PALETTES.dusk
          : SKY_PALETTES.day;

  // React to season store
  $: theme = MOUNTAIN_THEMES[$currentSeason] || MOUNTAIN_THEMES.spring;

  // Apply a dark filter at night
  $: brightness = isNight ? 0.3 : isDawn || isDusk ? 0.6 : 1;
</script>

<div
  class="absolute inset-0 w-full h-full overflow-hidden transition-colors duration-[2000ms]"
  style="background: linear-gradient(to bottom, {currentSky[0]}, {currentSky[1]}, {currentSky[2]});"
>
  <slot />

  <!-- Background Mountains (Farthest) -->
  <svg
    class="absolute bottom-0 w-full h-[60%] transition-all duration-1000 ease-in-out"
    viewBox="0 0 1440 320"
    preserveAspectRatio="none"
    style="opacity: 0.8; filter: brightness({brightness}); color: {theme[0]}"
  >
    <path
      fill="currentColor"
      fill-opacity="1"
      d="M0,224L48,224C96,224,192,224,288,208C384,192,480,160,576,170.7C672,181,768,235,864,240C960,245,1056,203,1152,186.7C1248,171,1344,181,1392,186.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
    ></path>
  </svg>

  <!-- Midground Mountains -->
  <svg
    class="absolute bottom-0 w-full h-[50%] transition-all duration-1000 ease-in-out"
    viewBox="0 0 1440 320"
    preserveAspectRatio="none"
    style="opacity: 0.9; filter: brightness({brightness}); color: {theme[1]}"
  >
    <path
      fill="currentColor"
      fill-opacity="1"
      d="M0,96L48,112C96,128,192,160,288,186.7C384,213,480,235,576,213.3C672,192,768,128,864,128C960,128,1056,192,1152,213.3C1248,235,1344,213,1392,202.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
    ></path>
  </svg>

  <!-- Foreground Mountains (Closest) -->
  <svg
    class="absolute bottom-0 w-full h-[35%] transition-all duration-1000 ease-in-out"
    viewBox="0 0 1440 320"
    preserveAspectRatio="none"
    style="filter: brightness({brightness}); color: {theme[2]}"
  >
    <path
      fill="currentColor"
      fill-opacity="1"
      d="M0,192L48,197.3C96,203,192,213,288,229.3C384,245,480,267,576,250.7C672,235,768,181,864,160C960,139,1056,149,1152,149.3C1248,149,1344,139,1392,133.3L1440,128L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
    ></path>
  </svg>
</div>
