// $lib/stores/auth.ts
import { persisted } from 'svelte-persisted-store';
import { resetTabs } from './tabs';
import type { User } from '$lib/types/user';

export const isAuthenticated = persisted<boolean>('brisas-auth', false);
export const currentUser = persisted<User | null>('brisas-user', null);

export function login(user: User): void {
  isAuthenticated.set(true);
  currentUser.set(user);
  // NO maneja tabs aquí
}

export function logout(): void {
  isAuthenticated.set(false);
  currentUser.set(null);
  resetTabs();
}