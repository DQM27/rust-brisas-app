import { persisted } from 'svelte-persisted-store';
import { openTab, resetTabs, tabsStore } from './tabs';
import type { User } from '$lib/types/user';
import { get } from 'svelte/store';

// ----------------------------
// Stores
// ----------------------------
export const isAuthenticated = persisted<boolean>('brisas-auth', false);
export const currentUser = persisted<User | null>('brisas-user', null);

// ----------------------------
// Funciones
// ----------------------------
export function login(user: User): void {
  isAuthenticated.set(true);
  currentUser.set(user);

  const tabs = get(tabsStore);
  if (tabs.length === 0) {
    openTab({
      componentKey: 'welcome',
      title: 'Bienvenida',
      id: 'welcome'
    });
  }
}

export function logout(): void {
  isAuthenticated.set(false);
  currentUser.set(null);
  resetTabs();
  // No hace falta redirect; la página raíz muestra login automáticamente
}

// ----------------------------
// Helper opcional
// ----------------------------
export function initializeAuth(): void {
  const auth = get(isAuthenticated);
  const tabs = get(tabsStore);
  if (auth && tabs.length === 0) {
    openTab({
      componentKey: 'welcome',
      title: 'Bienvenida',
      id: 'welcome'
    });
  }
}
