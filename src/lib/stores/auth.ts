// $lib/stores/auth.ts
import { writable } from 'svelte/store';
import { resetTabs } from './tabs';
import type { UserResponse } from '$lib/types/user';
import { startSession, stopSession } from './sessionStore';

// Session-only stores (NOT persisted - login required after app restart)
export const isAuthenticated = writable<boolean>(false);
export const currentUser = writable<UserResponse | null>(null);

export function login(user: UserResponse): void {
  isAuthenticated.set(true);
  currentUser.set(user);

  // Start session monitoring (activity tracking and timeout checking)
  startSession();
}

export async function logout(): Promise<void> {
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