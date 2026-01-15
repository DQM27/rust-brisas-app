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
import { PARTICLE_CONFIGS, TIME, randomRange, clamp } from '../constants';

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
	// Determine type first to check if it's weather-affected
	const type = getParticleType(config);

	const size = randomRange(config.sizeRange[0], config.sizeRange[1]);

	// Apply weather multipliers if available (passed via config or special logic, but here we might need to access them differently or pass them in)
	// Ideally, speed adjustment happens primarily in update, but initial speed also matters.
	// We'll stick to base speed here and modify effective speed in update to be dynamic with the slider.
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
		glowSpeed: config.glows ? randomRange(1, 3) : undefined
	};
}

// Placeholder - waiting for view_file result to know where to edit rendering logic.
// For now, I'll update the 'getParticleType' logic in particleSystem.ts to identify rain.

function getParticleType(config: ParticleConfig): Particle['type'] {
	if (config.glows) return 'firefly';
	// Rain logic
	if (config.speedRange[0] > 10) return 'rain'; // Simple heuristic for now or use specific prop if added

	if (config.sizeRange[0] >= 6 && config.rotates) {
		if (config.colors.some((c) => c.includes('ff') && c.includes('b7'))) return 'petal';
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

	// Apply density multiplier
	const densityMult = render.weatherSettings?.densityMultiplier ?? 1;
	const effectiveCount = Math.floor(newConfig.count * densityMult);

	// Check if we need to recreate (count changed or glows changed)
	const configChanged =
		effectiveCount !== state.particles.length || newConfig.glows !== state.config.glows;

	if (configChanged) {
		// Clone config and apply density to count
		const effectiveConfig = { ...newConfig, count: effectiveCount };
		return {
			particles: createParticles(effectiveConfig, canvas, true),
			config: newConfig // Keep original config for reference? Or update it? state.config is used for checking changes.
			// Only the count changes dynamically.
		};
	}

	// Update existing particles
	const particles = state.particles.map((p) => updateParticle(p, render, canvas, state.config));

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

	// Get multipliers (default to 1 if missing for safety)
	const _densityMult = render.weatherSettings?.densityMultiplier ?? 1;
	const speedMult = render.weatherSettings?.speedMultiplier ?? 1;
	const sizeMult = render.weatherSettings?.sizeMultiplier ?? 1;
	const windMult = render.weatherSettings?.windInfluence ?? 1;
	const turbMult = render.weatherSettings?.turbulence ?? 1;

	let { x, y, vx, vy, rotation, opacity, glowPhase } = particle;

	// Apply visual size scaling via a temporary scale factor? No, we likely need to scale drawing.
	// Actually, we can just treat particle.size as base size and apply multiplier during render?
	// Or modify effective size here? No, 'particle.size' is state.
	// Best approach: Use sizeMult in the render function.

	// Apply wind
	const windEffect = wind * 0.5 * windMult;

	switch (particle.type) {
		case 'snowflake':
			// Snow drifts with wind, falls steadily
			x += (vx + windEffect) * dt * speedMult;
			y += vy * dt * speedMult;
			// Add subtle wobble
			x += Math.sin(y * 0.02 + particle.size) * 0.3 * turbMult;
			break;

		case 'petal':
		case 'leaf':
			// Petals/leaves flutter and rotate
			x += (vx + windEffect * 1.5) * dt * speedMult;
			y += vy * dt * speedMult;
			// Oscillating horizontal movement
			x += Math.sin(y * 0.01 + particle.size * 10) * 1.5 * turbMult;
			rotation += particle.rotationSpeed * dt * speedMult;
			break;

		case 'pollen':
			// Pollen floats with wind, organic wavy motion
			x += (vx + windEffect * 0.8) * dt * speedMult;
			// Compound sine waves for organic float
			y += (Math.sin(render.timestamp * 0.001 + particle.size * 10) * 0.2 + 0.1) * dt * speedMult;
			// Slight random wobble
			x += Math.cos(render.timestamp * 0.0015 + particle.y * 0.01) * 0.1 * dt * turbMult;
			break;

		case 'firefly':
			// Fireflies wander randomly, glow pulses
			x +=
				(vx + Math.sin(render.timestamp * 0.002 + particle.size * 50) * 0.5 * turbMult) *
				dt *
				speedMult;
			y +=
				(vy + Math.cos(render.timestamp * 0.002 + particle.size * 30) * 0.5 * turbMult) *
				dt *
				speedMult;

			// Update glow
			if (glowPhase !== undefined && particle.glowSpeed !== undefined) {
				glowPhase += particle.glowSpeed * 0.05 * dt;
				opacity = 0.3 + Math.sin(glowPhase) * 0.5 + 0.2;
			}

			break;

		case 'rain':
			// Basic rain fall with wind slant
			x += (vx + windEffect * 0.2) * dt * speedMult;
			y += vy * dt * speedMult;
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
	} else if (particle.type === 'rain') {
		// Rain resets when hitting bottom
		if (y > height) needsReset = true;
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

	state.particles.forEach((particle) => {
		renderParticle(particle, ctx, render);
	});
}

function renderParticle(
	particle: Particle,
	ctx: CanvasRenderingContext2D,
	render: RenderState
): void {
	const sizeMult = render.weatherSettings?.sizeMultiplier ?? 1;

	ctx.save();
	ctx.translate(particle.x, particle.y);
	// Apply size multiplier here for all drawing operations
	if (sizeMult !== 1) {
		ctx.scale(sizeMult, sizeMult);
	}
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

		case 'rain':
			renderRain(particle, ctx);
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
	ctx.fill();
}

function renderRain(particle: Particle, ctx: CanvasRenderingContext2D): void {
	ctx.strokeStyle = particle.color;
	ctx.lineWidth = Math.max(1, particle.size / 2);
	ctx.lineCap = 'round';

	// Draw streak
	ctx.beginPath();
	ctx.moveTo(0, 0); // Translated to x,y
	// Length relates to speed? Or just fixed length based on size
	// Let's use speed-based length for motion blur effect
	const length = particle.vy * 0.5 + particle.size * 2;

	// Angle based on wind? Ideally yes, but for now vertical streak
	// We can use vx/vy to determine angle if we want strict accuracy, usually rain falls straight or slanted
	// Let's just draw line down-ish
	ctx.lineTo(particle.vx * 0.2, length);
	ctx.stroke();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const particleSystem = {
	init: initParticleSystem,
	update: updateParticleSystem,
	render: renderParticleSystem
};
