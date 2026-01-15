/**
 * Utility to highlight search matches in a string.
 * It wraps matching parts of the string in a bold tag with a specific color.
 */
export function highlightMatches(text: string, query: string): string {
	if (!query || !query.trim()) return text;
	if (!text) return '';

	try {
		const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
		const regex = new RegExp(`(${escapedQuery})`, 'gi');

		// Replace with a span or bold tag.
		// We use a specific class for consistent styling.
		return text.replace(
			regex,
			'<span class="text-white font-black bg-blue-500/30 rounded-sm px-0.5">$1</span>'
		);
	} catch (e) {
		console.error('Error highlighting matches:', e);
		return text;
	}
}
