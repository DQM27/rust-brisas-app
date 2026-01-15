// =============================================================================
// BIRTHDAY SYSTEM - Confetti and fireworks celebration!
// =============================================================================

import type {
	CanvasContext,
	RenderState,
	Confetti,
	Firework,
	FireworkParticle,
	BirthdaySystemState
} from '../types';
import { BIRTHDAY_CONFIG, randomRange, clamp } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initBirthdaySystem(canvas: CanvasContext): BirthdaySystemState {
	const confetti = createConfettiBurst(canvas);

	return {
		confetti,
		fireworks: [],
		nextFireworkTimer: randomRange(
			BIRTHDAY_CONFIG.FIREWORK_INTERVAL[0],
			BIRTHDAY_CONFIG.FIREWORK_INTERVAL[1]
		)
	};
}

// Create initial confetti burst
function createConfettiBurst(canvas: CanvasContext): Confetti[] {
	const { width, height } = canvas;
	const confetti: Confetti[] = [];

	for (let i = 0; i < BIRTHDAY_CONFIG.CONFETTI_COUNT; i++) {
		confetti.push(createConfetti(width, height, true));
	}

	return confetti;
}

function createConfetti(canvasWidth: number, canvasHeight: number, burst: boolean): Confetti {
	const shapes: Confetti['shape'][] = ['rect', 'circle', 'serpentine'];

	return {
		x: burst ? canvasWidth / 2 + randomRange(-100, 100) : Math.random() * canvasWidth,
		y: burst ? canvasHeight * 0.3 : -20,
		vx: burst ? randomRange(-8, 8) : randomRange(-2, 2),
		vy: burst ? randomRange(-15, -5) : randomRange(1, 3),
		rotation: Math.random() * 360,
		rotationSpeed: randomRange(-10, 10),
		color:
			BIRTHDAY_CONFIG.CONFETTI_COLORS[
			Math.floor(Math.random() * BIRTHDAY_CONFIG.CONFETTI_COLORS.length)
			],
		shape: shapes[Math.floor(Math.random() * shapes.length)],
		size: randomRange(
			BIRTHDAY_CONFIG.CONFETTI_SIZE_RANGE[0],
			BIRTHDAY_CONFIG.CONFETTI_SIZE_RANGE[1]
		),
		wobble: Math.random() * Math.PI * 2,
		wobbleSpeed: randomRange(0.05, 0.15)
	};
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateBirthdaySystem(
	state: BirthdaySystemState,
	render: RenderState,
	canvas: CanvasContext
): BirthdaySystemState {
	const { width, height } = canvas;
	const dt = render.deltaTime / 16;

	// Update confetti
	let confetti = state.confetti.map((c) => updateConfetti(c, dt, width, height));

	// Respawn confetti that fell off screen (continuous rain)
	confetti = confetti.map((c) => {
		if (c.y > height + 50) {
			return createConfetti(width, height, false);
		}
		return c;
	});

	// Update fireworks
	const fireworks = state.fireworks
		.map((f) => updateFirework(f, dt))
		.filter((f) => !isFireworkDead(f));

	// Timer for new fireworks
	let nextFireworkTimer = state.nextFireworkTimer - render.deltaTime;

	if (nextFireworkTimer <= 0) {
		fireworks.push(createFirework(width, height));
		nextFireworkTimer = randomRange(
			BIRTHDAY_CONFIG.FIREWORK_INTERVAL[0],
			BIRTHDAY_CONFIG.FIREWORK_INTERVAL[1]
		);
	}

	return { confetti, fireworks, nextFireworkTimer };
}

function updateConfetti(
	confetti: Confetti,
	dt: number,
	_canvasWidth: number,
	_canvasHeight: number
): Confetti {
	let { x, y, vx, vy, rotation, wobble } = confetti;

	// Physics
	vy += 0.15 * dt; // Gravity
	vy = Math.min(vy, 5); // Terminal velocity

	// Air resistance
	vx *= 0.99;

	// Wobble (side to side)
	wobble += confetti.wobbleSpeed * dt;
	const wobbleX = Math.sin(wobble) * 2;

	x += (vx + wobbleX) * dt;
	y += vy * dt;
	rotation += confetti.rotationSpeed * dt;

	return { ...confetti, x, y, vx, vy, rotation, wobble };
}

// -----------------------------------------------------------------------------
// Fireworks
// -----------------------------------------------------------------------------

function createFirework(canvasWidth: number, canvasHeight: number): Firework {
	return {
		x: randomRange(canvasWidth * 0.2, canvasWidth * 0.8),
		y: canvasHeight,
		vy: randomRange(-12, -8),
		targetY: randomRange(canvasHeight * 0.15, canvasHeight * 0.4),
		color:
			BIRTHDAY_CONFIG.FIREWORK_COLORS[
			Math.floor(Math.random() * BIRTHDAY_CONFIG.FIREWORK_COLORS.length)
			],
		exploded: false,
		particles: []
	};
}

function updateFirework(firework: Firework, dt: number): Firework {
	if (!firework.exploded) {
		// Rising phase
		const y = firework.y + firework.vy * dt;

		if (y <= firework.targetY) {
			// Explode!
			return {
				...firework,
				y: firework.targetY,
				exploded: true,
				particles: createExplosionParticles(firework)
			};
		}

		return { ...firework, y };
	} else {
		// Explosion phase - update particles
		const particles = firework.particles
			.map((p) => updateFireworkParticle(p, dt))
			.filter((p) => p.opacity > 0);

		return { ...firework, particles };
	}
}

function createExplosionParticles(firework: Firework): FireworkParticle[] {
	const particles: FireworkParticle[] = [];
	const count = BIRTHDAY_CONFIG.FIREWORK_PARTICLE_COUNT;

	for (let i = 0; i < count; i++) {
		const angle = (i / count) * Math.PI * 2;
		const speed = randomRange(3, 7);

		particles.push({
			x: firework.x,
			y: firework.y,
			vx: Math.cos(angle) * speed,
			vy: Math.sin(angle) * speed,
			opacity: 1,
			color: firework.color
		});
	}

	return particles;
}

function updateFireworkParticle(particle: FireworkParticle, dt: number): FireworkParticle {
	return {
		...particle,
		x: particle.x + particle.vx * dt,
		y: particle.y + particle.vy * dt,
		vy: particle.vy + 0.1 * dt, // Gravity
		vx: particle.vx * 0.98, // Air resistance
		opacity: particle.opacity - 0.015 * dt
	};
}

function isFireworkDead(firework: Firework): boolean {
	return firework.exploded && firework.particles.length === 0;
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderBirthdaySystem(
	state: BirthdaySystemState,
	render: RenderState,
	canvas: CanvasContext
): void {
	const { ctx } = canvas;

	// Render fireworks first (behind confetti)
	state.fireworks.forEach((firework) => {
		renderFirework(firework, ctx);
	});

	// Render confetti
	state.confetti.forEach((confetti) => {
		renderConfetti(confetti, ctx);
	});
}

function renderConfetti(confetti: Confetti, ctx: CanvasRenderingContext2D): void {
	ctx.save();
	ctx.translate(confetti.x, confetti.y);
	ctx.rotate((confetti.rotation * Math.PI) / 180);
	ctx.fillStyle = confetti.color;

	switch (confetti.shape) {
		case 'rect':
			ctx.fillRect(-confetti.size / 2, -confetti.size / 4, confetti.size, confetti.size / 2);
			break;

		case 'circle':
			ctx.beginPath();
			ctx.arc(0, 0, confetti.size / 3, 0, Math.PI * 2);
			ctx.fill();
			break;

		case 'serpentine':
			// Curvy ribbon
			ctx.lineWidth = confetti.size / 4;
			ctx.strokeStyle = confetti.color;
			ctx.lineCap = 'round';
			ctx.beginPath();
			ctx.moveTo(-confetti.size, 0);
			ctx.quadraticCurveTo(-confetti.size / 2, -confetti.size / 2, 0, 0);
			ctx.quadraticCurveTo(confetti.size / 2, confetti.size / 2, confetti.size, 0);
			ctx.stroke();
			break;
	}

	ctx.restore();
}

function renderFirework(firework: Firework, ctx: CanvasRenderingContext2D): void {
	if (!firework.exploded) {
		// Draw rising trail
		const gradient = ctx.createLinearGradient(firework.x, firework.y, firework.x, firework.y + 30);
		gradient.addColorStop(0, firework.color);
		gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');

		ctx.strokeStyle = gradient;
		ctx.lineWidth = 3;
		ctx.lineCap = 'round';

		ctx.beginPath();
		ctx.moveTo(firework.x, firework.y);
		ctx.lineTo(firework.x, firework.y + 30);
		ctx.stroke();

		// Bright head
		ctx.fillStyle = '#ffffff';
		ctx.beginPath();
		ctx.arc(firework.x, firework.y, 3, 0, Math.PI * 2);
		ctx.fill();
	} else {
		// Draw explosion particles
		firework.particles.forEach((particle) => {
			ctx.save();
			ctx.globalAlpha = clamp(particle.opacity, 0, 1);

			// Particle with glow
			const gradient = ctx.createRadialGradient(
				particle.x,
				particle.y,
				0,
				particle.x,
				particle.y,
				8
			);
			gradient.addColorStop(0, '#ffffff');
			gradient.addColorStop(0.3, particle.color);
			gradient.addColorStop(1, `${particle.color}00`);

			ctx.fillStyle = gradient;
			ctx.beginPath();
			ctx.arc(particle.x, particle.y, 8, 0, Math.PI * 2);
			ctx.fill();

			ctx.restore();
		});
	}
}

// -----------------------------------------------------------------------------
// Birthday Background Gradient (animated)
// -----------------------------------------------------------------------------

export function getBirthdayGradientCSS(timestamp: number): string {
	const colors = BIRTHDAY_CONFIG.BG_GRADIENT_COLORS;
	const cycleTime = 10000; // 10 seconds per full cycle
	const progress = (timestamp % cycleTime) / cycleTime;

	// Rotate through colors
	const offset = Math.floor(progress * colors.length);
	const color1 = colors[offset % colors.length];
	const color2 = colors[(offset + 1) % colors.length];
	const color3 = colors[(offset + 2) % colors.length];
	const color4 = colors[(offset + 3) % colors.length];

	return `linear-gradient(135deg, ${color1} 0%, ${color2} 33%, ${color3} 66%, ${color4} 100%)`;
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const birthdaySystem = {
	init: initBirthdaySystem,
	update: updateBirthdaySystem,
	render: renderBirthdaySystem,
	getGradientCSS: getBirthdayGradientCSS
};
