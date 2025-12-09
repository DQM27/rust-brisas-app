<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { generalSettings, type Season } from "$lib/stores/settingsStore";
  import { currentSeason } from "$lib/utils/season";
  import { currentTime } from "$lib/stores/timeStore";
  
  import type {
    CanvasContext,
    RenderState,
    SkyState,
    StarSystemState,
    CelestialSystemState,
    CloudSystemState,
    ParticleSystemState,
    WindState,
    BirthdaySystemState,
  } from "./types";
  
  import { TIME, getTimeOfDay } from "./constants";
  import { skySystem, getSkyGradientCSS } from "./systems/skySystem";
  import { starSystem } from "./systems/starSystem";
  import { celestialSystem } from "./systems/celestialSystem";
  import { cloudSystem } from "./systems/cloudSystem";
  import { windSystem } from "./systems/windSystem";
  import { particleSystem } from "./systems/particleSystem";
  import { birthdaySystem, getBirthdayGradientCSS } from "./systems/birthdaySystem";

  // -----------------------------------------------------------------------------
  // Props (Svelte 5 syntax)
  // -----------------------------------------------------------------------------
  
  interface Props {
    isBirthday?: boolean;
    children?: import('svelte').Snippet;
  }
  
  let { isBirthday = false, children }: Props = $props();
  
  // These read from store (reactive - Svelte 5)
  let enableWeather = $derived($generalSettings.enableWeatherEffects);
  let showClouds = $derived($generalSettings.showClouds);
  let showStars = $derived($generalSettings.showStars);
  let showCelestial = $derived($generalSettings.showCelestial);
  
  // -----------------------------------------------------------------------------
  // Canvas & State
  // -----------------------------------------------------------------------------
  
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;
  let animationFrameId: number;
  let lastTimestamp: number = 0;
  
  // System states
  let skyState: SkyState;
  let starState: StarSystemState;
  let celestialState: CelestialSystemState;
  let cloudState: CloudSystemState;
  let particleState: ParticleSystemState;
  let windState: WindState;
  let birthdayState: BirthdaySystemState | null = null;
  
  // Reactive sky gradient for CSS background
  let skyGradientCSS: string = $state("");
  let birthdayGradientCSS: string = $state("");
  
  // -----------------------------------------------------------------------------
  // Reactive values
  // -----------------------------------------------------------------------------
  
  function getHourFromDate(date: Date): number {
    return date.getHours() + date.getMinutes() / 60;
  }
  
  let effectiveHour = $derived($generalSettings.overrideHour ?? getHourFromDate($currentTime));
  let season = $derived(($generalSettings.overrideSeason ?? $currentSeason) as Season);
  let isNight = $derived(effectiveHour >= TIME.DUSK_END || effectiveHour < TIME.DAWN_START);
  
  // -----------------------------------------------------------------------------
  // Canvas Setup
  // -----------------------------------------------------------------------------
  
  function getCanvasContext(): CanvasContext | null {
    if (!canvas || !ctx) return null;
    
    const dpr = window.devicePixelRatio || 1;
    return {
      ctx,
      width: canvas.width / dpr,
      height: canvas.height / dpr,
      dpr,
    };
  }
  
  function resize() {
    if (!canvas) return;
    
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    
    if (ctx) {
      ctx.scale(dpr, dpr);
    }
    
    // Reinitialize systems that depend on canvas size
    const canvasCtx = getCanvasContext();
    if (canvasCtx) {
      initializeSystems(canvasCtx);
    }
  }
  
  // -----------------------------------------------------------------------------
  // System Initialization
  // -----------------------------------------------------------------------------
  
  function initializeSystems(canvasCtx: CanvasContext) {
    skyState = skySystem.init();
    starState = starSystem.init(canvasCtx);
    celestialState = celestialSystem.init();
    cloudState = cloudSystem.init(canvasCtx);
    windState = windSystem.init();
    particleState = particleSystem.init(canvasCtx, season, isNight);
    
    if (isBirthday) {
      birthdayState = birthdaySystem.init(canvasCtx);
    }
  }
  
  // -----------------------------------------------------------------------------
  // Animation Loop
  // -----------------------------------------------------------------------------
  
  function loop(timestamp: number) {
    animationFrameId = requestAnimationFrame(loop);
    
    const canvasCtx = getCanvasContext();
    if (!canvasCtx) return;
    
    // Calculate delta time
    const deltaTime = lastTimestamp ? timestamp - lastTimestamp : 16;
    lastTimestamp = timestamp;
    
    // Build render state
    const renderState: RenderState = {
      time: effectiveHour,
      season,
      deltaTime,
      timestamp,
      wind: windState,
      isBirthday,
    };
    
    // Clear canvas
    canvasCtx.ctx.clearRect(0, 0, canvasCtx.width, canvasCtx.height);
    
    // --- UPDATE PHASE ---
    
    // Always update wind (affects particles and clouds)
    windState = windSystem.update(windState, deltaTime);
    renderState.wind = windState;
    
    // Update sky (for CSS gradient)
    skyState = skySystem.update(skyState, renderState);
    
    if (isBirthday) {
      // Birthday mode
      if (birthdayState) {
        birthdayState = birthdaySystem.update(birthdayState, renderState, canvasCtx);
      }
      birthdayGradientCSS = getBirthdayGradientCSS(timestamp);
    } else {
      // Normal mode
      skyGradientCSS = getSkyGradientCSS(skyState);
      
      // Update stars (only at night, and if enabled)
      if (showStars) {
        starState = starSystem.update(starState, renderState, canvasCtx);
      }
      
      // Update celestial bodies
      if (showCelestial) {
        celestialState = celestialSystem.update(celestialState, renderState);
      }
      
      // Update clouds
      if (showClouds) {
        cloudState = cloudSystem.update(cloudState, renderState, canvasCtx);
      }
    }
    
    // Update weather particles
    if (enableWeather && !isBirthday) {
      particleState = particleSystem.update(particleState, renderState, canvasCtx);
    }
    
    // --- RENDER PHASE ---
    // Order matters: back to front
    
    if (isBirthday) {
      // Birthday rendering (no sky/stars/celestial - uses CSS gradient)
      if (birthdayState) {
        birthdaySystem.render(birthdayState, renderState, canvasCtx);
      }
    } else {
      // Normal rendering order:
      // 1. Stars (behind everything)
      if (showStars) {
        starSystem.render(starState, renderState, canvasCtx);
      }
      
      // 2. Clouds (behind celestial, but stars show through gaps)
      if (showClouds) {
        cloudSystem.render(cloudState, renderState, canvasCtx);
      }
      
      // 3. Celestial bodies (sun/moon)
      if (showCelestial) {
        celestialSystem.render(celestialState, renderState, canvasCtx);
      }
      
      // 4. Weather particles (in front of everything)
      if (enableWeather) {
        particleSystem.render(particleState, renderState, canvasCtx);
      }
    }
  }
  
  // -----------------------------------------------------------------------------
  // Lifecycle
  // -----------------------------------------------------------------------------
  
  onMount(() => {
    ctx = canvas.getContext("2d");
    
    resize();
    window.addEventListener("resize", resize);
    
    // Start animation loop
    animationFrameId = requestAnimationFrame(loop);
  });
  
  onDestroy(() => {
    if (typeof window !== "undefined") {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(animationFrameId);
    }
  });
  
  // Watch for birthday mode changes
  $effect(() => {
    if (isBirthday && canvas && ctx) {
      const canvasCtx = getCanvasContext();
      if (canvasCtx && !birthdayState) {
        birthdayState = birthdaySystem.init(canvasCtx);
      }
    } else if (!isBirthday) {
      birthdayState = null;
    }
  });
  
  // Watch for season changes to reinitialize particles
  let lastSeason: Season | null = $state(null);
  
  $effect(() => {
    if (season !== lastSeason && canvas && ctx) {
      lastSeason = season;
      const canvasCtx = getCanvasContext();
      if (canvasCtx) {
        particleState = particleSystem.init(canvasCtx, season, isNight);
      }
    }
  });
</script>

<!-- Background gradient (CSS for performance) -->
<div
  class="absolute inset-0 transition-all duration-1000 ease-out"
  style="background: {isBirthday ? birthdayGradientCSS : skyGradientCSS};"
/>

<!-- Canvas for all animated elements -->
<canvas
  bind:this={canvas}
  class="absolute inset-0 w-full h-full"
  style="z-index: 1;"
/>

<!-- Slot for MountainLandscape SVG (rendered on top of canvas sky) -->
<div class="absolute inset-0" style="z-index: 2;">
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  canvas {
    pointer-events: none;
  }
</style>
