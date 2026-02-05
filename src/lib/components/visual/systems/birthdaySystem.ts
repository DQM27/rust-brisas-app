// ============================================================================= 
// BIRTHDAY SYSTEM - Confetti, Balloons, and Fireworks!
// =============================================================================

import type {
	CanvasContext,
	RenderState,
	Confetti,
	Firework,
	FireworkParticle,
	BirthdaySystemState,
	Balloon
} from '../types';
import { BIRTHDAY_CONFIG, randomRange, randomInt, clamp } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initBirthdaySystem(canvas: CanvasContext): BirthdaySystemState {
	const confetti = createConfettiBurst(canvas);

	return {
		confetti,
		balloons: [],
		fireworks: [],
		nextFireworkTimer: 1000,
		nextBalloonTimer: 500
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

	// --- CONFETTI ---
	let confetti = state.confetti.map((c) => updateConfetti(c, dt, width, height));
	// Respawn logic
	confetti = confetti.map((c) => {
		if (c.y > height + 20) return createConfetti(width, height, false);
		return c;
	});

	// --- BALLOONS ---
	let balloons = state.balloons.map((b) => updateBalloon(b, dt));
	balloons = balloons.filter((b) => b.y > -100); // Remove balloons that flew away

	// Spawn new balloons
	let nextBalloonTimer = state.nextBalloonTimer - render.deltaTime;
	if (state.balloons.length < BIRTHDAY_CONFIG.BALLOON_COUNT_LIMIT && nextBalloonTimer <= 0) {
		balloons.push(createBalloon(width, height));
		nextBalloonTimer = randomRange(BIRTHDAY_CONFIG.BALLOON_INTERVAL[0], BIRTHDAY_CONFIG.BALLOON_INTERVAL[1]);
	}

	// --- FIREWORKS ---
	const fireworks = state.fireworks
		.map((f) => updateFirework(f, dt))
		.filter((f) => !isFireworkDead(f));

	let nextFireworkTimer = state.nextFireworkTimer - render.deltaTime;
	if (nextFireworkTimer <= 0) {
		fireworks.push(createFirework(width, height));
		nextFireworkTimer = randomRange(BIRTHDAY_CONFIG.FIREWORK_INTERVAL[0], BIRTHDAY_CONFIG.FIREWORK_INTERVAL[1]);
	}

	return { confetti, balloons, fireworks, nextFireworkTimer, nextBalloonTimer };
}

// -----------------------------------------------------------------------------
// Logic: Confetti
// -----------------------------------------------------------------------------

function createConfettiBurst(canvas: CanvasContext): Confetti[] {
	const confetti: Confetti[] = [];
	for (let i = 0; i < BIRTHDAY_CONFIG.CONFETTI_COUNT; i++) {
		confetti.push(createConfetti(canvas.width, canvas.height, true));
	}
	return confetti;
}

function createConfetti(w: number, h: number, burst: boolean): Confetti {
	const shapes: Confetti['shape'][] = ['rect', 'circle', 'serpentine'];
	return {
		x: burst ? w / 2 + randomRange(-100, 100) : randomRange(0, w),
		y: burst ? h * 0.3 : -20,
		vx: burst ? randomRange(-8, 8) : randomRange(-2, 2),
		vy: burst ? randomRange(-15, -5) : randomRange(2, 5),
		rotation: randomRange(0, 360),
		rotationSpeed: randomRange(-10, 10),
		color: BIRTHDAY_CONFIG.CONFETTI_COLORS[randomInt(0, BIRTHDAY_CONFIG.CONFETTI_COLORS.length - 1)],
		shape: shapes[randomInt(0, shapes.length - 1)],
		size: randomRange(BIRTHDAY_CONFIG.CONFETTI_SIZE_RANGE[0], BIRTHDAY_CONFIG.CONFETTI_SIZE_RANGE[1]),
		wobble: randomRange(0, Math.PI * 2),
		wobbleSpeed: randomRange(0.05, 0.15)
	};
}

function updateConfetti(c: Confetti, dt: number, _w: number, _h: number): Confetti {
	c.vy += 0.1 * dt; // Gravity
	c.vy = Math.min(c.vy, 6);
	c.vx *= 0.99;
	c.wobble += c.wobbleSpeed * dt;
	c.x += (c.vx + Math.cos(c.wobble)) * dt;
	c.y += c.vy * dt;
	c.rotation += c.rotationSpeed * dt;
	return c;
}

// -----------------------------------------------------------------------------
// Logic: Balloons
// -----------------------------------------------------------------------------

function createBalloon(w: number, h: number): Balloon {
	return {
		x: randomRange(w * 0.1, w * 0.9),
		y: h + 100, // Starts below screen
		vx: 0,
		vy: -randomRange(BIRTHDAY_CONFIG.BALLOON_SPEED_RANGE[0], BIRTHDAY_CONFIG.BALLOON_SPEED_RANGE[1]),
		color: BIRTHDAY_CONFIG.BALLOON_COLORS[randomInt(0, BIRTHDAY_CONFIG.BALLOON_COLORS.length - 1)],
		stringLength: randomRange(30, 60),
		wobble: randomRange(0, Math.PI * 2),
		wobbleSpeed: randomRange(0.02, 0.05),
		seed: Math.random()
	};
}

function updateBalloon(b: Balloon, dt: number): Balloon {
	b.wobble += b.wobbleSpeed * dt;
	b.x += Math.sin(b.wobble) * 0.5 * dt; // Gentle sway
	b.y += b.vy * dt; // Float up
	return b;
}

// -----------------------------------------------------------------------------
// Logic: Fireworks
// -----------------------------------------------------------------------------

// Interactive: Spawn where clicked
export function spawnInteractionFirework(state: BirthdaySystemState, x: number, y: number): BirthdaySystemState {
	const firework = createFirework(1000, 1000); // Dummy size, coords overwritten
	firework.x = x;

	// Start slightly below click to simulate rapid rise or just explode instantly? 
	// Let's make it explode instantly for snappy feedback
	firework.y = y;
	firework.targetY = y;
	firework.exploded = true;
	firework.particles = createExplosionParticles(firework.x, firework.y, firework.color);

	return {
		...state,
		fireworks: [...state.fireworks, firework]
	};
}

function createFirework(w: number, h: number): Firework {
	const targetY = randomRange(h * 0.1, h * 0.5);
	return {
		x: randomRange(w * 0.2, w * 0.8),
		y: h,
		vy: randomRange(-14, -10),
		targetY,
		color: BIRTHDAY_CONFIG.FIREWORK_COLORS[randomInt(0, BIRTHDAY_CONFIG.FIREWORK_COLORS.length - 1)],
		exploded: false,
		particles: [],
		trail: []
	};
}

function updateFirework(f: Firework, dt: number): Firework {
	if (!f.exploded) {
		f.y += f.vy * dt;
		f.vy += 0.1 * dt; // Gravity drags it down slightly as it rises

		// Add trail point
		f.trail.push({ x: f.x, y: f.y, opacity: 1.0 });
		// Fade trails
		f.trail.forEach(t => t.opacity -= 0.05 * dt);
		f.trail = f.trail.filter(t => t.opacity > 0);

		if (f.vy >= 0 || f.y <= f.targetY) {
			f.exploded = true;
			f.particles = createExplosionParticles(f.x, f.y, f.color);
		}
		return f;
	} else {
		// Update particles
		f.particles.forEach(p => {
			p.x += p.vx * dt;
			p.y += p.vy * dt;
			p.vy += 0.05 * dt; // Gravity
			p.vx *= 0.96; // Air resistance
			p.opacity -= p.decay * dt;
		});
		f.particles = f.particles.filter(p => p.opacity > 0);
		return f;
	}
}

function createExplosionParticles(x: number, y: number, color: string): FireworkParticle[] {
	const particles: FireworkParticle[] = [];
	const count = BIRTHDAY_CONFIG.FIREWORK_PARTICLE_COUNT;
	for (let i = 0; i < count; i++) {
		const angle = (Math.PI * 2 * i) / count;
		const speed = randomRange(2, 6);
		particles.push({
			x, y,
			vx: Math.cos(angle) * speed,
			vy: Math.sin(angle) * speed,
			opacity: 1,
			color,
			decay: randomRange(0.01, 0.03)
		});
	}
	return particles;
}

function isFireworkDead(f: Firework): boolean {
	return f.exploded && f.particles.length === 0;
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

	// 1. Fireworks (Background)
	state.fireworks.forEach(f => renderFirework(f, ctx));

	// 2. Balloons (Midground)
	state.balloons.forEach(b => renderBalloon(b, ctx));

	// 3. Confetti (Foreground)
	state.confetti.forEach(c => renderConfetti(c, ctx));
}

function renderBalloon(b: Balloon, ctx: CanvasRenderingContext2D) {
	ctx.save();
	ctx.translate(b.x, b.y);

	// String
	ctx.beginPath();
	ctx.moveTo(0, 30);
	ctx.quadraticCurveTo(
		Math.sin(b.wobble * 2) * 5,
		30 + b.stringLength / 2,
		Math.sin(b.wobble) * 10,
		30 + b.stringLength
	);
	ctx.strokeStyle = 'rgba(255, 255, 255, 0.6)';
	ctx.lineWidth = 1;
	ctx.stroke();

	// Balloon body
	ctx.fillStyle = b.color;
	ctx.beginPath();
	// Egg shape
	ctx.ellipse(0, 0, 20, 26, Math.sin(b.wobble * 0.5) * 0.1, 0, Math.PI * 2);
	ctx.fill();

	// Shine
	ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
	ctx.beginPath();
	ctx.ellipse(-8, -8, 4, 8, -0.5, 0, Math.PI * 2);
	ctx.fill();

	// Knot
	ctx.fillStyle = b.color;
	ctx.beginPath();
	ctx.moveTo(-4, 25);
	ctx.lineTo(4, 25);
	ctx.lineTo(0, 30);
	ctx.fill();

	ctx.restore();
}

function renderFirework(f: Firework, ctx: CanvasRenderingContext2D) {
	// Trail
	if (!f.exploded) {
		f.trail.forEach(t => {
			ctx.beginPath();
			ctx.arc(t.x, t.y, 2, 0, Math.PI * 2);
			ctx.fillStyle = `rgba(255, 255, 255, ${t.opacity})`;
			ctx.fill();
		});
		// Head
		ctx.beginPath();
		ctx.arc(f.x, f.y, 3, 0, Math.PI * 2);
		ctx.fillStyle = '#FFF';
		ctx.fill();
	}
	// Particles
	else {
		f.particles.forEach(p => {
			ctx.globalAlpha = p.opacity;
			ctx.fillStyle = p.color;
			ctx.beginPath();
			ctx.arc(p.x, p.y, 2.5, 0, Math.PI * 2);
			ctx.fill();

			// Glow
			ctx.shadowBlur = 10;
			ctx.shadowColor = p.color;
			ctx.globalAlpha = 1;
			ctx.shadowBlur = 0;
		});
	}
}

function renderConfetti(c: Confetti, ctx: CanvasRenderingContext2D) {
	ctx.save();
	ctx.translate(c.x, c.y);
	ctx.rotate((c.rotation * Math.PI) / 180);
	ctx.fillStyle = c.color;

	if (c.shape === 'rect') ctx.fillRect(-c.size / 2, -c.size / 4, c.size, c.size / 2);
	else if (c.shape === 'circle') {
		ctx.beginPath(); ctx.arc(0, 0, c.size / 3, 0, Math.PI * 2); ctx.fill();
	}
	else if (c.shape === 'serpentine') {
		ctx.lineWidth = c.size / 4; ctx.strokeStyle = c.color; ctx.lineCap = 'round';
		ctx.beginPath();
		ctx.moveTo(-c.size, 0);
		ctx.quadraticCurveTo(0, -c.size / 2, c.size, 0);
		ctx.stroke();
	}
	ctx.restore();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const birthdaySystem = {
	init: initBirthdaySystem,
	update: updateBirthdaySystem,
	render: renderBirthdaySystem,
	spawnInteractionFirework, // Export interaction
	getGradientCSS: getBirthdayGradientCSS
};

export function getBirthdayGradientCSS(t: number): string {
	const colors = BIRTHDAY_CONFIG.BG_GRADIENT_COLORS;
	const cycle = 15000;
	const progress = (t % cycle) / cycle;
	const i = Math.floor(progress * colors.length);
	const c1 = colors[i];
	const c2 = colors[(i + 1) % colors.length];
	return `linear-gradient(135deg, ${c1}, ${c2})`;
}
