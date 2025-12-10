import { writable, get } from "svelte/store";
// @ts-ignore
import { tinykeys } from "tinykeys";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "svelte-5-french-toast";
import { currentUser } from "./auth";

// Tipos
export type ShortcutAction =
    | "ingreso.save"
    | "ingreso.cancel"
    | "ingreso.new"
    | "general.save"
    | "general.cancel"
    | "search.focus";

export interface ShortcutDef {
    id: ShortcutAction;
    label: string;
    defaultKey: string;
    category: "Ingreso" | "General" | "Búsqueda";
}

// Definición de atajos por defecto
export const SHORTCUT_DEFS: Record<ShortcutAction, ShortcutDef> = {
    "ingreso.save": {
        id: "ingreso.save",
        label: "Guardar Ingreso",
        defaultKey: "Control+s",
        category: "Ingreso",
    },
    "ingreso.cancel": {
        id: "ingreso.cancel",
        label: "Cancelar / Cerrar",
        defaultKey: "Escape",
        category: "Ingreso",
    },
    "ingreso.new": {
        id: "ingreso.new",
        label: "Nuevo Registro",
        defaultKey: "Control+n",
        category: "Ingreso",
    },
    "general.save": {
        id: "general.save",
        label: "Guardar (General)",
        defaultKey: "Control+s",
        category: "General",
    },
    "general.cancel": {
        id: "general.cancel",
        label: "Cancelar (General)",
        defaultKey: "Escape",
        category: "General",
    },
    "search.focus": {
        id: "search.focus",
        label: "Focar Buscador",
        defaultKey: "Control+f",
        category: "Búsqueda",
    },
};

// Store principal: Mapa de ActionID -> KeyCombo
const createShortcutStore = () => {
    const { subscribe, set, update } = writable<Record<string, string>>({});

    // Cargar defaults inicialmente
    const defaults: Record<string, string> = {};
    Object.values(SHORTCUT_DEFS).forEach((def) => {
        defaults[def.id] = def.defaultKey;
    });
    set(defaults);

    // Función para cargar overrides del usuario
    const loadUserShortcuts = async () => {
        const user = get(currentUser);
        if (!user) return;

        try {
            const prefs = await invoke<{ key: string; value: string }[]>(
                "get_user_preferences",
                {
                    userId: user.id,
                    category: "shortcuts",
                },
            );

            update((current) => {
                const next = { ...current };
                prefs.forEach((p) => {
                    if (SHORTCUT_DEFS[p.key as ShortcutAction]) {
                        next[p.key] = p.value;
                    }
                });
                return next;
            });
        } catch (err) {
            console.error("Error loading shortcuts:", err);
        }
    };

    // Función para guardar un nuevo atajo
    const updateShortcut = async (actionId: ShortcutAction, newKey: string) => {
        const user = get(currentUser);
        if (!user) {
            toast.error("Debes iniciar sesión para personalizar atajos");
            return;
        }

        try {
            await invoke("set_user_preference", {
                userId: user.id,
                category: "shortcuts",
                key: actionId,
                value: newKey,
            });

            // Actualizar localmente
            update((s) => ({ ...s, [actionId]: newKey }));
            toast.success("Atajo actualizado");
        } catch (err) {
            console.error("Error saving shortcut:", err);
            toast.error("Error al guardar atajo");
        }
    };

    // Action de Svelte para usar en elementos o window
    // Uso: <svelte:window use:shortcut={{ trigger: 'ingreso.save', handler: () => ... }} />
    const useShortcut = (
        node: HTMLElement | Window,
        params: { trigger: ShortcutAction; handler: (e: KeyboardEvent) => void },
    ) => {
        let unsubscribeTinykeys: () => void;

        const setup = () => {
            const currentShortcuts = get(shortcutStore);
            const keyCombo = currentShortcuts[params.trigger];

            if (keyCombo) {
                unsubscribeTinykeys = tinykeys(node as HTMLElement, {
                    [keyCombo]: (event: KeyboardEvent) => {
                        event.preventDefault();
                        params.handler(event);
                    },
                });
            }
        };

        setup();

        // Reaccionar a cambios en el store (si el usuario cambia el atajo en caliente)
        const unsubscribeStore = subscribe(() => {
            if (unsubscribeTinykeys) unsubscribeTinykeys();
            setup();
        });

        return {
            update(newParams: {
                trigger: ShortcutAction;
                handler: (e: KeyboardEvent) => void;
            }) {
                params = newParams;
                if (unsubscribeTinykeys) unsubscribeTinykeys();
                setup();
            },
            destroy() {
                if (unsubscribeTinykeys) unsubscribeTinykeys();
                unsubscribeStore();
            },
        };
    };

    return {
        subscribe,
        loadUserShortcuts,
        updateShortcut,
        useShortcut,
        resetToDefaults: async () => {
            // TODO: Implementar borrado de BD si se desea
            // Por ahora solo resetea local
            set(defaults);
        },
    };
};

export const shortcutStore = createShortcutStore();

// Función de inicialización para evitar dependencias circulares
export function initShortcutSystem() {
    currentUser.subscribe((u) => {
        if (u) {
            shortcutStore.loadUserShortcuts();
        }
    });
}

