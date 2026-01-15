// =============================================================================
// SKY SYSTEM - Dynamic gradient sky based on time of day
// =============================================================================

import type { CanvasContext, RenderState, SkyState, TimeOfDay } from '../types';
import { TIME, SKY_PALETTES, getTimeOfDay, interpolateSkyPalette } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initSkySystem(): SkyState {
	return {
		gradient: {
			stops: [
				{ offset: 0, color: SKY_PALETTES.day[0] },
				{ offset: 0.5, color: SKY_PALETTES.day[1] },
				{ offset: 1, color: SKY_PALETTES.day[2] }
			]
		},
		timeOfDay: 'day'
	};
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateSkySystem(state: SkyState, render: RenderState): SkyState {
	const hour = render.time;
	// const timeOfDay = getTimeOfDay(hour);

	// Get the current and next palette with transition progress
	const { palette, timeOfDay: tod } = calculateSkyPalette(hour);

	return {
		gradient: {
			stops: [
				{ offset: 0, color: palette[0] },
				{ offset: 0.5, color: palette[1] },
				{ offset: 1, color: palette[2] }
			]
		},
		timeOfDay: tod
	};
}

// -----------------------------------------------------------------------------
// Render (returns CSS gradient string for use by parent)
// -----------------------------------------------------------------------------

export function getSkyGradientCSS(state: SkyState): string {
	const { stops } = state.gradient;
	const colorStops = stops.map((s) => `${s.color} ${s.offset * 100}%`).join(', ');
	return `linear-gradient(to bottom, ${colorStops})`;
}

// Also provide canvas rendering if needed
export function renderSkySystem(state: SkyState, render: RenderState, canvas: CanvasContext): void {
	const { ctx, width, height } = canvas;

	// Create gradient
	const gradient = ctx.createLinearGradient(0, 0, 0, height);
	state.gradient.stops.forEach((stop) => {
		gradient.addColorStop(stop.offset, stop.color);
	});

	// Fill background
	ctx.fillStyle = gradient;
	ctx.fillRect(0, 0, width, height);
}

// -----------------------------------------------------------------------------
// Helper: Calculate blended palette based on exact time
// -----------------------------------------------------------------------------

function calculateSkyPalette(hour: number): {
	palette: [string, string, string];
	timeOfDay: TimeOfDay;
} {
	// Night (18:30 - 5:30)
	if (hour >= TIME.DUSK_END || hour < TIME.DAWN_START) {
		return { palette: SKY_PALETTES.night, timeOfDay: 'night' };
	}

	// Dawn transition (5:30 - 6:30)
	if (hour >= TIME.DAWN_START && hour < TIME.DAWN_END) {
		// First half: night -> dawn
		if (hour < TIME.SUNRISE) {
			const progress = (hour - TIME.DAWN_START) / (TIME.SUNRISE - TIME.DAWN_START);
			return {
				palette: interpolateSkyPalette(SKY_PALETTES.night, SKY_PALETTES.dawn, progress),
				timeOfDay: 'dawn'
			};
		}
		// Second half: dawn -> morning
		const progress = (hour - TIME.SUNRISE) / (TIME.DAWN_END - TIME.SUNRISE);
		return {
			palette: interpolateSkyPalette(SKY_PALETTES.dawn, SKY_PALETTES.morning, progress),
			timeOfDay: 'morning'
		};
	}

	// Morning (6:30 - 9:00)
	if (hour >= TIME.DAWN_END && hour < 9) {
		const progress = (hour - TIME.DAWN_END) / (9 - TIME.DAWN_END);
		return {
			palette: interpolateSkyPalette(SKY_PALETTES.morning, SKY_PALETTES.day, progress),
			timeOfDay: 'morning'
		};
	}

	// Full day (9:00 - 17:30)
	if (hour >= 9 && hour < TIME.DUSK_START) {
		return { palette: SKY_PALETTES.day, timeOfDay: 'day' };
	}

	// Dusk transition (17:30 - 18:30)
	if (hour >= TIME.DUSK_START && hour < TIME.DUSK_END) {
		// First half: day -> dusk
		if (hour < TIME.SUNSET) {
			const progress = (hour - TIME.DUSK_START) / (TIME.SUNSET - TIME.DUSK_START);
			return {
				palette: interpolateSkyPalette(SKY_PALETTES.day, SKY_PALETTES.dusk, progress),
				timeOfDay: 'dusk'
			};
		}
		// Second half: dusk -> evening -> night
		const progress = (hour - TIME.SUNSET) / (TIME.DUSK_END - TIME.SUNSET);
		if (progress < 0.5) {
			return {
				palette: interpolateSkyPalette(SKY_PALETTES.dusk, SKY_PALETTES.evening, progress * 2),
				timeOfDay: 'evening'
			};
		}
		return {
			palette: interpolateSkyPalette(
				SKY_PALETTES.evening,
				SKY_PALETTES.night,
				(progress - 0.5) * 2
			),
			timeOfDay: 'evening'
		};
	}

	// Fallback
	return { palette: SKY_PALETTES.day, timeOfDay: 'day' };
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const skySystem = {
	init: initSkySystem,
	update: updateSkySystem,
	render: renderSkySystem,
	getCSS: getSkyGradientCSS
};
