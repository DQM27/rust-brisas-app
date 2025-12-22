// $lib/stores/auth.ts
import { persisted } from 'svelte-persisted-store';
import { resetTabs } from './tabs';
import type { UserResponse } from '$lib/types/user';

export const isAuthenticated = persisted<boolean>('brisas-auth', false);
export const currentUser = persisted<UserResponse | null>('brisas-user', null);

export function login(user: UserResponse): void {
  isAuthenticated.set(true);
  currentUser.set(user);

}


export function logout(): void {
  isAuthenticated.set(false);
  currentUser.set(null);
  resetTabs();
}

/**
 * Recarga la sesión actual desde el backend
 * Útil cuando se actualiza el perfil propio
 */
export async function reloadSession(user: UserResponse): Promise<void> {
  currentUser.set(user);
}