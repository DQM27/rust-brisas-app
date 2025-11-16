import { persisted } from 'svelte-persisted-store';
import { openTab, resetTabs, tabsStore } from './tabs';
import { get } from 'svelte/store';
import { goto } from '$app/navigation';
import type { User } from '$lib/types/user';

export const isAuthenticated = persisted<boolean>('brisas-auth', false);
export const currentUser = persisted<User | null>('brisas-user', null);

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
  goto('/login');
}

export function checkSession(): boolean {
  const authenticated = get(isAuthenticated);
  if (authenticated) {
    const tabs = get(tabsStore);
    if (tabs.length === 0) {
      openTab({
        componentKey: 'welcome',
        title: 'Bienvenida',
        id: 'welcome'
      });
    }
  }
  return authenticated;
}