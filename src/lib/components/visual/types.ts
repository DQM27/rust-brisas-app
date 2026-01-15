// =============================================================================
// BRISAS VISUAL SYSTEM v2 - Type Definitions
// =============================================================================

export type Season = 'winter' | 'spring' | 'summer' | 'autumn' | 'rain';

export type TimeOfDay = 'night' | 'dawn' | 'morning' | 'day' | 'dusk' | 'evening';

// -----------------------------------------------------------------------------
// Canvas & Rendering
// -----------------------------------------------------------------------------

export interface CanvasContext {
	ctx: CanvasRenderingContext2D;
	width: number;
	height: number;
	dpr: number; // Device Pixel Ratio
}

export interface RenderState {
	time: number; // 0-24 (hora decimal)
	season: Season;
	deltaTime: number; // ms desde último frame
	timestamp: number; // performance.now()
	wind: WindState;
	isBirthday: boolean;
	weatherSettings: {
		densityMultiplier: number;
		speedMultiplier: number;
		sizeMultiplier: number;
		windInfluence: number;
		turbulence: number;
	};
	celestialSettings: {
		moonPhase: string;
		sunStyle: string;
	};
	starSettings: {
		countMultiplier: number;
		twinkleSpeed: number;
		shootingStarFrequency: number;
		shootingStarSpeed: number;
		meteorShowerEnabled: boolean;
	};
	cloudSettings?: {
		style: 'cartoon' | 'soft';
		opacity: number;
		count: number;
		windSpeed: number;
		turbulence: number;
	};
}

// -----------------------------------------------------------------------------
// Sky System
// -----------------------------------------------------------------------------

export interface SkyGradient {
	stops: Array<{ offset: number; color: string }>;
}

export interface SkyState {
	gradient: SkyGradient;
	timeOfDay: TimeOfDay;
}

// -----------------------------------------------------------------------------
// Star System
// -----------------------------------------------------------------------------

export interface Star {
	x: number; // 0-1 normalized
	y: number; // 0-1 normalized
	size: number; // pixels
	brightness: number; // 0-1 base brightness
	twinkleOffset: number; // random phase offset for twinkling
	twinkleSpeed: number; // how fast it twinkles
}

export interface ShootingStar {
	x: number;
	y: number;
	angle: number; // direction in radians
	speed: number;
	length: number;
	opacity: number;
	active: boolean;
}

export interface StarSystemState {
	stars: Star[];
	shootingStars: ShootingStar[];
	visibility: number; // 0-1 fade for dawn/dusk
}

// -----------------------------------------------------------------------------
// Celestial System (Sun/Moon)
// -----------------------------------------------------------------------------

export interface CelestialBody {
	x: number; // percentage 0-100
	y: number; // percentage 0-100
	opacity: number; // 0-1
	rotation: number; // degrees (for sun rays)
	scale: number; // for squash/stretch
}

export interface SunState extends CelestialBody {
	rayRotation: number;
	glowIntensity: number;
}

export interface MoonState extends CelestialBody {
	phase: MoonPhase;
}

export type MoonPhase =
	| 'new'
	| 'waxing-crescent'
	| 'first-quarter'
	| 'waxing-gibbous'
	| 'full'
	| 'waning-gibbous'
	| 'last-quarter'
	| 'waning-crescent';

export interface CelestialSystemState {
	sun: SunState;
	moon: MoonState;
	showBoth: boolean; // during transitions
}

// -----------------------------------------------------------------------------
// Cloud System
// -----------------------------------------------------------------------------

export interface Cloud {
	x: number; // can go beyond 0-100 for entering/exiting
	y: number; // percentage
	baseY: number; // For turbulence calculation
	scale: number; // size multiplier
	speed: number; // horizontal movement speed
	opacity: number;
	layer: number; // 0=back, 1=mid, 2=front (parallax)
	seed: number; // for consistent blob shape
}

export interface CloudSystemState {
	clouds: Cloud[];
	colorTint: string; // changes based on time of day
}

// -----------------------------------------------------------------------------
// Weather Particle System
// -----------------------------------------------------------------------------

export interface Particle {
	x: number;
	y: number;
	vx: number; // velocity x
	vy: number; // velocity y
	size: number;
	rotation: number;
	rotationSpeed: number;
	opacity: number;
	color: string;
	type: ParticleType;
	// For fireflies
	glowPhase?: number;
	glowSpeed?: number;
}

export type ParticleType = 'snowflake' | 'petal' | 'leaf' | 'pollen' | 'firefly' | 'rain';

export interface ParticleConfig {
	count: number;
	colors: string[];
	sizeRange: [number, number];
	speedRange: [number, number];
	rotates: boolean;
	glows: boolean;
}

export interface ParticleSystemState {
	particles: Particle[];
	config: ParticleConfig;
}

// -----------------------------------------------------------------------------
// Wind System
// -----------------------------------------------------------------------------

export interface WindState {
	strength: number; // -3 to 3 (negative = left, positive = right)
	targetStrength: number; // what we're easing towards
	gustTimer: number; // countdown to next gust
	isGusting: boolean;
}

// -----------------------------------------------------------------------------
// Birthday System
// -----------------------------------------------------------------------------

export interface Confetti {
	x: number;
	y: number;
	vx: number;
	vy: number;
	rotation: number;
	rotationSpeed: number;
	color: string;
	shape: 'rect' | 'circle' | 'serpentine';
	size: number;
	wobble: number;
	wobbleSpeed: number;
}

export interface Firework {
	x: number;
	y: number;
	vy: number; // upward velocity
	targetY: number; // where it explodes
	color: string;
	exploded: boolean;
	particles: FireworkParticle[];
}

export interface FireworkParticle {
	x: number;
	y: number;
	vx: number;
	vy: number;
	opacity: number;
	color: string;
}

export interface BirthdaySystemState {
	confetti: Confetti[];
	fireworks: Firework[];
	nextFireworkTimer: number;
}

// -----------------------------------------------------------------------------
// Bokeh (Flent) System
// -----------------------------------------------------------------------------

export interface BokehParticle extends Particle {
	targetOpacity: number;
	pulseSpeed: number;
}

export interface BokehSystemState {
	particles: BokehParticle[];
}

// -----------------------------------------------------------------------------
// Complete Scene State
// -----------------------------------------------------------------------------

export interface SceneState {
	sky: SkyState;
	stars: StarSystemState;
	celestial: CelestialSystemState;
	clouds: CloudSystemState;
	particles: ParticleSystemState;
	wind: WindState;
	birthday: BirthdaySystemState | null;
}

// -----------------------------------------------------------------------------
// System Interface (all systems implement this)
// -----------------------------------------------------------------------------

export interface VisualSystem<T> {
	init(canvas: CanvasContext): T;
	update(state: T, render: RenderState, canvas: CanvasContext): T;
	render(state: T, render: RenderState, canvas: CanvasContext): void;
	reset?(state: T): T;
}
