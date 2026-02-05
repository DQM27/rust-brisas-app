// =============================================================================
// CELESTIAL SYSTEM - Simple Sun & Moon (estilo original)
// =============================================================================

import type {
	CanvasContext,
	RenderState,
	CelestialSystemState,
	SunState,
	MoonState
} from '../types';
import { TIME, CELESTIAL_CONFIG } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initCelestialSystem(): CelestialSystemState {
	return {
		sun: {
			x: 50,
			y: 120,
			opacity: 0,
			rotation: 0,
			scale: 1,
			rayRotation: 0,
			glowIntensity: 1
		},
		moon: {
			x: 50,
			y: 120,
			opacity: 0,
			rotation: 0,
			scale: 1,
			phase: 'full'
		},
		showBoth: false
	};
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateCelestialSystem(
	state: CelestialSystemState,
	render: RenderState
): CelestialSystemState {
	const hour = render.time;

	// Calculate sun position
	const sunrise = render.celestialSettings.sunrise ?? TIME.DAWN_START;
	const sunset = render.celestialSettings.sunset ?? TIME.DUSK_END;

	const sun = calculateSunPosition(hour, state.sun, render.timestamp, sunrise, sunset);

	// Calculate moon position
	const moon = calculateMoonPosition(hour, state.moon, sunset, sunrise); // Moon opposite to sun approx

	// Show both during transitions
	const showBoth =
		(hour >= sunrise && hour < sunrise + 1) ||
		(hour >= sunset - 1 && hour < sunset);

	return { sun, moon, showBoth };
}

// -----------------------------------------------------------------------------
// Render
// -----------------------------------------------------------------------------

export function renderCelestialSystem(
	state: CelestialSystemState,
	render: RenderState,
	canvas: CanvasContext
): void {
	const { ctx, width, height } = canvas;

	// Render moon first (behind sun during transitions)
	if (state.moon.opacity > 0) {
		renderMoon(state.moon, ctx, width, height, render.timestamp, render.celestialSettings);
	}

	// Render sun
	if (state.sun.opacity > 0) {
		renderSun(state.sun, ctx, width, height, render.timestamp, render.celestialSettings);
	}
}

// -----------------------------------------------------------------------------
// Sun Position & Rendering (estilo original - simple con glow parpadeante)
// -----------------------------------------------------------------------------

function calculateSunPosition(
	hour: number,
	_current: SunState,
	timestamp: number,
	sunrise: number,
	sunset: number
): SunState {
	let x = 50;
	let y = CELESTIAL_CONFIG.ARC_BOTTOM;
	let opacity = 0;

	// Sun visible from Sunrise to Sunset
	// We add a small buffer for dawn/dusk visibility
	const visibleStart = sunrise - 1;
	const visibleEnd = sunset + 1;

	if (hour >= visibleStart && hour < visibleEnd) {
		const duration = visibleEnd - visibleStart;
		const progress = (hour - visibleStart) / duration;

		// X: -10% to 110%
		x = -10 + progress * 120;

		// Y: Arco usando seno - de ARC_BOTTOM a ARC_TOP
		const arcHeight = Math.sin(progress * Math.PI);
		y =
			CELESTIAL_CONFIG.ARC_BOTTOM -
			arcHeight * (CELESTIAL_CONFIG.ARC_BOTTOM - CELESTIAL_CONFIG.ARC_TOP);

		// Opacity: fade in/out en los bordes
		if (progress < 0.1) {
			opacity = progress / 0.1;
		} else if (progress > 0.9) {
			opacity = (1 - progress) / 0.1;
		} else {
			opacity = 1;
		}
	}

	// Glow pulsante (como animate-pulse del original)
	const glowIntensity = 0.8 + Math.sin(timestamp / 1500) * 0.2;

	return {
		x,
		y,
		opacity,
		rotation: 0,
		scale: 1,
		rayRotation: 0,
		glowIntensity
	};
}

function renderSun(
	sun: SunState,
	ctx: CanvasRenderingContext2D,
	width: number,
	height: number,
	timestamp: number,
	settings?: { sunStyle: string }
): void {
	const x = (sun.x / 100) * width;
	const y = (sun.y / 100) * height;
	const size = CELESTIAL_CONFIG.SUN_SIZE;
	const colors = CELESTIAL_CONFIG.SUN_COLORS;

	ctx.save();
	ctx.globalAlpha = sun.opacity;

	// Glow exterior grande (parpadeante)
	const outerGlowSize = size * 2.5 * sun.glowIntensity;
	const outerGlow = ctx.createRadialGradient(x, y, size * 0.3, x, y, outerGlowSize);
	outerGlow.addColorStop(0, colors.glowOuter);
	outerGlow.addColorStop(1, 'rgba(255, 160, 60, 0)');
	ctx.fillStyle = outerGlow;
	ctx.beginPath();
	ctx.arc(x, y, outerGlowSize, 0, Math.PI * 2);
	ctx.fill();

	// Glow medio
	const midGlowSize = size * 1.5;
	const midGlow = ctx.createRadialGradient(x, y, size * 0.2, x, y, midGlowSize);
	midGlow.addColorStop(0, colors.glow);
	midGlow.addColorStop(1, 'rgba(255, 200, 100, 0)');
	ctx.fillStyle = midGlow;
	ctx.beginPath();
	ctx.arc(x, y, midGlowSize, 0, Math.PI * 2);
	ctx.fill();

	// Cuerpo del sol
	const bodyGradient = ctx.createRadialGradient(
		x - size * 0.15,
		y - size * 0.15,
		0,
		x,
		y,
		size * 0.5
	);
	bodyGradient.addColorStop(0, '#ffee88');
	bodyGradient.addColorStop(0.7, colors.core);
	bodyGradient.addColorStop(1, '#ffcc22');

	ctx.fillStyle = bodyGradient;
	ctx.beginPath();
	ctx.arc(x, y, size * 0.5, 0, Math.PI * 2);
	ctx.fill();

	// Cloudy Style Overlay
	if (settings?.sunStyle === 'cloudy') {
		ctx.fillStyle = 'rgba(255, 255, 255, 0.4)';

		// Cloud 1
		ctx.beginPath();
		ctx.arc(x - size * 0.4, y + size * 0.2, size * 0.3, 0, Math.PI * 2);
		ctx.arc(x, y + size * 0.3, size * 0.4, 0, Math.PI * 2);
		ctx.arc(x + size * 0.5, y + size * 0.2, size * 0.3, 0, Math.PI * 2);
		ctx.fill();

		// Cloud 2
		ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
		ctx.beginPath();
		ctx.arc(x - size * 0.2, y - size * 0.1, size * 0.35, 0, Math.PI * 2);
		ctx.arc(x + size * 0.3, y - size * 0.1, size * 0.3, 0, Math.PI * 2);
		ctx.fill();
	}

	ctx.restore();
}

// -----------------------------------------------------------------------------
// Moon Position & Rendering (estilo original - simple crescent con glow)
// -----------------------------------------------------------------------------

function calculateMoonPosition(
	hour: number,
	current: MoonState,
	visibleStart?: number,
	visibleEnd?: number
): MoonState {
	let x = 50;
	let y = CELESTIAL_CONFIG.ARC_BOTTOM;
	let opacity = 0;

	// Moon logic: Generally visible when sun is NOT.
	// We default to old constants if dynamic ones aren't passed (safety)
	const start = visibleStart ?? TIME.DUSK_START;
	const end = visibleEnd ?? TIME.DAWN_END;

	// Normalize times for crossing midnight
	// If start > end (e.g. 18:00 to 06:00), we handle wrap

	let isVisible = false;
	let progress = 0;

	const nightDuration = (end < start) ? (24 - start + end) : (end - start);

	if (end < start) {
		// Normal night wrapping midnight (e.g. 18 to 6)
		if (hour >= start) {
			isVisible = true;
			progress = (hour - start) / nightDuration;
		} else if (hour < end) {
			isVisible = true;
			progress = (hour + (24 - start)) / nightDuration;
		}
	} else {
		// Polar night or odd timing (start < end)
		if (hour >= start && hour < end) {
			isVisible = true;
			progress = (hour - start) / nightDuration;
		}
	}

	if (isVisible) {
		// X: -10% to 110%
		x = -10 + progress * 120;

		// Y: Arco
		const arcHeight = Math.sin(progress * Math.PI);
		y =
			CELESTIAL_CONFIG.ARC_BOTTOM -
			arcHeight * (CELESTIAL_CONFIG.ARC_BOTTOM - CELESTIAL_CONFIG.ARC_TOP);

		// Opacity
		if (progress < 0.1) {
			opacity = progress / 0.1;
		} else if (progress > 0.9) {
			opacity = (1 - progress) / 0.1;
		} else {
			opacity = 1;
		}
	}

	return {
		x,
		y,
		opacity,
		rotation: 0,
		scale: 1,
		phase: 'full' // No usamos fases complicadas
	};
}

function renderMoon(
	moon: MoonState,
	ctx: CanvasRenderingContext2D,
	width: number,
	height: number,
	timestamp: number,
	settings?: { moonPhase: string }
): void {
	const x = (moon.x / 100) * width;
	const y = (moon.y / 100) * height;
	const size = CELESTIAL_CONFIG.MOON_SIZE;
	const colors = CELESTIAL_CONFIG.MOON_COLORS;

	const phaseId = settings?.moonPhase ?? 'full';

	ctx.save();
	ctx.globalAlpha = moon.opacity;

	// Glow pulsante
	const glowPulse = 0.8 + Math.sin(timestamp / 2000) * 0.2;

	// Glow exterior grande
	const outerGlowSize = size * 2 * glowPulse;
	const outerGlow = ctx.createRadialGradient(x, y, size * 0.3, x, y, outerGlowSize);
	outerGlow.addColorStop(0, 'rgba(180, 200, 255, 0.3)');
	outerGlow.addColorStop(1, 'rgba(180, 200, 255, 0)');
	ctx.fillStyle = outerGlow;
	ctx.beginPath();
	ctx.arc(x, y, outerGlowSize, 0, Math.PI * 2);
	ctx.fill();

	// Glow medio
	const midGlowSize = size * 1.3;
	const midGlow = ctx.createRadialGradient(x, y, size * 0.2, x, y, midGlowSize);
	midGlow.addColorStop(0, colors.glow);
	midGlow.addColorStop(1, 'rgba(200, 210, 255, 0)');
	ctx.fillStyle = midGlow;
	ctx.beginPath();
	ctx.arc(x, y, midGlowSize, 0, Math.PI * 2);
	ctx.fill();

	const moonRadius = size * 0.45;

	// New Moon
	if (phaseId === 'new') {
		ctx.fillStyle = 'rgba(15, 20, 40, 0.9)';
		ctx.beginPath();
		ctx.arc(x, y, moonRadius, 0, Math.PI * 2);
		ctx.fill();
		ctx.restore();
		return;
	}

	// Full Moon
	if (phaseId === 'full') {
		ctx.fillStyle = colors.fill;
		ctx.beginPath();
		ctx.arc(x, y, moonRadius, 0, Math.PI * 2);
		ctx.fill();
		ctx.restore();
		return;
	}

	const isWaxing = ['waxing-crescent', 'first-quarter', 'waxing-gibbous'].includes(phaseId);
	const isGibbous = phaseId.includes('gibbous');
	const isCrescent = phaseId.includes('crescent');
	const isQuarter = phaseId.includes('quarter');

	// Dark Base
	ctx.fillStyle = 'rgba(15, 20, 40, 0.9)';
	ctx.beginPath();
	ctx.arc(x, y, moonRadius, 0, Math.PI * 2);
	ctx.fill();

	// Lit part
	ctx.fillStyle = colors.fill;
	ctx.beginPath();

	if (isWaxing) {
		ctx.arc(x, y, moonRadius, -Math.PI / 2, Math.PI / 2);
	} else {
		ctx.arc(x, y, moonRadius, Math.PI / 2, (3 * Math.PI) / 2);
	}

	if (isQuarter) {
		ctx.fill();
	} else if (isCrescent) {
		ctx.fill();
		const ellipseWidth = moonRadius * 0.6;
		ctx.fillStyle = 'rgba(15, 20, 40, 1)';
		ctx.beginPath();
		ctx.ellipse(x, y, ellipseWidth, moonRadius, 0, 0, Math.PI * 2);
		ctx.fill();
	} else if (isGibbous) {
		ctx.fill();
		const ellipseWidth = moonRadius * 0.6;
		ctx.fillStyle = colors.fill;
		ctx.beginPath();
		ctx.ellipse(x, y, ellipseWidth, moonRadius, 0, 0, Math.PI * 2);
		ctx.fill();
	}

	ctx.restore();
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const celestialSystem = {
	init: initCelestialSystem,
	update: updateCelestialSystem,
	render: renderCelestialSystem
};
