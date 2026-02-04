// $lib/stores/auth.ts
import { writable, get } from 'svelte/store';
import { resetTabs } from './tabs';
import type { UserResponse } from '$lib/types/user';
import { startSession, stopSession, getCurrentSessionDuration } from './sessionStore';
import { sessionSettings } from './sessionSettingsStore';
import { auditService } from '$lib/logic/audit/auditService';

// Session-only stores (NOT persisted - login required after app restart)
export const isAuthenticated = writable<boolean>(false);
export const currentUser = writable<UserResponse | null>(null);

export function login(user: UserResponse, detail: string = 'User Login'): void {
	isAuthenticated.set(true);
	currentUser.set(user);

	// Start session monitoring (activity tracking and timeout checking)
	startSession();

	// Audit Logging
	const settings = get(sessionSettings);
	if (settings.enableSessionAudit) {
		auditService.log('LOGIN', user.nombreCompleto, detail);
	}
}

export async function logout(detail: string = 'User Logout'): Promise<void> {
	// Audit Logout
	const user = get(currentUser);
	const settings = get(sessionSettings);
	if (user && settings.enableSessionAudit) {
		const duration = getCurrentSessionDuration();
		await auditService.log('LOGOUT', user.nombreCompleto, detail, duration);
	}

	// Stop session monitoring first
	stopSession();

	// Clear authentication state
	isAuthenticated.set(false);
	currentUser.set(null);

	// Close all tabs
	resetTabs();
}

/**
 * Recarga la sesión actual desde el backend
 * Útil cuando se actualiza el perfil propio
 */
export async function reloadSession(user: UserResponse): Promise<void> {
	currentUser.set(user);
}
