// =============================================================================
// BOKEH SYSTEM - "Flent" effect (large out-of-focus particles)
// =============================================================================

import type { CanvasContext, RenderState, Particle } from '../types';
import { randomRange, clamp } from '../constants';
import { get } from 'svelte/store';
import { particleSettings } from '../../../stores/particleSettingsStore';

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

export interface BokehParticle extends Particle {
	targetOpacity: number;
	pulseSpeed: number;
}

export interface BokehSystemState {
	particles: BokehParticle[];
}

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

// Default config acts as fallback or initial state
const BOKEH_CONFIG = {
	COLORS: [
		'rgba(255, 220, 150, 0.15)', // Warm yellow/gold
		'rgba(150, 255, 150, 0.12)', // Distinct green
		'rgba(255, 150, 150, 0.12)', // Distinct pink
		'rgba(150, 150, 255, 0.12)', // Distinct blue
		'rgba(255, 255, 255, 0.08)' // Pure white
	],
	SPEED_RANGE: [0.03, 0.1] as [number, number]
};

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initBokehSystem(canvas: CanvasContext): BokehSystemState {
	const particles = createBokehParticles(canvas);
	return { particles };
}

function createBokehParticles(canvas: CanvasContext): BokehParticle[] {
	const settings = get(particleSettings);
	const particles: BokehParticle[] = [];
	const { width, height } = canvas;

	for (let i = 0; i < settings.bokehCount; i++) {
		particles.push(createBokehParticle(width, height));
	}

	return particles;
}

function createBokehParticle(width: number, height: number): BokehParticle {
	const settings = get(particleSettings);
	const size = randomRange(settings.bokehMinSize, settings.bokehMaxSize);
	const speed =
		randomRange(BOKEH_CONFIG.SPEED_RANGE[0], BOKEH_CONFIG.SPEED_RANGE[1]) *
		settings.bokehSpeedMultiplier;
	const angle = Math.random() * Math.PI * 2;

	return {
		x: Math.random() * width,
		y: Math.random() * height,
		vx: Math.cos(angle) * speed,
		vy: Math.sin(angle) * speed,
		size,
		rotation: 0,
		rotationSpeed: 0,
		opacity: 0,
		targetOpacity: randomRange(0.1, settings.bokehMaxOpacity),
		pulseSpeed: randomRange(0.001, 0.003),
		color: BOKEH_CONFIG.COLORS[Math.floor(Math.random() * BOKEH_CONFIG.COLORS.length)],
		type: 'pollen',
		glowPhase: Math.random() * Math.PI * 2
	};
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateBokehSystem(
	state: BokehSystemState,
	render: RenderState,
	canvas: CanvasContext
): BokehSystemState {
	const { width, height } = canvas;
	const deltaTime = render.deltaTime;

	// Read current settings
	const settings = get(particleSettings);

	// Handle count changes dynamically (simple approach: trim or add)
	if (state.particles.length !== settings.bokehCount) {
		if (state.particles.length < settings.bokehCount) {
			// Add missing
			for (let i = state.particles.length; i < settings.bokehCount; i++) {
				state.particles.push(createBokehParticle(width, height));
			}
		} else {
			// Remove excess
			state.particles = state.particles.slice(0, settings.bokehCount);
		}
	}

	const particles = state.particles.map((p) => {
		let { x, y, vx, vy, glowPhase, opacity, pulseSpeed, targetOpacity } = p;

		// Apply global speed multiplier
		const currentSpeedMultiplier = settings.bokehSpeedMultiplier;

		// Movement
		x += vx * (deltaTime / 16) * currentSpeedMultiplier;
		y += vy * (deltaTime / 16) * currentSpeedMultiplier;

		// Gentle pulsing
		if (glowPhase !== undefined) {
			glowPhase += pulseSpeed * deltaTime;
			// Cap opacity based on current settings
			const maxOpacity = settings.bokehMaxOpacity;
			// Re-calculate target opacity if it exceeds new max
			const effectiveTargetPayload = Math.min(targetOpacity, maxOpacity);

			opacity = (Math.sin(glowPhase) + 1) * 0.5 * effectiveTargetPayload;
		}

		// Wrap around screen
		if (x < -p.size * 2) x = width + p.size * 2;
		if (x > width + p.size * 2) x = -p.size * 2;
		if (y < -p.size * 2) y = height + p.size * 2;
		if (y > height + p.size * 2) y = -p.size * 2;

		return { ...p, x, y, glowPhase, opacity };
	});

	return { particles };
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

	// Composite operation for "glow" effect (additive blending might be too strong, sticking to source-over for subtle)
	// But for "flent", screen or lighter might look nice if background is dark
	ctx.save();
	// ctx.globalCompositeOperation = 'screen';

	state.particles.forEach((p) => {
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
	render: renderBokehSystem
};
