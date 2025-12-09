// =============================================================================
// BOKEH SYSTEM - "Flent" effect (large out-of-focus particles)
// =============================================================================

import type {
    CanvasContext,
    RenderState,
    Particle,
    ParticleSystemState,
} from '../types';
import { randomRange, clamp } from '../constants';
import type { BokehConfig } from '$lib/stores/particleSettingsStore';

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

export interface BokehParticle extends Particle {
    targetOpacity: number;
    pulseSpeed: number;
}

export interface BokehSystemState {
    particles: BokehParticle[];
    config: BokehConfig; // Keep track of current config
}

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initBokehSystem(canvas: CanvasContext, config: BokehConfig): BokehSystemState {
    const particles = createBokehParticles(canvas, config);
    return { particles, config };
}

function createBokehParticles(canvas: CanvasContext, config: BokehConfig): BokehParticle[] {
    const particles: BokehParticle[] = [];
    const { width, height } = canvas;

    for (let i = 0; i < config.count; i++) {
        particles.push(createBokehParticle(width, height, config));
    }

    return particles;
}

function createBokehParticle(width: number, height: number, config: BokehConfig): BokehParticle {
    const size = randomRange(config.minSize, config.maxSize);
    const speed = randomRange(config.minSpeed, config.maxSpeed);
    const angle = Math.random() * Math.PI * 2;

    const COLORS = [
        'rgba(255, 220, 150, 1)',  // Warm yellow/gold
        'rgba(150, 255, 150, 1)', // Distinct green
        'rgba(255, 150, 150, 1)', // Distinct pink
        'rgba(150, 150, 255, 1)', // Distinct blue
        'rgba(255, 255, 255, 1)', // Pure white
    ];

    const baseColor = COLORS[Math.floor(Math.random() * COLORS.length)];

    return {
        x: Math.random() * width,
        y: Math.random() * height,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        size,
        rotation: 0,
        rotationSpeed: 0,
        opacity: 0, // Start invisible and fade in
        targetOpacity: randomRange(config.minOpacity, config.maxOpacity),
        pulseSpeed: randomRange(0.001, 0.003),
        color: baseColor,
        type: 'pollen',
        glowPhase: Math.random() * Math.PI * 2,
    };
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateBokehSystem(
    state: BokehSystemState,
    render: RenderState,
    canvas: CanvasContext,
    newConfig?: BokehConfig
): BokehSystemState {
    const { width, height } = canvas;
    const deltaTime = render.deltaTime;

    let currentConfig = state.config;
    let particles = state.particles;

    // Check for config config changes
    if (newConfig) {
        if (newConfig.count !== currentConfig.count) {
            if (newConfig.count > currentConfig.count) {
                // Add more
                const toAdd = newConfig.count - currentConfig.count;
                for (let i = 0; i < toAdd; i++) {
                    particles.push(createBokehParticle(width, height, newConfig));
                }
            } else {
                // Remove extras
                particles = particles.slice(0, newConfig.count);
            }
        }
        currentConfig = newConfig;
    }

    particles = particles.map(p => {
        let { x, y, vx, vy, glowPhase, opacity, pulseSpeed, targetOpacity, color } = p;

        // Movement
        x += vx * (deltaTime / 16);
        y += vy * (deltaTime / 16);

        // Gentle pulsing
        if (glowPhase !== undefined) {
            glowPhase += pulseSpeed * deltaTime;
            const maxOp = currentConfig.maxOpacity;
            opacity = (Math.sin(glowPhase) + 1) * 0.5 * Math.min(targetOpacity, maxOp);
        }

        // Wrap around screen
        if (x < -p.size * 2) x = width + p.size * 2;
        if (x > width + p.size * 2) x = -p.size * 2;
        if (y < -p.size * 2) y = height + p.size * 2;
        if (y > height + p.size * 2) y = -p.size * 2;

        return { ...p, x, y, glowPhase, opacity };
    });

    return { particles, config: currentConfig };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderBokehSystem(
    state: BokehSystemState,
    render: RenderState,
    canvas: CanvasContext
): void {
    const { ctx } = canvas;

    ctx.save();

    state.particles.forEach(p => {
        ctx.globalAlpha = clamp(p.opacity, 0, 1);
        ctx.fillStyle = p.color;
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fill();
    });

    ctx.restore();
}

// -----------------------------------------------------------------------------
// Export
// -----------------------------------------------------------------------------

export const bokehSystem = {
    init: initBokehSystem,
    update: updateBokehSystem,
    render: renderBokehSystem,
};
