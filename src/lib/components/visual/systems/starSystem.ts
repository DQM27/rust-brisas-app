// =============================================================================
// STAR SYSTEM - Twinkling stars and shooting stars for night sky
// =============================================================================

import type { CanvasContext, RenderState, Star, ShootingStar, StarSystemState } from '../types';
import {
  TIME,
  STAR_CONFIG,
  seededRandom,
  randomRange,
  clamp,
  lerp
} from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initStarSystem(canvas: CanvasContext): StarSystemState {
  const stars: Star[] = [];

  // Create stars with seeded positions (consistent between renders)
  for (let i = 0; i < STAR_CONFIG.COUNT; i++) {
    const seed = i * 137.5; // Golden angle-ish seed
    const isLarge = seededRandom(seed + 1) < STAR_CONFIG.LARGE_STAR_CHANCE;

    stars.push({
      x: seededRandom(seed),
      y: seededRandom(seed + 0.5) * 0.6, // Stars in top 60% of sky
      size: isLarge
        ? randomRange(STAR_CONFIG.LARGE_STAR_MIN_SIZE, STAR_CONFIG.SIZE_RANGE[1])
        : randomRange(STAR_CONFIG.SIZE_RANGE[0], STAR_CONFIG.LARGE_STAR_MIN_SIZE),
      brightness: randomRange(0.5, 1),
      twinkleOffset: seededRandom(seed + 0.25) * Math.PI * 2,
      twinkleSpeed: randomRange(STAR_CONFIG.TWINKLE_SPEED_RANGE[0], STAR_CONFIG.TWINKLE_SPEED_RANGE[1]),
    });
  }

  return {
    stars,
    shootingStars: [],
    visibility: 0,
  };
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateStarSystem(
  state: StarSystemState,
  render: RenderState,
  canvas: CanvasContext
): StarSystemState {
  const hour = render.time;

  // Calculate visibility based on time (fade in at dusk, fade out at dawn)
  let visibility = 0;

  if (hour >= TIME.DUSK_END || hour < TIME.DAWN_START) {
    // Full night - full visibility
    visibility = 1;
  } else if (hour >= TIME.DUSK_START && hour < TIME.DUSK_END) {
    // Dusk - fade in
    visibility = (hour - TIME.DUSK_START) / (TIME.DUSK_END - TIME.DUSK_START);
  } else if (hour >= TIME.DAWN_START && hour < TIME.DAWN_END) {
    // Dawn - fade out
    visibility = 1 - (hour - TIME.DAWN_START) / (TIME.DAWN_END - TIME.DAWN_START);
  }

  // Update shooting stars
  let shootingStars = state.shootingStars
    .map(star => updateShootingStar(star, render.deltaTime, canvas, render.starSettings?.shootingStarSpeed))
    .filter(star => star.active);

  // Maybe spawn new shooting star (only at night)
  const baseChance = 0.0003;
  const freqMultiplier = render.starSettings?.shootingStarFrequency ?? 1.0;

  if (visibility > 0.5 && Math.random() < baseChance * freqMultiplier) {
    shootingStars.push(createShootingStar(canvas, render.starSettings?.shootingStarSpeed));
  }

  return {
    ...state,
    visibility,
    shootingStars,
  };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderStarSystem(
  state: StarSystemState,
  render: RenderState,
  canvas: CanvasContext
): void {
  if (state.visibility <= 0) return;

  const { ctx, width, height } = canvas;
  const time = render.timestamp / 1000; // Convert to seconds for animation

  ctx.save();
  ctx.globalAlpha = state.visibility;

  // Settings
  const countMultiplier = render.starSettings?.countMultiplier ?? 1.0;
  const twinkleSpeedMult = render.starSettings?.twinkleSpeed ?? 1.0;

  // Determine visible stars based on count multiplier
  // We limit the number of stars rendered from the pre-generated pool
  const visibleStars = Math.floor(state.stars.length * Math.min(1, countMultiplier));

  for (let i = 0; i < visibleStars; i++) {
    const star = state.stars[i];
    const x = star.x * width;
    const y = star.y * height;

    // Calculate twinkle
    // Use the multiplier from settings to speed up/slow down twinkle
    const twinkle = Math.sin(time * star.twinkleSpeed * twinkleSpeedMult + star.twinkleOffset);
    const brightness = star.brightness * (0.6 + 0.4 * twinkle);
    const currentSize = star.size * (0.9 + 0.1 * twinkle);

    // Draw glow for larger stars
    if (star.size >= STAR_CONFIG.LARGE_STAR_MIN_SIZE) {
      const gradient = ctx.createRadialGradient(x, y, 0, x, y, currentSize * 3);
      gradient.addColorStop(0, `rgba(255, 255, 240, ${brightness * 0.4})`);
      gradient.addColorStop(1, 'rgba(255, 255, 240, 0)');
      ctx.fillStyle = gradient;
      ctx.beginPath();
      ctx.arc(x, y, currentSize * 3, 0, Math.PI * 2);
      ctx.fill();
    }

    // Draw star core
    ctx.fillStyle = `rgba(255, 255, 250, ${brightness})`;
    ctx.beginPath();
    ctx.arc(x, y, currentSize / 2, 0, Math.PI * 2);
    ctx.fill();

    // Add subtle cross shape for larger stars (cartoon sparkle)
    if (star.size >= STAR_CONFIG.LARGE_STAR_MIN_SIZE) {
      ctx.strokeStyle = `rgba(255, 255, 255, ${brightness * 0.5})`;
      ctx.lineWidth = 1;
      const sparkleSize = currentSize * 1.5;

      ctx.beginPath();
      ctx.moveTo(x - sparkleSize, y);
      ctx.lineTo(x + sparkleSize, y);
      ctx.moveTo(x, y - sparkleSize);
      ctx.lineTo(x, y + sparkleSize);
      ctx.stroke();
    }
  }

  // Render shooting stars
  state.shootingStars.forEach(star => {
    renderShootingStar(star, ctx);
  });

  ctx.restore();
}

// -----------------------------------------------------------------------------
// Shooting Star Helpers
// -----------------------------------------------------------------------------

function createShootingStar(canvas: CanvasContext, speedMultiplier?: number): ShootingStar {
  const { width, height } = canvas;

  // Start from top-right area, go down-left
  return {
    x: width * (0.3 + Math.random() * 0.6),
    y: height * Math.random() * 0.3,
    angle: Math.PI * (0.6 + Math.random() * 0.3), // ~108-135 degrees (down-left)
    speed: STAR_CONFIG.SHOOTING_STAR_SPEED * (0.8 + Math.random() * 0.4) * (speedMultiplier ?? 1.0),
    length: STAR_CONFIG.SHOOTING_STAR_LENGTH * (0.7 + Math.random() * 0.6),
    opacity: 1,
    active: true,
  };
}

function updateShootingStar(
  star: ShootingStar,
  deltaTime: number,
  canvas: CanvasContext,
  speedMultiplier?: number
): ShootingStar {
  const modSpeed = star.speed * (speedMultiplier ?? 1.0);
  const speed = modSpeed * (deltaTime / 16); // Normalize to 60fps

  const newX = star.x + Math.cos(star.angle) * speed;
  const newY = star.y + Math.sin(star.angle) * speed;

  // Fade out as it travels
  const newOpacity = star.opacity - 0.015;

  // Deactivate if out of bounds or faded
  const active = newOpacity > 0 &&
    newX > -star.length &&
    newX < canvas.width + star.length &&
    newY < canvas.height;

  return {
    ...star,
    x: newX,
    y: newY,
    opacity: newOpacity,
    active,
  };
}

function renderShootingStar(star: ShootingStar, ctx: CanvasRenderingContext2D): void {
  const tailX = star.x - Math.cos(star.angle) * star.length;
  const tailY = star.y - Math.sin(star.angle) * star.length;

  // Create gradient from head to tail
  const gradient = ctx.createLinearGradient(star.x, star.y, tailX, tailY);
  gradient.addColorStop(0, `rgba(255, 255, 255, ${star.opacity})`);
  gradient.addColorStop(0.3, `rgba(255, 255, 200, ${star.opacity * 0.6})`);
  gradient.addColorStop(1, 'rgba(255, 255, 200, 0)');

  ctx.strokeStyle = gradient;
  ctx.lineWidth = 2;
  ctx.lineCap = 'round';

  ctx.beginPath();
  ctx.moveTo(star.x, star.y);
  ctx.lineTo(tailX, tailY);
  ctx.stroke();

  // Bright head
  ctx.fillStyle = `rgba(255, 255, 255, ${star.opacity})`;
  ctx.beginPath();
  ctx.arc(star.x, star.y, 2, 0, Math.PI * 2);
  ctx.fill();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const starSystem = {
  init: initStarSystem,
  update: updateStarSystem,
  render: renderStarSystem,
};
