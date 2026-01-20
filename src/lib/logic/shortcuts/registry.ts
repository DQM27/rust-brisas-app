import { tinykeys } from 'tinykeys';
import { get, writable } from 'svelte/store';
import type { ShortcutDefinition, ShortcutScope } from './types';
import { activeContext, emitCommand, type KeyboardCommand } from '$lib/stores/keyboardCommands';
import { showSpotlight } from '$lib/stores/ui';
import { logout } from '$lib/stores/auth';
import { toggleTheme } from '$lib/stores/themeStore';

// Store para saber si hay un modal abierto (podemos conectarlo a un store de UI global si existe, o usar DOM checking)
// Por ahora usaremos la verificación del DOM que ya tenías, pero encapsulada aquí.

/**
 * Registro central de atajos
 */
class ShortcutRegistry {
    private shortcuts: ShortcutDefinition[] = [];
    private unsubscribeFn: (() => void) | null = null;

    // Store para exponer los atajos activos a la UI (Cheat Sheet)
    public activeShortcuts = writable<ShortcutDefinition[]>([]);

    constructor() { }

    /**
     * Cargar definiciones iniciales
     */
    public loadDefinitions(defs: ShortcutDefinition[]) {
        this.shortcuts = defs;
        this.updateActiveShortcuts();
    }

    /**
     * Inicializar listeners
     */
    public init() {
        if (this.unsubscribeFn) this.unsubscribeFn();

        // Construir mapa para tinykeys
        const keyMap: Record<string, (e: KeyboardEvent) => void> = {};

        // Agrupar por tecla para manejar colisiones (mismo atajo, diferente contexto)
        const byKey: Record<string, ShortcutDefinition[]> = {};

        this.shortcuts.forEach(def => {
            if (!byKey[def.keys]) byKey[def.keys] = [];
            byKey[def.keys].push(def);
        });

        Object.keys(byKey).forEach(key => {
            keyMap[key] = (event: KeyboardEvent) => {
                this.handleKeyPress(event, byKey[key]);
            };
        });

        this.unsubscribeFn = tinykeys(window, keyMap);
    }

    public destroy() {
        if (this.unsubscribeFn) this.unsubscribeFn();
    }

    private handleKeyPress(event: KeyboardEvent, candidates: ShortcutDefinition[]) {
        const target = event.target as HTMLElement;
        const isInput = this.isTextInput(target);
        const modalOpen = this.isModalOpen();

        // Filtrar candidatos válidos para el contexto actual
        const valid = candidates.filter(def => {
            // 1. Verificar si debe funcionar en inputs
            // Por defecto, NO funcionan en inputs salvo que sea específico
            // Si queremos permitir algunos en inputs, necesitaríamos una flag en la definición.
            // Asumiremos que 'Escape' siempre funciona.
            if (isInput && def.keys !== 'Escape') return false;

            // 2. Verificar scope
            if (def.scope === 'global') return true;
            if (def.scope === 'modal') return modalOpen;
            if (def.scope === 'list' && !modalOpen) return true; // Asumimos 'list' como vista normal sin modal

            // TODO: Refinar lógica de checkeo de contexto específico usando activeContext
            // Si el scope es 'list', deberíamos verificar si hay un contexto activo
            if (def.scope === 'list') {
                return !!get(activeContext);
            }

            return false;
        });

        // Ejecutar el primero que coincida (prioridad: Modal > List > Global implícita por orden de definición si quisiéramos, pero aquí filtramos)
        // Si hay múltiples válidos, ejecutamos el que tenga el scope más específico?
        // Simple: Ejecutamos el primero válido.
        if (valid.length > 0) {
            // Opcional: preventDefault si se ejecutó algo
            // event.preventDefault(); // Dejamos que la acción decida o lo hacemos aquí? 
            // Mejor que la acción decida o hacerlo aquí si es un comando conocido.

            valid[0].action(event);
        }
    }

    /**
     * Utilería para detectar inputs
     */
    private isTextInput(target: HTMLElement): boolean {
        if (target.isContentEditable) return true;
        if (target.tagName === 'TEXTAREA') return true;
        if (target.tagName === 'INPUT') {
            const inputType = (target as HTMLInputElement).type.toLowerCase();
            const nonTextTypes = ['checkbox', 'radio', 'button', 'submit', 'reset', 'file', 'image'];
            return !nonTextTypes.includes(inputType);
        }
        return false;
    }

    private isModalOpen(): boolean {
        return document.querySelector('[role="dialog"], .modal-overlay, [data-modal="true"]') !== null;
    }

    private updateActiveShortcuts() {
        this.activeShortcuts.set(this.shortcuts);
    }

    // Método para obtener atajos agrupados para la UI de ayuda
    public getGroupedShortcuts() {
        // ... implementación futura
        return this.shortcuts;
    }
}

export const shortcutRegistry = new ShortcutRegistry();
