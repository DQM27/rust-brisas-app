// src/lib/stores/themeStore.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark';

// Función para obtener tema inicial
function getInitialTheme(): Theme {
    if (browser) {
        const stored = localStorage.getItem('theme') as Theme | null;
        if (stored) return stored;

        // Default a dark mode para la app Brisas
        return 'dark';
    }
    return 'dark';
}

// Crear store writable
const theme = writable<Theme>(getInitialTheme());

// Aplicar tema al documento
function applyTheme(newTheme: Theme): void {
    if (browser) {
        const root = document.documentElement;

        if (newTheme === 'dark') {
            root.classList.add('dark');
        } else {
            root.classList.remove('dark');
        }

        localStorage.setItem('theme', newTheme);
    }
}

// Inicializar tema en el DOM
if (browser) {
    const initialTheme = getInitialTheme();
    applyTheme(initialTheme);
}

// Suscribirse a cambios para aplicarlos al DOM
if (browser) {
    theme.subscribe((value) => {
        applyTheme(value);
    });
}

// Función para toggle del tema
export function toggleTheme(): void {
    theme.update(current => current === 'dark' ? 'light' : 'dark');
}

// Exportar el store
export const themeStore = theme;
