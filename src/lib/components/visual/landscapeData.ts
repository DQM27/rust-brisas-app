export type LandscapeType = 'mountains' | 'forest' | 'city' | 'desert' | 'beach' | 'moon';

export const LANDSCAPE_TYPES: { id: LandscapeType; name: string; icon: string }[] = [
    { id: 'mountains', name: 'Montañas', icon: 'Mountain' },
    { id: 'forest', name: 'Bosque', icon: 'Trees' },
    { id: 'city', name: 'Ciudad', icon: 'Building2' },
    { id: 'desert', name: 'Desierto', icon: 'Sun' }, // Use Sun as placeholder if Cactus not available
    { id: 'beach', name: 'Playa', icon: 'Umbrella' },
    { id: 'moon', name: 'Luna', icon: 'Moon' },
];

// Reusing Season type from constants/types for consistency
import type { Season } from "$lib/stores/settingsStore";

export interface LandscapeTheme {
    colors: [string, string, string]; // [Back, Mid, Front]
}

// Default Mountain Themes (Conserved)
export const MOUNTAIN_THEMES: Record<Season, LandscapeTheme> = {
    winter: { colors: ['#e2e8f0', '#cbd5e1', '#94a3b8'] }, // Slate 200, 300, 400
    spring: { colors: ['#dcfce7', '#86efac', '#4ade80'] }, // Green 100, 300, 400
    summer: { colors: ['#fae8ff', '#f0abfc', '#e879f9'] }, // Fuchsia 100, 300, 400
    autumn: { colors: ['#ffedd5', '#fdba74', '#fb923c'] }, // Orange 100, 300, 400
    rain: { colors: ['#94a3b8', '#64748b', '#475569'] }, // Rainy gray/blue
};

// Forest Themes
export const FOREST_THEMES: Record<Season, LandscapeTheme> = {
    winter: { colors: ['#cbd5e1', '#64748b', '#334155'] }, // Snowy/Dark pine
    spring: { colors: ['#bef264', '#84cc16', '#4d7c0f'] }, // Lime greens
    summer: { colors: ['#4ade80', '#16a34a', '#14532d'] }, // Deep greens
    autumn: { colors: ['#fcd34d', '#d97706', '#92400e'] }, // Amber/Brown
    rain: { colors: ['#334155', '#1e293b', '#0f172a'] }, // Dark/Wet Forest
};

// City Themes (More neutral, changing with light mostly)
export const CITY_THEMES: Record<Season, LandscapeTheme> = {
    winter: { colors: ['#94a3b8', '#64748b', '#475569'] },
    spring: { colors: ['#a5b4fc', '#818cf8', '#6366f1'] }, // Indigo tint
    summer: { colors: ['#c4b5fd', '#a78bfa', '#8b5cf6'] }, // Violet tint
    autumn: { colors: ['#fdba74', '#fb923c', '#f97316'] }, // Orange sunset vibe
    rain: { colors: ['#64748b', '#475569', '#334155'] }, // Rainy City
};

// Desert Themes (Consistent mostly)
export const DESERT_THEMES: Record<Season, LandscapeTheme> = {
    winter: { colors: ['#e7e5e4', '#d6d3d1', '#a8a29e'] }, // Cold desert
    spring: { colors: ['#fde68a', '#fcd34d', '#fbbf24'] },
    summer: { colors: ['#fef3c7', '#fde68a', '#d97706'] }, // Hot gold
    autumn: { colors: ['#fed7aa', '#fdba74', '#ea580c'] },
    rain: { colors: ['#a8a29e', '#78716c', '#57534e'] }, // Wet Sand
};

// Beach Themes
export const BEACH_THEMES: Record<Season, LandscapeTheme> = {
    // For beach, we might use colors for Water(back), Sand(mid), Palm/Prop(front)
    // Or Water Back, Water Mid, Sand Front. Let's do: Sea Back, Sea Mid, Sand Front
    winter: { colors: ['#0ea5e9', '#0284c7', '#e5e5e5'] }, // Cold blue
    spring: { colors: ['#38bdf8', '#0ea5e9', '#fde047'] },
    summer: { colors: ['#06b6d4', '#0891b2', '#fcd34d'] }, // Cyan/Gold
    autumn: { colors: ['#67e8f9', '#22d3ee', '#fdba74'] },
    rain: { colors: ['#0c4a6e', '#075985', '#78716c'] }, // Stormy Sea
};

// Moon Themes (Monochrome mostly)
export const MOON_THEMES: Record<Season, LandscapeTheme> = {
    winter: { colors: ['#334155', '#1e293b', '#0f172a'] },
    spring: { colors: ['#475569', '#334155', '#1e293b'] },
    summer: { colors: ['#64748b', '#475569', '#334155'] },
    autumn: { colors: ['#475569', '#334155', '#1e293b'] },
    rain: { colors: ['#334155', '#1e293b', '#0f172a'] },
};


export const LANDSCAPE_PATHS: Record<LandscapeType, [string, string, string]> = {
    mountains: [
        "M0,224L48,224C96,224,192,224,288,208C384,192,480,160,576,170.7C672,181,768,235,864,240C960,245,1056,203,1152,186.7C1248,171,1344,181,1392,186.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z",
        "M0,96L48,112C96,128,192,160,288,186.7C384,213,480,235,576,213.3C672,192,768,128,864,128C960,128,1056,192,1152,213.3C1248,235,1344,213,1392,202.7L1440,192L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z",
        "M0,192L48,197.3C96,203,192,213,288,229.3C384,245,480,267,576,250.7C672,235,768,181,864,160C960,139,1056,149,1152,149.3C1248,149,1344,139,1392,133.3L1440,128L1440,320L1392,320C1344,320,1248,320,1152,320C1056,320,960,320,864,320C768,320,672,320,576,320C480,320,384,320,288,320C192,320,96,320,48,320L0,320Z"
    ],
    forest: [
        // Rolling hills back
        "M0,160 C300,100 600,200 900,140 C1200,80 1440,160 1440,160 V320 H0 Z",
        // Spiky tree-line mid
        "M0,220 L20,200 L40,220 L60,190 L80,220 L100,200 L120,220 L140,180 L160,220 L180,200 L200,220 L220,190 L240,220 L260,200 L280,220 L300,170 L320,220 L340,200 L360,220 L380,190 L400,220 L420,200 L440,220 L460,180 L480,220 L500,200 L520,220 L540,190 L560,220 L580,200 L600,220 L620,170 L640,220 L660,200 L680,220 L700,190 L720,220 L740,200 L760,220 L780,180 L800,220 L820,200 L840,220 L860,190 L880,220 L900,200 L920,220 L940,170 L960,220 L980,200 L1000,220 L1020,190 L1040,220 L1060,200 L1080,220 L1100,180 L1120,220 L1140,200 L1160,220 L1180,190 L1200,220 L1220,200 L1240,220 L1260,170 L1280,220 L1300,200 L1320,220 L1340,190 L1360,220 L1380,200 L1400,220 L1420,180 L1440,220 V320 H0 Z",
        // Detailed foreground hills with some tree bumps
        "M0,280 C100,260 200,290 300,270 C400,250 500,280 600,260 C700,240 800,270 900,250 C1000,230 1100,260 1200,240 C1300,220 1440,260 1440,260 V320 H0 Z"
    ],
    city: [
        // Far generic skyline
        "M0,250 L50,250 L50,200 L100,200 L100,230 L150,230 L150,180 L200,180 L200,240 L250,240 L250,190 L300,190 L300,250 L350,250 L350,210 L400,210 L400,250 L450,250 L450,170 L500,170 L500,250 L550,250 L550,200 L600,200 L600,250 L650,250 L650,180 L700,180 L700,250 L750,250 L750,220 L800,220 L800,250 L850,250 L850,190 L900,190 L900,250 L950,250 L950,210 L1000,210 L1000,250 L1050,250 L1050,160 L1100,160 L1100,250 L1150,250 L1150,230 L1200,230 L1200,250 L1250,250 L1250,180 L1300,180 L1300,250 L1350,250 L1350,200 L1400,200 L1400,250 L1440,250 V320 H0 Z",
        // Mid detailed buildings
        "M0,320 L0,260 L40,260 L40,220 L80,220 L80,260 L120,260 L120,200 L160,200 L160,260 L200,260 L200,230 L240,230 L240,260 L280,260 L280,180 L320,180 L320,260 L360,260 L360,210 L400,210 L400,260 L440,260 L440,240 L480,240 L480,260 L520,260 L520,190 L560,190 L560,260 L600,260 L600,220 L640,220 L640,260 L680,260 L680,200 L720,200 L720,260 L760,260 L760,230 L800,230 L800,260 L840,260 L840,170 L880,170 L880,260 L920,260 L920,240 L960,240 L960,260 L1000,260 L1000,200 L1040,200 L1040,260 L1080,260 L1080,220 L1120,220 L1120,260 L1160,260 L1160,180 L1200,180 L1200,260 L1240,260 L1240,210 L1280,210 L1280,260 L1320,260 L1320,230 L1360,230 L1360,260 L1400,260 L1400,200 L1440,200 L1440,320 H0 Z",
        // Front structures/park
        "M0,320 L0,300 L1440,300 L1440,320 H0 Z" // Just a ground plane for now
    ],
    desert: [
        // Far dunes
        "M0,200 C360,160 540,240 720,200 C900,160 1260,220 1440,180 V320 H0 Z",
        // Mid dunes
        "M0,260 C200,220 500,300 700,260 C900,220 1200,280 1440,250 V320 H0 Z",
        // Close dunes
        "M0,320 L0,290 C400,260 800,330 1440,270 V320 H0 Z"
    ],
    beach: [
        // Far Sea (Horizon)
        "M0,220 L1440,220 L1440,320 L0,320 Z",
        // Mid Sea (Waves)
        "M0,250 C100,245 200,255 300,250 C400,245 500,255 600,250 C700,245 800,255 900,250 C1000,245 1100,255 1200,250 C1300,245 1400,255 1440,250 V320 H0 Z",
        // Sand Beach (Front)
        "M0,320 L0,280 C360,290 720,270 1440,290 V320 H0 Z"
    ],
    moon: [
        // Far craters
        "M0,200 C150,195 300,205 450,200 C500,160 600,160 650,200 C800,205 950,195 1100,200 C1150,150 1250,150 1300,200 C1400,205 1440,200 V320 H0 Z",
        // Mid craters
        "M0,250 C200,245 350,220 400,250 C450,280 550,280 600,250 C800,240 1000,260 1200,250 C1250,210 1350,210 1400,250 L1440,255 V320 H0 Z",
        // Front jagged rocks
        "M0,320 L0,290 L50,270 L100,295 L150,275 L200,290 L300,280 L400,295 L600,285 L800,295 L1000,285 L1200,292 L1440,285 V320 H0 Z"
    ]
};

export function getLandscapeTheme(type: LandscapeType, season: Season): LandscapeTheme {
    let themeSet: Record<Season, LandscapeTheme>;
    switch (type) {
        case 'forest': themeSet = FOREST_THEMES; break;
        case 'city': themeSet = CITY_THEMES; break;
        case 'desert': themeSet = DESERT_THEMES; break;
        case 'beach': themeSet = BEACH_THEMES; break;
        case 'moon': themeSet = MOON_THEMES; break;
        // Case mountains falls through to default
        case 'mountains':
        default: themeSet = MOUNTAIN_THEMES; break;
    }
    // Fallback to winter if specific season is missing/undefined
    return themeSet[season] || themeSet['winter'];
}
