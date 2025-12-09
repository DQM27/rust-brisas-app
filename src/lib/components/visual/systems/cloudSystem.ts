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
  
  return {
    x: startOffScreen ? -30 : seededRandom(seed + 0.1) * 130 - 15, // -15% to 115%
    y: randomRange(CLOUD_CONFIG.Y_RANGE[0], CLOUD_CONFIG.Y_RANGE[1]),
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
  
  // Update cloud positions
  const clouds = state.clouds.map((cloud, index) => {
    // Move cloud based on wind and base speed
    const windEffect = render.wind.strength * 0.3;
    let newX = cloud.x + (cloud.speed + windEffect) * (render.deltaTime / 16);
    
    // Reset cloud if it goes off screen
    if (newX > 120) {
      return createCloud(index, canvas, true);
    }
    
    return { ...cloud, x: newX };
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
  
  // Sort by layer (back to front)
  const sortedClouds = [...state.clouds].sort((a, b) => a.layer - b.layer);
  
  sortedClouds.forEach(cloud => {
    renderCloud(cloud, state.colorTint, ctx, width, height);
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
  canvasHeight: number
): void {
  const x = (cloud.x / 100) * canvasWidth;
  const y = (cloud.y / 100) * canvasHeight;
  const baseSize = 60 * cloud.scale;
  
  ctx.save();
  ctx.translate(x, y);
  ctx.globalAlpha = cloud.opacity;
  
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
