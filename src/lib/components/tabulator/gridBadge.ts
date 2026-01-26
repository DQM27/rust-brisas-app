/**
 * Utility to generate uniform badge HTML for Tabulator grids.
 * Centralizes styling for status, levels, and access indicators.
 */

export type BadgeColor = 'green' | 'red' | 'amber' | 'blue' | 'gray';

interface BadgeOptions {
	text: string;
	color?: BadgeColor;
	isButton?: boolean;
	className?: string;
	withDot?: boolean;
	withCheck?: boolean;
}

/**
 * Generates a standard badge HTML string with premium dark theme styles.
 * Use this in Tabulator formatters to ensure UI consistency.
 */
export const createGridBadge = ({
	text,
	color = 'gray',
	isButton = false,
	className = '',
	withDot = false,
	withCheck = false
}: BadgeOptions): string => {
	const colors: Record<BadgeColor, string> = {
		green:
			'bg-emerald-500/15 text-emerald-700 dark:text-emerald-400 border-emerald-600/30 dark:border-emerald-500/20 hover:bg-emerald-500/25',
		red: 'bg-rose-500/15 text-rose-700 dark:text-rose-400 border-rose-600/30 dark:border-rose-500/20 hover:bg-rose-500/25',
		amber:
			'bg-amber-500/15 text-amber-700 dark:text-amber-400 border-amber-600/30 dark:border-amber-500/20 hover:bg-amber-500/25',
		blue: 'bg-blue-500/15 text-blue-700 dark:text-blue-400 border-blue-600/30 dark:border-blue-500/20 hover:bg-blue-500/25',
		gray: 'bg-gray-500/15 text-gray-700 dark:text-gray-400 border-gray-600/30 dark:border-gray-500/20 hover:bg-gray-500/25'
	};

	const baseClass =
		'inline-flex items-center justify-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none transition-colors shadow-sm';
	const tag = isButton ? 'button' : 'span';
	const buttonClass = isButton ? 'cursor-pointer active:scale-95' : '';

	let content = text;
	if (withDot) {
		content = `<span class="mr-1.5 opacity-70">●</span>${text}`;
	} else if (withCheck) {
		content = `<span class="mr-1.5 opacity-70">✓</span>${text}`;
	}

	return `<${tag} class="${baseClass} ${colors[color]} ${buttonClass} ${className}">${content}</${tag}>`;
};
