export async function getAvailableFormats(): Promise<string[]> {
	// This could be fetched from backend config or static list
	return ['excel', 'csv', 'pdf', 'json'];
}
