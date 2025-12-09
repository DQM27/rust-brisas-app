// =============================================================================
// CELESTIAL SYSTEM - Simple Sun & Moon (estilo original)
// =============================================================================

import type { 
  CanvasContext, 
  RenderState, 
  CelestialSystemState, 
  SunState, 
  MoonState,
} from '../types';
import { TIME, CELESTIAL_CONFIG } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initCelestialSystem(): CelestialSystemState {
  return {
    sun: {
      x: 50,
      y: 120,
      opacity: 0,
      rotation: 0,
      scale: 1,
      rayRotation: 0,
      glowIntensity: 1,
    },
    moon: {
      x: 50,
      y: 120,
      opacity: 0,
      rotation: 0,
      scale: 1,
      phase: 'full',
    },
    showBoth: false,
  };
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateCelestialSystem(
  state: CelestialSystemState,
  render: RenderState
): CelestialSystemState {
  const hour = render.time;
  
  // Calculate sun position
  const sun = calculateSunPosition(hour, state.sun, render.timestamp);
  
  // Calculate moon position
  const moon = calculateMoonPosition(hour, state.moon);
  
  // Show both during transitions
  const showBoth = (hour >= TIME.DAWN_START && hour < TIME.DAWN_END) ||
                   (hour >= TIME.DUSK_START && hour < TIME.DUSK_END);
  
  return { sun, moon, showBoth };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderCelestialSystem(
  state: CelestialSystemState,
  render: RenderState,
  canvas: CanvasContext
): void {
  const { ctx, width, height } = canvas;
  
  // Render moon first (behind sun during transitions)
  if (state.moon.opacity > 0) {
    renderMoon(state.moon, ctx, width, height, render.timestamp);
  }
  
  // Render sun
  if (state.sun.opacity > 0) {
    renderSun(state.sun, ctx, width, height, render.timestamp);
  }
}

// -----------------------------------------------------------------------------
// Sun Position & Rendering (estilo original - simple con glow parpadeante)
// -----------------------------------------------------------------------------

function calculateSunPosition(hour: number, current: SunState, timestamp: number): SunState {
  let x = 50;
  let y = CELESTIAL_CONFIG.ARC_BOTTOM;
  let opacity = 0;
  
  // Sun visible from DAWN_START to DUSK_END
  if (hour >= TIME.DAWN_START && hour < TIME.DUSK_END) {
    const duration = TIME.DUSK_END - TIME.DAWN_START;
    const progress = (hour - TIME.DAWN_START) / duration;
    
    // X: -10% to 110%
    x = -10 + progress * 120;
    
    // Y: Arco usando seno - de ARC_BOTTOM a ARC_TOP
    const arcHeight = Math.sin(progress * Math.PI);
    y = CELESTIAL_CONFIG.ARC_BOTTOM - arcHeight * (CELESTIAL_CONFIG.ARC_BOTTOM - CELESTIAL_CONFIG.ARC_TOP);
    
    // Opacity: fade in/out en los bordes
    if (progress < 0.08) {
      opacity = progress / 0.08;
    } else if (progress > 0.92) {
      opacity = (1 - progress) / 0.08;
    } else {
      opacity = 1;
    }
  }
  
  // Glow pulsante (como animate-pulse del original)
  const glowIntensity = 0.8 + Math.sin(timestamp / 1500) * 0.2;
  
  return {
    x,
    y,
    opacity,
    rotation: 0,
    scale: 1,
    rayRotation: 0,
    glowIntensity,
  };
}

function renderSun(sun: SunState, ctx: CanvasRenderingContext2D, width: number, height: number, timestamp: number): void {
  const x = (sun.x / 100) * width;
  const y = (sun.y / 100) * height;
  const size = CELESTIAL_CONFIG.SUN_SIZE;
  const colors = CELESTIAL_CONFIG.SUN_COLORS;
  
  ctx.save();
  ctx.globalAlpha = sun.opacity;
  
  // Glow exterior grande (parpadeante) - como el blur-xl del original
  const outerGlowSize = size * 2.5 * sun.glowIntensity;
  const outerGlow = ctx.createRadialGradient(x, y, size * 0.3, x, y, outerGlowSize);
  outerGlow.addColorStop(0, colors.glowOuter);
  outerGlow.addColorStop(1, 'rgba(255, 160, 60, 0)');
  ctx.fillStyle = outerGlow;
  ctx.beginPath();
  ctx.arc(x, y, outerGlowSize, 0, Math.PI * 2);
  ctx.fill();
  
  // Glow medio (como blur-md del original)
  const midGlowSize = size * 1.5;
  const midGlow = ctx.createRadialGradient(x, y, size * 0.2, x, y, midGlowSize);
  midGlow.addColorStop(0, colors.glow);
  midGlow.addColorStop(1, 'rgba(255, 200, 100, 0)');
  ctx.fillStyle = midGlow;
  ctx.beginPath();
  ctx.arc(x, y, midGlowSize, 0, Math.PI * 2);
  ctx.fill();
  
  // Cuerpo del sol (círculo sólido con gradiente suave)
  const bodyGradient = ctx.createRadialGradient(
    x - size * 0.15, y - size * 0.15, 0,
    x, y, size * 0.5
  );
  bodyGradient.addColorStop(0, '#ffee88');
  bodyGradient.addColorStop(0.7, colors.core);
  bodyGradient.addColorStop(1, '#ffcc22');
  
  ctx.fillStyle = bodyGradient;
  ctx.beginPath();
  ctx.arc(x, y, size * 0.5, 0, Math.PI * 2);
  ctx.fill();
  
  ctx.restore();
}

// -----------------------------------------------------------------------------
// Moon Position & Rendering (estilo original - simple crescent con glow)
// -----------------------------------------------------------------------------

function calculateMoonPosition(hour: number, current: MoonState): MoonState {
  let x = 50;
  let y = CELESTIAL_CONFIG.ARC_BOTTOM;
  let opacity = 0;
  
  // Moon visible from DUSK_START to DAWN_END (cruza medianoche)
  let progress = 0;
  const isVisible = hour >= TIME.DUSK_START || hour < TIME.DAWN_END;
  
  if (isVisible) {
    // Calcular progreso (0 a 1 a través de la noche)
    const nightDuration = (24 - TIME.DUSK_START) + TIME.DAWN_END; // ~13 horas
    
    if (hour >= TIME.DUSK_START) {
      progress = (hour - TIME.DUSK_START) / nightDuration;
    } else {
      progress = (hour + (24 - TIME.DUSK_START)) / nightDuration;
    }
    
    // X: -10% to 110%
    x = -10 + progress * 120;
    
    // Y: Arco
    const arcHeight = Math.sin(progress * Math.PI);
    y = CELESTIAL_CONFIG.ARC_BOTTOM - arcHeight * (CELESTIAL_CONFIG.ARC_BOTTOM - CELESTIAL_CONFIG.ARC_TOP);
    
    // Opacity
    if (progress < 0.08) {
      opacity = progress / 0.08;
    } else if (progress > 0.92) {
      opacity = (1 - progress) / 0.08;
    } else {
      opacity = 1;
    }
  }
  
  return {
    x,
    y,
    opacity,
    rotation: 0,
    scale: 1,
    phase: 'full', // No usamos fases complicadas
  };
}

function renderMoon(moon: MoonState, ctx: CanvasRenderingContext2D, width: number, height: number, timestamp: number): void {
  const x = (moon.x / 100) * width;
  const y = (moon.y / 100) * height;
  const size = CELESTIAL_CONFIG.MOON_SIZE;
  const colors = CELESTIAL_CONFIG.MOON_COLORS;
  
  ctx.save();
  ctx.globalAlpha = moon.opacity;
  
  // Glow pulsante (como el original)
  const glowPulse = 0.8 + Math.sin(timestamp / 2000) * 0.2;
  
  // Glow exterior grande
  const outerGlowSize = size * 2 * glowPulse;
  const outerGlow = ctx.createRadialGradient(x, y, size * 0.3, x, y, outerGlowSize);
  outerGlow.addColorStop(0, 'rgba(180, 200, 255, 0.3)');
  outerGlow.addColorStop(1, 'rgba(180, 200, 255, 0)');
  ctx.fillStyle = outerGlow;
  ctx.beginPath();
  ctx.arc(x, y, outerGlowSize, 0, Math.PI * 2);
  ctx.fill();
  
  // Glow medio
  const midGlowSize = size * 1.3;
  const midGlow = ctx.createRadialGradient(x, y, size * 0.2, x, y, midGlowSize);
  midGlow.addColorStop(0, colors.glow);
  midGlow.addColorStop(1, 'rgba(200, 210, 255, 0)');
  ctx.fillStyle = midGlow;
  ctx.beginPath();
  ctx.arc(x, y, midGlowSize, 0, Math.PI * 2);
  ctx.fill();
  
  // Dibujar luna creciente (crescent moon) - estilo simple
  // Círculo principal de la luna
  ctx.fillStyle = colors.fill;
  ctx.beginPath();
  ctx.arc(x, y, size * 0.45, 0, Math.PI * 2);
  ctx.fill();
  
  // Sombra para crear efecto de media luna
  ctx.fillStyle = 'rgba(15, 20, 40, 0.92)';
  ctx.beginPath();
  ctx.arc(x + size * 0.22, y, size * 0.36, 0, Math.PI * 2);
  ctx.fill();
  
  ctx.restore();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const celestialSystem = {
  init: initCelestialSystem,
  update: updateCelestialSystem,
  render: renderCelestialSystem,
};
