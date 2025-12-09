// =============================================================================
// CLOUD SYSTEM - Cartoon blob clouds with parallax movement
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
// Initialization
// -----------------------------------------------------------------------------

export function initCloudSystem(canvas: CanvasContext): CloudSystemState {
  const clouds: Cloud[] = [];
  // Start with default count
  for (let i = 0; i < CLOUD_CONFIG.COUNT; i++) {
    clouds.push(createCloud(i, canvas));
  }

  return {
    clouds,
    colorTint: CLOUD_CONFIG.TINTS.day,
  };
}

function createCloud(index: number, canvas: CanvasContext, startOffScreen = false): Cloud {
  const seed = index * 42.7;
  const layer = Math.floor(seededRandom(seed) * CLOUD_CONFIG.LAYERS);
  const scaleRange = CLOUD_CONFIG.LAYER_SCALES[layer];
  const y = randomRange(CLOUD_CONFIG.Y_RANGE[0], CLOUD_CONFIG.Y_RANGE[1]);

  return {
    x: startOffScreen ? -30 : seededRandom(seed + 0.1) * 130 - 15, // -15% to 115%
    y: y,
    baseY: y, // Store base Y for turbulence
    scale: randomRange(scaleRange[0], scaleRange[1]),
    speed: CLOUD_CONFIG.LAYER_SPEEDS[layer],
    opacity: 0.6 + layer * 0.15, // Front clouds more opaque
    layer,
    seed, // For consistent blob shape
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

  // Manage cloud count - Push new clouds ON SCREEN (random position) so they appear instantly
  while (state.clouds.length < targetCount) {
    state.clouds.push(createCloud(state.clouds.length, canvas, false));
  }
  while (state.clouds.length > targetCount) {
    state.clouds.pop();
  }

  // Update cloud positions
  const clouds = state.clouds.map((cloud, index) => {
    // 1. Horizontal Movement (Wind + Base Speed)
    // Apply Cloud Wind Multiplier
    const windEffect = render.wind.strength * 0.3 * windMult;
    const baseSpeed = cloud.speed * windMult;

    let newX = cloud.x + (baseSpeed + windEffect) * (render.deltaTime / 16);

    // Reset cloud if it goes off screen
    if (newX > 120) {
      return createCloud(index, canvas, true);
    }

    // 2. Vertical Turbulence (Wobble)
    // Use cloud seed and global time for sine wave
    let newY = cloud.baseY;
    if (turbulenceStr > 0) {
      // Wobble speed and magnitude based on turbulence setting
      const time = render.timestamp ?? Date.now();
      const wobble = Math.sin((time * 0.001) + cloud.seed) * (turbulenceStr * 10);
      newY += wobble;
    }

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
  const style = render.cloudSettings?.style ?? 'cartoon';
  const globalOpacity = render.cloudSettings?.opacity ?? 1.0;

  // Sort by layer (back to front)
  const sortedClouds = [...state.clouds].sort((a, b) => a.layer - b.layer);

  sortedClouds.forEach(cloud => {
    if (style === 'soft') {
      renderSoftCloud(cloud, state.colorTint, ctx, width, height, globalOpacity);
    } else {
      renderCloud(cloud, state.colorTint, ctx, width, height, globalOpacity);
    }
  });
}

// -----------------------------------------------------------------------------
// Cloud Rendering (Cartoon Blob Style)
// -----------------------------------------------------------------------------

function renderCloud(
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

  // Generate consistent blob shape based on seed
  const blobOffsets = generateBlobOffsets(cloud.seed);

  // Draw cloud as overlapping circles (cartoon style)
  ctx.fillStyle = tint;

  // Main body circles
  blobOffsets.forEach((offset, i) => {
    const circleX = offset.x * baseSize;
    const circleY = offset.y * baseSize;
    const radius = offset.size * baseSize;

    ctx.beginPath();
    ctx.arc(circleX, circleY, radius, 0, Math.PI * 2);
    ctx.fill();
  });

  // Add subtle highlight on top
  ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
  blobOffsets.slice(0, 2).forEach((offset) => {
    const circleX = offset.x * baseSize;
    const circleY = (offset.y - 0.15) * baseSize; // Slightly above
    const radius = offset.size * baseSize * 0.5;

    ctx.beginPath();
    ctx.arc(circleX, circleY, radius, 0, Math.PI * 2);
    ctx.fill();
  });

  ctx.restore();
}

function renderSoftCloud(
  cloud: Cloud,
  tint: string,
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  globalOpacity: number = 1.0
): void {
  const x = (cloud.x / 100) * canvasWidth;
  const y = (cloud.y / 100) * canvasHeight;
  const baseSize = 70 * cloud.scale; // Slightly larger base for soft

  ctx.save();
  ctx.translate(x, y);
  ctx.globalAlpha = cloud.opacity * globalOpacity * 0.8; // Slightly more transparent overall

  const blobOffsets = generateBlobOffsets(cloud.seed);

  // Use a soft gradient for each blob
  blobOffsets.forEach((offset, i) => {
    const circleX = offset.x * baseSize;
    const circleY = offset.y * baseSize;
    const radius = offset.size * baseSize * 1.5; // Larger radius for soft edges

    const gradient = ctx.createRadialGradient(circleX, circleY, 0, circleX, circleY, radius);
    gradient.addColorStop(0, tint);
    gradient.addColorStop(0.4, tint); // Solid core
    gradient.addColorStop(1, 'rgba(255, 255, 255, 0)'); // Fade out

    ctx.fillStyle = gradient;
    ctx.beginPath();
    ctx.arc(circleX, circleY, radius, 0, Math.PI * 2);
    ctx.fill();
  });

  ctx.restore();
}

// Generate consistent blob pattern for a cloud
function generateBlobOffsets(seed: number): Array<{ x: number; y: number; size: number }> {
  const offsets: Array<{ x: number; y: number; size: number }> = [];

  // Main center blob
  offsets.push({ x: 0, y: 0, size: 0.5 });

  // Surrounding blobs (5-7 circles)
  const blobCount = 5 + Math.floor(seededRandom(seed) * 3);

  for (let i = 0; i < blobCount; i++) {
    const angle = (i / blobCount) * Math.PI * 2 + seededRandom(seed + i) * 0.5;
    const distance = 0.3 + seededRandom(seed + i + 0.5) * 0.3;
    const size = 0.3 + seededRandom(seed + i + 0.25) * 0.25;

    offsets.push({
      x: Math.cos(angle) * distance,
      y: Math.sin(angle) * distance * 0.6, // Flatten vertically
      size,
    });
  }

  return offsets;
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const cloudSystem = {
  init: initCloudSystem,
  update: updateCloudSystem,
  render: renderCloudSystem,
};
