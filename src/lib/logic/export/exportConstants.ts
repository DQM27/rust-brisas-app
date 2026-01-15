export const MARGIN_UNITS = [
	{ id: 'mm', label: 'mm' },
	{ id: 'cm', label: 'cm' },
	{ id: 'in', label: 'in' },
	{ id: 'pt', label: 'pt' }
] as const;

export type MarginUnit = (typeof MARGIN_UNITS)[number]['id'];

export const BANNER_COLORS = [
	{ id: '#059669', label: 'Verde' },
	{ id: '#2563eb', label: 'Azul' },
	{ id: '#7c3aed', label: 'Violeta' },
	{ id: '#dc2626', label: 'Rojo' },
	{ id: '#ea580c', label: 'Naranja' },
	{ id: '#0891b2', label: 'Cyan' },
	{ id: '#374151', label: 'Gris' },
	{ id: '#000000', label: 'Negro' }
];

export const FONT_VARIANTS = {
	Inter: ['Inter', 'Inter Light', 'Inter Medium', 'Inter SemiBold', 'Inter Bold'],
	Arial: ['Arial', 'Arial Bold'],
	'Segoe UI': ['Segoe UI', 'Segoe UI Light', 'Segoe UI Semibold'],
	Calibri: ['Calibri', 'Calibri Light', 'Calibri Bold'],
	'Times New Roman': ['Times New Roman', 'Times New Roman Bold']
};

export const PAPER_SIZES = [
	{ id: 'us-letter', label: 'Carta (US Letter)' },
	{ id: 'a4', label: 'A4' },
	{ id: 'legal', label: 'Legal' }
];
