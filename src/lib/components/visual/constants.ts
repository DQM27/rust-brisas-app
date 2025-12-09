// =============================================================================
// BRISAS VISUAL SYSTEM v2 - Constants
// =============================================================================

import type { Season, ParticleConfig, TimeOfDay } from './types';

// -----------------------------------------------------------------------------
// Time Constants (centralized to avoid desync between components)
// -----------------------------------------------------------------------------

export const TIME = {
  // Dawn transition
  DAWN_START: 5.5,      // 5:30 AM - Moon starts descending, stars fade
  SUNRISE: 6,           // 6:00 AM - Sun appears at horizon
  DAWN_END: 6.5,        // 6:30 AM - Full day mode

  // Dusk transition  
  DUSK_START: 17.5,     // 5:30 PM - Sun starts descending
  SUNSET: 18,           // 6:00 PM - Sun at horizon, moon appears
  DUSK_END: 18.5,       // 6:30 PM - Full night mode

  // For celestial arc calculation
  SUN_ARC_START: 5.5,   // When sun enters screen
  SUN_ARC_END: 18.5,    // When sun exits screen
  MOON_ARC_START: 17.5, // When moon enters screen
  MOON_ARC_END: 6.5,    // When moon exits screen (next day)
} as const;

// Helper to get time of day
export function getTimeOfDay(hour: number): TimeOfDay {
  if (hour >= TIME.DUSK_END || hour < TIME.DAWN_START) return 'night';
  if (hour >= TIME.DAWN_START && hour < TIME.SUNRISE) return 'dawn';
  if (hour >= TIME.SUNRISE && hour < 9) return 'morning';
  if (hour >= 9 && hour < TIME.DUSK_START) return 'day';
  if (hour >= TIME.DUSK_START && hour < TIME.SUNSET) return 'dusk';
  return 'evening'; // SUNSET to DUSK_END
}

// -----------------------------------------------------------------------------
// Sky Color Palettes (Cartoon style - vibrant but harmonious)
// -----------------------------------------------------------------------------

// Each palette has 3 stops: top, middle, bottom
export const SKY_PALETTES: Record<TimeOfDay, [string, string, string]> = {
  night: ['#0a0e1a', '#1a1f3a', '#2a2f4a'],           // Deep blue-purple
  dawn: ['#2a2040', '#6b4a6e', '#ff9a7b'],            // Purple to pink-orange
  morning: ['#5a9fd4', '#87ceeb', '#c5e8ff'],         // Fresh blue
  day: ['#4a90c2', '#7ec8e3', '#c8e6f5'],             // Clear sky blue
  dusk: ['#7b4a7b', '#d4726a', '#ffb088'],            // Purple-orange
  evening: ['#2a2050', '#4a3060', '#6a4070'],         // Deep purple
};

// Interpolate between two palettes based on progress (0-1)
export function interpolateSkyPalette(
  from: [string, string, string],
  to: [string, string, string],
  progress: number
): [string, string, string] {
  return [
    interpolateColor(from[0], to[0], progress),
    interpolateColor(from[1], to[1], progress),
    interpolateColor(from[2], to[2], progress),
  ];
}

// -----------------------------------------------------------------------------
// Mountain Color Themes (per season - cartoon style)
// -----------------------------------------------------------------------------

// [far, mid, near] mountains
export const MOUNTAIN_THEMES: Record<Season, [string, string, string]> = {
  spring: ['#7cb98a', '#5a9a6a', '#3d7a4a'],     // Fresh greens
  summer: ['#8bc990', '#6aaa70', '#4a8a50'],     // Lush greens
  autumn: ['#d4855a', '#c06a3a', '#9a4a2a'],     // Warm oranges/browns
  winter: ['#6a7a8a', '#4a5a6a', '#3a4a5a'],     // Cool blue-grays
  rain: ['#4a5a4a', '#3a4a3a', '#2a3a2a'],       // Dark/Moody greens for rain
};

// Night modifier - darken mountains at night
export function getMountainBrightness(hour: number): number {
  const tod = getTimeOfDay(hour);
  switch (tod) {
    case 'night': return 0.3;
    case 'dawn':
    case 'evening': return 0.5;
    case 'dusk': return 0.7;
    default: return 1;
  }
}

// -----------------------------------------------------------------------------
// Star Configuration
// -----------------------------------------------------------------------------

export const STAR_CONFIG = {
  COUNT: 340, // More stars to allow for >1x multiplier effect (default 1x shows half)
  SIZE_RANGE: [1.5, 4] as [number, number],
  TWINKLE_SPEED_RANGE: [0.5, 2] as [number, number],

  // Large stars (with glow) percentage
  LARGE_STAR_CHANCE: 0.15,
  LARGE_STAR_MIN_SIZE: 3,

  // Shooting stars
  SHOOTING_STAR_INTERVAL: [30000, 60000] as [number, number], // ms
  SHOOTING_STAR_SPEED: 15,
  SHOOTING_STAR_LENGTH: 80,
};

// -----------------------------------------------------------------------------
// Cloud Configuration
// -----------------------------------------------------------------------------

export const CLOUD_CONFIG = {
  COUNT: 5,
  LAYERS: 3,

  // Speed per layer (parallax - back layers slower)
  LAYER_SPEEDS: [0.005, 0.01, 0.02],

  // Scale range per layer
  LAYER_SCALES: [
    [0.6, 0.8],   // Back - smaller
    [0.8, 1.0],   // Mid
    [1.0, 1.3],   // Front - larger
  ] as [number, number][],

  // Y position range (percentage from top)
  Y_RANGE: [5, 35] as [number, number],

  // Color tints based on time
  TINTS: {
    night: 'rgba(40, 50, 70, 0.3)',
    dawn: 'rgba(255, 180, 150, 0.6)',
    morning: 'rgba(255, 255, 255, 0.8)',
    day: 'rgba(255, 255, 255, 0.9)',
    dusk: 'rgba(255, 150, 120, 0.7)',
    evening: 'rgba(100, 80, 120, 0.4)',
  } as Record<TimeOfDay, string>,
};

// -----------------------------------------------------------------------------
// Celestial Bodies Configuration
// -----------------------------------------------------------------------------

export const CELESTIAL_CONFIG = {
  // Sun
  SUN_SIZE: 80,                    // Tamaño más grande
  SUN_COLORS: {
    core: '#ffdd44',
    glow: 'rgba(255, 200, 100, 0.5)',
    glowOuter: 'rgba(255, 160, 60, 0.3)',
  },

  // Moon
  MOON_SIZE: 80,                   // Tamaño más grande (igual que sol)
  MOON_COLORS: {
    fill: '#f0f0ff',
    glow: 'rgba(200, 210, 255, 0.4)',
  },

  // Arc path - MÁS AMPLIO
  ARC_TOP: 20,                   // Qué tan arriba llega (% desde top) - antes era 70
  ARC_BOTTOM: 115,               // Dónde entra/sale (% desde top) - un poco abajo de pantalla
};

// -----------------------------------------------------------------------------
// Weather Particles Configuration (per season)
// -----------------------------------------------------------------------------

export const PARTICLE_CONFIGS: Record<Season, ParticleConfig & { nightVariant?: ParticleConfig }> = {
  winter: {
    count: 80,
    colors: ['#ffffff', '#e8f4ff', '#d0e8ff'],
    sizeRange: [3, 7],
    speedRange: [0.5, 1.5],
    rotates: false,
    glows: false,
  },

  spring: {
    count: 40,
    colors: ['#ffb7c5', '#ffc0cb', '#ffe4e8', '#ffffff'],
    sizeRange: [6, 12],
    speedRange: [0.3, 0.8],
    rotates: true,
    glows: false,
  },

  summer: {
    // Day - pollen/dust/dandelions
    count: 60,
    colors: ['#fffdd0', '#fff8c0', '#ffffff', '#fffaf0'], // Cream/White mix
    sizeRange: [2, 5],
    speedRange: [0.3, 1.0], // Slower, more drifting
    rotates: true, // Some might be dandelions
    glows: false,
    // Night - fireflies
    nightVariant: {
      count: 40,
      colors: ['#ffff88', '#ccffcc', '#ffffff', '#e0ffe0', '#ffffe0'], // Yellow, Green, White mix
      sizeRange: [2, 4],
      speedRange: [0.5, 2.0], // Erratic movement
      rotates: false,
      glows: true,
    },
  },

  autumn: {
    count: 50,
    colors: ['#d4855a', '#c06a3a', '#9a4a2a', '#e8b88a'],
    sizeRange: [6, 12],
    speedRange: [1.0, 3.0],
    rotates: true,
    glows: false,
  },

  rain: {
    count: 300,
    colors: ['#a0c0ff', '#80a0e0', '#6080c0'],
    sizeRange: [2, 4], // Using "size" for length/width ratio
    speedRange: [15, 25], // Falling fast
    rotates: false,
    glows: false,
  },
};


// -----------------------------------------------------------------------------
// Wind Configuration
// -----------------------------------------------------------------------------

export const WIND_CONFIG = {
  BASE_STRENGTH: 0.3,
  MAX_STRENGTH: 2.5,
  CHANGE_SPEED: 0.01,          // How fast wind changes
  GUST_CHANCE: 0.002,          // Per frame chance of gust
  GUST_STRENGTH: 2,            // Additional strength during gust
  GUST_DURATION: 2000,         // ms
};

// -----------------------------------------------------------------------------
// Birthday Configuration
// -----------------------------------------------------------------------------

export const BIRTHDAY_CONFIG = {
  // Confetti
  CONFETTI_COUNT: 120,
  CONFETTI_COLORS: [
    '#ff6b6b', '#ffd93d', '#6bcb77', '#4d96ff',
    '#ff6bff', '#ffa500', '#00ff88', '#ff1493',
  ],
  CONFETTI_SIZE_RANGE: [6, 12] as [number, number],

  // Fireworks
  FIREWORK_INTERVAL: [3000, 6000] as [number, number],
  FIREWORK_COLORS: ['#ff6b6b', '#ffd93d', '#6bcb77', '#4d96ff', '#ff6bff'],
  FIREWORK_PARTICLE_COUNT: 30,

  // Background gradient (animated)
  BG_GRADIENT_COLORS: [
    '#667eea', '#764ba2', '#f093fb', '#f5576c',
    '#4facfe', '#00f2fe', '#43e97b', '#38f9d7',
  ],
};

// -----------------------------------------------------------------------------
// Utility Functions
// -----------------------------------------------------------------------------

// Linear interpolation
export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

// Clamp value between min and max
export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

// Random number in range
export function randomRange(min: number, max: number): number {
  return min + Math.random() * (max - min);
}

// Random integer in range (inclusive)
export function randomInt(min: number, max: number): number {
  return Math.floor(randomRange(min, max + 1));
}

// Seeded random for consistent values
export function seededRandom(seed: number): number {
  const x = Math.sin(seed * 12.9898) * 43758.5453;
  return x - Math.floor(x);
}

// Hex to RGB
export function hexToRgb(hex: string): { r: number; g: number; b: number } {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? {
    r: parseInt(result[1], 16),
    g: parseInt(result[2], 16),
    b: parseInt(result[3], 16),
  } : { r: 0, g: 0, b: 0 };
}

// RGB to Hex
export function rgbToHex(r: number, g: number, b: number): string {
  return '#' + [r, g, b].map(x => {
    const hex = Math.round(clamp(x, 0, 255)).toString(16);
    return hex.length === 1 ? '0' + hex : hex;
  }).join('');
}

// Interpolate between two hex colors
export function interpolateColor(color1: string, color2: string, t: number): string {
  const c1 = hexToRgb(color1);
  const c2 = hexToRgb(color2);
  return rgbToHex(
    lerp(c1.r, c2.r, t),
    lerp(c1.g, c2.g, t),
    lerp(c1.b, c2.b, t)
  );
}

// Easing functions for animations
export const easing = {
  // Smooth start and end
  easeInOut: (t: number) => t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2,

  // Bouncy (for cartoon feel)
  easeOutBack: (t: number) => {
    const c1 = 1.70158;
    const c3 = c1 + 1;
    return 1 + c3 * Math.pow(t - 1, 3) + c1 * Math.pow(t - 1, 2);
  },

  // Elastic (for birthday effects)
  easeOutElastic: (t: number) => {
    const c4 = (2 * Math.PI) / 3;
    return t === 0 ? 0 : t === 1 ? 1 : Math.pow(2, -10 * t) * Math.sin((t * 10 - 0.75) * c4) + 1;
  },
};
