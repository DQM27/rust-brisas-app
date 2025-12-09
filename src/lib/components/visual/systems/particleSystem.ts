// =============================================================================
// PARTICLE SYSTEM - Seasonal weather effects (snow, petals, leaves, etc.)
// =============================================================================

import type { 
  CanvasContext, 
  RenderState, 
  Particle, 
  ParticleConfig,
  ParticleSystemState,
  Season 
} from '../types';
import { 
  PARTICLE_CONFIGS, 
  TIME,
  randomRange, 
  clamp,
  getTimeOfDay 
} from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initParticleSystem(
  canvas: CanvasContext, 
  season: Season,
  isNight: boolean
): ParticleSystemState {
  const config = getParticleConfig(season, isNight);
  const particles = createParticles(config, canvas, true);
  
  return { particles, config };
}

function getParticleConfig(season: Season, isNight: boolean): ParticleConfig {
  const baseConfig = PARTICLE_CONFIGS[season];
  
  // Summer has different particles for day/night
  if (season === 'summer' && isNight && baseConfig.nightVariant) {
    return baseConfig.nightVariant;
  }
  
  return baseConfig;
}

function createParticles(
  config: ParticleConfig, 
  canvas: CanvasContext,
  randomizeY: boolean
): Particle[] {
  const particles: Particle[] = [];
  const { width, height } = canvas;
  
  for (let i = 0; i < config.count; i++) {
    particles.push(createParticle(config, width, height, randomizeY));
  }
  
  return particles;
}

function createParticle(
  config: ParticleConfig,
  canvasWidth: number,
  canvasHeight: number,
  randomizeY: boolean
): Particle {
  const type = getParticleType(config);
  const size = randomRange(config.sizeRange[0], config.sizeRange[1]);
  const baseSpeed = randomRange(config.speedRange[0], config.speedRange[1]);
  
  return {
    x: Math.random() * canvasWidth,
    y: randomizeY ? Math.random() * canvasHeight : -size * 2,
    vx: type === 'pollen' ? baseSpeed : randomRange(-0.5, 0.5),
    vy: type === 'firefly' ? randomRange(-0.2, 0.2) : baseSpeed,
    size,
    rotation: config.rotates ? Math.random() * 360 : 0,
    rotationSpeed: config.rotates ? randomRange(-3, 3) : 0,
    opacity: config.glows ? Math.random() : randomRange(0.6, 1),
    color: config.colors[Math.floor(Math.random() * config.colors.length)],
    type,
    glowPhase: config.glows ? Math.random() * Math.PI * 2 : undefined,
    glowSpeed: config.glows ? randomRange(1, 3) : undefined,
  };
}

function getParticleType(config: ParticleConfig): Particle['type'] {
  // Determine type based on config characteristics
  if (config.glows) return 'firefly';
  if (config.sizeRange[0] >= 6 && config.rotates) {
    // Could be petal or leaf based on colors
    if (config.colors.some(c => c.includes('ff') && c.includes('b7'))) return 'petal';
    return 'leaf';
  }
  if (config.sizeRange[1] <= 5 && !config.rotates) return 'pollen';
  return 'snowflake';
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateParticleSystem(
  state: ParticleSystemState,
  render: RenderState,
  canvas: CanvasContext
): ParticleSystemState {
  const { width, height } = canvas;
  const isNight = render.time >= TIME.DUSK_END || render.time < TIME.DAWN_START;
  
  // Check if we need to recreate particles (season or day/night changed)
  const newConfig = getParticleConfig(render.season, isNight);
  const configChanged = newConfig.count !== state.config.count || 
                        newConfig.glows !== state.config.glows;
  
  if (configChanged) {
    return {
      particles: createParticles(newConfig, canvas, true),
      config: newConfig,
    };
  }
  
  // Update existing particles
  const particles = state.particles.map(p => 
    updateParticle(p, render, canvas, state.config)
  );
  
  return { particles, config: state.config };
}

function updateParticle(
  particle: Particle,
  render: RenderState,
  canvas: CanvasContext,
  config: ParticleConfig
): Particle {
  const { width, height } = canvas;
  const dt = render.deltaTime / 16; // Normalize to 60fps
  const wind = render.wind.strength;
  
  let { x, y, vx, vy, rotation, opacity, glowPhase } = particle;
  
  // Apply wind
  const windEffect = wind * 0.5;
  
  switch (particle.type) {
    case 'snowflake':
      // Snow drifts with wind, falls steadily
      x += (vx + windEffect) * dt;
      y += vy * dt;
      // Add subtle wobble
      x += Math.sin(y * 0.02 + particle.size) * 0.3;
      break;
      
    case 'petal':
    case 'leaf':
      // Petals/leaves flutter and rotate
      x += (vx + windEffect * 1.5) * dt;
      y += vy * dt;
      // Oscillating horizontal movement
      x += Math.sin(y * 0.01 + particle.size * 10) * 1.5;
      rotation += particle.rotationSpeed * dt;
      break;
      
    case 'pollen':
      // Pollen floats with wind, slight vertical drift
      x += (vx + windEffect * 2) * dt;
      y += Math.sin(render.timestamp * 0.001 + particle.size * 100) * 0.3 * dt;
      break;
      
    case 'firefly':
      // Fireflies wander randomly, glow pulses
      x += (vx + Math.sin(render.timestamp * 0.002 + particle.size * 50) * 0.5) * dt;
      y += (vy + Math.cos(render.timestamp * 0.002 + particle.size * 30) * 0.5) * dt;
      
      // Update glow
      if (glowPhase !== undefined && particle.glowSpeed !== undefined) {
        glowPhase += particle.glowSpeed * 0.05 * dt;
        opacity = 0.3 + Math.sin(glowPhase) * 0.5 + 0.2;
      }
      break;
  }
  
  // Reset if out of bounds
  let needsReset = false;
  
  if (particle.type === 'firefly') {
    // Fireflies wrap around
    if (x < -20) x = width + 20;
    if (x > width + 20) x = -20;
    if (y < -20) y = height + 20;
    if (y > height + 20) y = -20;
  } else if (particle.type === 'pollen') {
    // Pollen exits right, enters left
    if (x > width + particle.size) {
      needsReset = true;
    }
  } else {
    // Snow/petals/leaves fall down and can drift sideways
    if (y > height + particle.size || x < -50 || x > width + 50) {
      needsReset = true;
    }
  }
  
  if (needsReset) {
    return createParticle(config, width, height, false);
  }
  
  return { ...particle, x, y, rotation, opacity, glowPhase };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderParticleSystem(
  state: ParticleSystemState,
  render: RenderState,
  canvas: CanvasContext
): void {
  const { ctx } = canvas;
  
  state.particles.forEach(particle => {
    renderParticle(particle, ctx, render.timestamp);
  });
}

function renderParticle(
  particle: Particle,
  ctx: CanvasRenderingContext2D,
  timestamp: number
): void {
  ctx.save();
  ctx.translate(particle.x, particle.y);
  ctx.globalAlpha = clamp(particle.opacity, 0, 1);
  
  switch (particle.type) {
    case 'snowflake':
      renderSnowflake(particle, ctx);
      break;
      
    case 'petal':
      renderPetal(particle, ctx);
      break;
      
    case 'leaf':
      renderLeaf(particle, ctx);
      break;
      
    case 'pollen':
      renderPollen(particle, ctx);
      break;
      
    case 'firefly':
      renderFirefly(particle, ctx);
      break;
  }
  
  ctx.restore();
}

function renderSnowflake(particle: Particle, ctx: CanvasRenderingContext2D): void {
  // Simple circle for cartoon snow
  const gradient = ctx.createRadialGradient(0, 0, 0, 0, 0, particle.size);
  gradient.addColorStop(0, particle.color);
  gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');
  
  ctx.fillStyle = gradient;
  ctx.beginPath();
  ctx.arc(0, 0, particle.size, 0, Math.PI * 2);
  ctx.fill();
}

function renderPetal(particle: Particle, ctx: CanvasRenderingContext2D): void {
  ctx.rotate((particle.rotation * Math.PI) / 180);
  
  // Ellipse petal shape
  ctx.fillStyle = particle.color;
  ctx.beginPath();
  ctx.ellipse(0, 0, particle.size * 0.5, particle.size, 0, 0, Math.PI * 2);
  ctx.fill();
  
  // Subtle center line
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(0, -particle.size * 0.7);
  ctx.lineTo(0, particle.size * 0.7);
  ctx.stroke();
}

function renderLeaf(particle: Particle, ctx: CanvasRenderingContext2D): void {
  ctx.rotate((particle.rotation * Math.PI) / 180);
  
  // Leaf shape (pointed ellipse)
  ctx.fillStyle = particle.color;
  ctx.beginPath();
  ctx.moveTo(0, -particle.size);
  ctx.quadraticCurveTo(particle.size * 0.5, -particle.size * 0.3, particle.size * 0.3, 0);
  ctx.quadraticCurveTo(particle.size * 0.5, particle.size * 0.3, 0, particle.size);
  ctx.quadraticCurveTo(-particle.size * 0.5, particle.size * 0.3, -particle.size * 0.3, 0);
  ctx.quadraticCurveTo(-particle.size * 0.5, -particle.size * 0.3, 0, -particle.size);
  ctx.fill();
  
  // Center vein
  ctx.strokeStyle = 'rgba(0, 0, 0, 0.2)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(0, -particle.size * 0.8);
  ctx.lineTo(0, particle.size * 0.8);
  ctx.stroke();
}

function renderPollen(particle: Particle, ctx: CanvasRenderingContext2D): void {
  // Tiny soft circle
  ctx.fillStyle = particle.color;
  ctx.beginPath();
  ctx.arc(0, 0, particle.size, 0, Math.PI * 2);
  ctx.fill();
}

function renderFirefly(particle: Particle, ctx: CanvasRenderingContext2D): void {
  // Glowing circle
  const glowSize = particle.size * (2 + particle.opacity);
  
  // Outer glow
  const gradient = ctx.createRadialGradient(0, 0, 0, 0, 0, glowSize);
  gradient.addColorStop(0, particle.color);
  gradient.addColorStop(0.3, `${particle.color}88`);
  gradient.addColorStop(1, `${particle.color}00`);
  
  ctx.fillStyle = gradient;
  ctx.beginPath();
  ctx.arc(0, 0, glowSize, 0, Math.PI * 2);
  ctx.fill();
  
  // Bright core
  ctx.fillStyle = '#ffffff';
  ctx.beginPath();
  ctx.arc(0, 0, particle.size * 0.3, 0, Math.PI * 2);
  ctx.fill();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const particleSystem = {
  init: initParticleSystem,
  update: updateParticleSystem,
  render: renderParticleSystem,
};
