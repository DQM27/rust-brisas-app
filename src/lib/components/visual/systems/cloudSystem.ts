// =============================================================================
// CLOUD SYSTEM v2 - Atmospheric clouds with natural variation
// =============================================================================

import type { CanvasContext, RenderState, Cloud, CloudSystemState } from '../types';
import {
  CLOUD_CONFIG,
  getTimeOfDay,
  randomRange,
  seededRandom,
  lerp
} from '../constants';

// -----------------------------------------------------------------------------
// Cloud Generation Helpers
// -----------------------------------------------------------------------------

/**
 * Generate highly varied blob offsets using seed
 * Each cloud gets a unique, asymmetric shape
 */
function generateAtmosphericBlobs(seed: number): Array<{ x: number; y: number; size: number; opacity: number }> {
  const blobs: Array<{ x: number; y: number; size: number; opacity: number }> = [];

  // Variable blob count (3-8) based on seed
  const blobCount = 3 + Math.floor(seededRandom(seed * 1.1) * 6);

  // Generate asymmetric cluster
  for (let i = 0; i < blobCount; i++) {
    const s = seed + i * 17.3;

    // Varied positions - NOT symmetric
    const angle = seededRandom(s) * Math.PI * 2;
    const distance = 0.1 + seededRandom(s + 0.3) * 0.6;

    // Varied sizes - larger range
    const size = 0.2 + seededRandom(s + 0.5) * 0.5;

    // Varied opacity per blob
    const opacity = 0.4 + seededRandom(s + 0.7) * 0.6;

    blobs.push({
      x: Math.cos(angle) * distance + (seededRandom(s + 0.9) - 0.5) * 0.3,
      y: Math.sin(angle) * distance * 0.5 + (seededRandom(s + 1.1) - 0.5) * 0.2,
      size,
      opacity
    });
  }

  return blobs;
}

/**
 * Create a cloud with unique characteristics
 */
function createAtmosphericCloud(index: number, canvas: CanvasContext, startOffScreen = false): Cloud {
  const seed = index * 73.7 + Math.random() * 100; // More randomness
  const layer = Math.floor(seededRandom(seed) * CLOUD_CONFIG.LAYERS);
  const scaleRange = CLOUD_CONFIG.LAYER_SCALES[layer];

  // Wider Y distribution
  const y = randomRange(CLOUD_CONFIG.Y_RANGE[0] - 5, CLOUD_CONFIG.Y_RANGE[1] + 10);

  // Variable speed per cloud (not just per layer)
  const baseSpeed = CLOUD_CONFIG.LAYER_SPEEDS[layer];
  const speedVariation = 0.5 + seededRandom(seed + 0.2) * 1.0; // 50% to 150% of base

  // Variable opacity
  const baseOpacity = 0.3 + layer * 0.1 + seededRandom(seed + 0.4) * 0.3;

  return {
    x: startOffScreen ? -40 - seededRandom(seed) * 30 : seededRandom(seed + 0.1) * 140 - 20,
    y: y,
    baseY: y,
    scale: randomRange(scaleRange[0], scaleRange[1]) * (0.7 + seededRandom(seed + 0.3) * 0.6),
    speed: baseSpeed * speedVariation,
    opacity: baseOpacity,
    layer,
    seed,
  };
}

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initCloudSystem(canvas: CanvasContext): CloudSystemState {
  const clouds: Cloud[] = [];
  for (let i = 0; i < CLOUD_CONFIG.COUNT; i++) {
    clouds.push(createAtmosphericCloud(i, canvas));
  }

  return {
    clouds,
    colorTint: CLOUD_CONFIG.TINTS.day,
  };
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateCloudSystem(
  state: CloudSystemState,
  render: RenderState,
  canvas: CanvasContext
): CloudSystemState {
  const timeOfDay = getTimeOfDay(render.time);
  const colorTint = CLOUD_CONFIG.TINTS[timeOfDay];

  const targetCount = render.cloudSettings?.count ?? CLOUD_CONFIG.COUNT;
  const windMult = render.cloudSettings?.windSpeed ?? 1.0;
  const turbulenceStr = render.cloudSettings?.turbulence ?? 0.0;

  // Manage cloud count
  while (state.clouds.length < targetCount) {
    state.clouds.push(createAtmosphericCloud(state.clouds.length + Date.now(), canvas, false));
  }
  while (state.clouds.length > targetCount) {
    state.clouds.pop();
  }

  const timestamp = render.timestamp ?? Date.now();

  // Update cloud positions with natural movement
  const clouds = state.clouds.map((cloud, index) => {
    // 1. Very slow drift (natural, not uniform)
    const driftSpeed = cloud.speed * windMult * 0.5; // Slower base movement
    const windEffect = render.wind.strength * 0.15 * windMult;

    let newX = cloud.x + (driftSpeed + windEffect) * (render.deltaTime / 16);

    // Reset cloud if it goes off screen
    if (newX > 130) {
      return createAtmosphericCloud(index + timestamp, canvas, true);
    }

    // 2. Organic vertical wobble (different frequency per cloud)
    let newY = cloud.baseY;
    const wobbleFreq = 0.0003 + seededRandom(cloud.seed + 0.6) * 0.0005;
    const wobbleAmp = 2 + seededRandom(cloud.seed + 0.8) * 4 + turbulenceStr * 5;
    const phaseOffset = cloud.seed * 10;

    newY += Math.sin(timestamp * wobbleFreq + phaseOffset) * wobbleAmp;

    // 3. Subtle horizontal sway
    const swayFreq = 0.0002 + seededRandom(cloud.seed + 1.0) * 0.0003;
    const swayAmp = 1 + seededRandom(cloud.seed + 1.2) * 2;
    newX += Math.sin(timestamp * swayFreq + phaseOffset * 0.5) * swayAmp * 0.1;

    return { ...cloud, x: newX, y: newY };
  });

  return { clouds, colorTint };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderCloudSystem(
  state: CloudSystemState,
  render: RenderState,
  canvas: CanvasContext
): void {
  const { ctx, width, height } = canvas;
  const style = render.cloudSettings?.style ?? 'soft';
  const globalOpacity = render.cloudSettings?.opacity ?? 1.0;

  // Sort by layer (back to front)
  const sortedClouds = [...state.clouds].sort((a, b) => a.layer - b.layer);

  sortedClouds.forEach(cloud => {
    if (style === 'cartoon') {
      renderCartoonCloud(cloud, state.colorTint, ctx, width, height, globalOpacity);
    } else {
      renderAtmosphericCloud(cloud, state.colorTint, ctx, width, height, globalOpacity, render.timestamp ?? 0);
    }
  });
}

// -----------------------------------------------------------------------------
// Atmospheric Cloud Rendering (Soft, Natural)
// -----------------------------------------------------------------------------

function renderAtmosphericCloud(
  cloud: Cloud,
  tint: string,
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  globalOpacity: number = 1.0,
  timestamp: number = 0
): void {
  const x = (cloud.x / 100) * canvasWidth;
  const y = (cloud.y / 100) * canvasHeight;
  const baseSize = 80 * cloud.scale;

  ctx.save();
  ctx.translate(x, y);

  // Generate blobs for this cloud
  const blobs = generateAtmosphericBlobs(cloud.seed);

  // Parse tint color for blending
  const tintRgb = parseRgba(tint);

  // Render each blob with soft gradient
  blobs.forEach((blob, i) => {
    const blobX = blob.x * baseSize;
    const blobY = blob.y * baseSize;
    const radius = blob.size * baseSize * 1.8;

    // Slight time-based size variation (breathing effect)
    const breathe = 1 + Math.sin(timestamp * 0.0005 + cloud.seed + i) * 0.05;
    const finalRadius = radius * breathe;

    // Very low opacity for atmospheric feel
    const finalOpacity = cloud.opacity * globalOpacity * blob.opacity * 0.6;

    ctx.globalAlpha = finalOpacity;

    // Soft radial gradient (no hard edges)
    const gradient = ctx.createRadialGradient(blobX, blobY, 0, blobX, blobY, finalRadius);

    // Core is more solid, edges fade to nothing
    const coreColor = `rgba(${tintRgb.r}, ${tintRgb.g}, ${tintRgb.b}, 0.8)`;
    const midColor = `rgba(${tintRgb.r}, ${tintRgb.g}, ${tintRgb.b}, 0.3)`;
    const edgeColor = `rgba(${tintRgb.r}, ${tintRgb.g}, ${tintRgb.b}, 0)`;

    gradient.addColorStop(0, coreColor);
    gradient.addColorStop(0.3, midColor);
    gradient.addColorStop(0.7, midColor);
    gradient.addColorStop(1, edgeColor);

    ctx.fillStyle = gradient;
    ctx.beginPath();
    ctx.arc(blobX, blobY, finalRadius, 0, Math.PI * 2);
    ctx.fill();
  });

  ctx.restore();
}

// -----------------------------------------------------------------------------
// Cartoon Cloud Rendering (Original style, kept for option)
// -----------------------------------------------------------------------------

function renderCartoonCloud(
  cloud: Cloud,
  tint: string,
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  globalOpacity: number = 1.0
): void {
  const x = (cloud.x / 100) * canvasWidth;
  const y = (cloud.y / 100) * canvasHeight;
  const baseSize = 60 * cloud.scale;

  ctx.save();
  ctx.translate(x, y);
  ctx.globalAlpha = cloud.opacity * globalOpacity;

  const blobs = generateAtmosphericBlobs(cloud.seed);

  ctx.fillStyle = tint;

  blobs.forEach((blob) => {
    const circleX = blob.x * baseSize;
    const circleY = blob.y * baseSize;
    const radius = blob.size * baseSize;

    ctx.beginPath();
    ctx.arc(circleX, circleY, radius, 0, Math.PI * 2);
    ctx.fill();
  });

  // Subtle highlight
  ctx.fillStyle = 'rgba(255, 255, 255, 0.2)';
  blobs.slice(0, 2).forEach((blob) => {
    const circleX = blob.x * baseSize;
    const circleY = (blob.y - 0.1) * baseSize;
    const radius = blob.size * baseSize * 0.4;

    ctx.beginPath();
    ctx.arc(circleX, circleY, radius, 0, Math.PI * 2);
    ctx.fill();
  });

  ctx.restore();
}

// -----------------------------------------------------------------------------
// Utility: Parse RGBA string
// -----------------------------------------------------------------------------

function parseRgba(color: string): { r: number; g: number; b: number; a: number } {
  // Handle rgba() format
  const rgbaMatch = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/);
  if (rgbaMatch) {
    return {
      r: parseInt(rgbaMatch[1]),
      g: parseInt(rgbaMatch[2]),
      b: parseInt(rgbaMatch[3]),
      a: rgbaMatch[4] ? parseFloat(rgbaMatch[4]) : 1
    };
  }

  // Fallback: white
  return { r: 255, g: 255, b: 255, a: 0.7 };
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const cloudSystem = {
  init: initCloudSystem,
  update: updateCloudSystem,
  render: renderCloudSystem,
};
