import { writable, get } from 'svelte/store';
import { persisted } from 'svelte-persisted-store';
import { openTab, resetTabs, tabsStore } from './tabs';

/**
 * Store de autenticación persistido
 */
export const isAuthenticated = persisted<boolean>('brisas-auth', false);

/**
 * Usuario actual (opcional, si necesitas guardarlo)
 */
export const currentUser = persisted<any>('brisas-user', null);

/**
 * Login: actualiza estado y abre tab de bienvenida
 */
export function login(user?: any): void {
  isAuthenticated.set(true);
  
  if (user) {
    currentUser.set(user);
  }
  
  // Abrir tab de bienvenida solo si no hay tabs abiertos
  const tabs = get(tabsStore);
  if (tabs.length === 0) {
    openTab({
      componentKey: 'welcome',
      title: 'Bienvenida',
      id: 'welcome'
    });
  }
}

/**
 * Logout: limpia estado y cierra todos los tabs
 */
export function logout(): void {
  isAuthenticated.set(false);
  currentUser.set(null);
  resetTabs();
}

/**
 * Verifica si hay sesión activa al iniciar la app
 */
export function checkSession(): boolean {
  const authenticated = get(isAuthenticated);
  
  // Si está autenticado pero no hay tabs, abrir welcome
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