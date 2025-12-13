import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";
import type { ShortcutConfig } from "$lib/types/shortcuts";

type CommandHandler = () => void;

class ShortcutService {
    private config: ShortcutConfig | null = null;
    private contextStack = writable<string[]>([]);
    private handlers = new Map<string, Map<string, CommandHandler>>();
    private initialized = false;

    constructor() {
        if (typeof window !== "undefined") {
            this.init();
        }
    }

    async init() {
        if (this.initialized) return;

        try {
            this.config = await invoke<ShortcutConfig>("get_shortcuts");
            console.log("⌨️ Shortcuts loaded:", this.config);

            window.addEventListener("keydown", this.handleKeydown.bind(this));
            this.initialized = true;
        } catch (error) {
            console.error("❌ Failed to load shortcuts:", error);
        }
    }

    /**
     * Maneja el evento keydown global
     */
    private handleKeydown(event: KeyboardEvent) {
        if (!this.config) return;

        // Ignorar si el foco está en un input/textarea (salvo que sea un comando con modificadores)
        const target = event.target as HTMLElement;
        const isInput = target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable;

        // Generar string del evento (ej. "Ctrl+S")
        const keyString = this.normalizeKey(event);
        if (!keyString) return;

        // Si es input y no tiene modificadores (solo letras/enter), dejar pasar
        if (isInput && !event.ctrlKey && !event.altKey && !event.metaKey && event.key !== 'Escape') {
            return;
        }

        // 1. Resolver comando basado en contexto
        const command = this.resolveCommand(keyString);

        if (command) {
            // 2. Ejecutar handler si existe
            if (this.dispatchCommand(command)) {
                event.preventDefault();
                event.stopPropagation();
            }
        }
    }

    /**
     * Resuelve qué comando corresponde a la tecla presionada según los contextos activos
     */
    private resolveCommand(keyString: string): string | null {
        if (!this.config) return null;

        const stack = get(this.contextStack);

        // 1. Buscar en contextos activos (del más reciente al más antiguo)
        for (let i = stack.length - 1; i >= 0; i--) {
            const contextId = stack[i];
            const contextMap = this.config.contexts[contextId];

            if (contextMap) {
                // Buscar si alguna acción en este contexto tiene asignada esta tecla
                for (const [cmd, key] of Object.entries(contextMap)) {
                    if (key.toLowerCase() === keyString.toLowerCase()) {
                        // Verificar si el comando tiene handler registrado en este contexto
                        // Opcional: Si está definido en config pero no tiene handler, ¿bloquea?
                        // Asumimos que sí, para evitar que 'bublee' a un contexto inferior incorrectamente.
                        return cmd;
                    }
                }
            }
        }

        // 2. Buscar en global
        for (const [cmd, key] of Object.entries(this.config.global)) {
            if (key.toLowerCase() === keyString.toLowerCase()) {
                return cmd;
            }
        }

        return null;
    }

    /**
     * Ejecuta el handler asociado al comando
     */
    private dispatchCommand(command: string): boolean {
        const stack = get(this.contextStack);

        // Buscar handler en la pila de contextos
        for (let i = stack.length - 1; i >= 0; i--) {
            const contextId = stack[i];
            const contextHandlers = this.handlers.get(contextId);

            if (contextHandlers && contextHandlers.has(command)) {
                contextHandlers.get(command)?.();
                return true;
            }
        }

        // Buscar handler global (contexto 'root' implícito o handlers globales)
        const globalHandlers = this.handlers.get("root");
        if (globalHandlers && globalHandlers.has(command)) {
            globalHandlers.get(command)?.();
            return true;
        }

        return false;
    }

    /**
     * Normaliza un evento de teclado a String (ej. "Ctrl+Shift+S")
     */
    private normalizeKey(e: KeyboardEvent): string {
        const parts = [];
        if (e.ctrlKey) parts.push("Ctrl");
        if (e.shiftKey) parts.push("Shift");
        if (e.altKey) parts.push("Alt");
        if (e.metaKey) parts.push("Meta");

        if (!e.key) return "";
        let key = e.key;
        if (key === " ") key = "Space";
        if (key === "Control" || key === "Shift" || key === "Alt" || key === "Meta") return ""; // Solo modificador presionado

        // Capitalizar primera letra
        if (key.length === 1) key = key.toUpperCase();

        parts.push(key);
        return parts.join("+");
    }

    // ==========================================
    // API PÚBLICA
    // ==========================================

    /**
     * Registra un scope de contexto. Usar con use:action en Svelte.
     */
    useScope = (node: HTMLElement, contextId: string) => {
        this.pushContext(contextId);

        return {
            destroy: () => {
                this.popContext(contextId);
            }
        };
    }

    pushContext(contextId: string) {
        this.contextStack.update(s => [...s, contextId]);
        console.log("📥 Context Push:", contextId, get(this.contextStack));
    }

    popContext(contextId: string) {
        this.contextStack.update(s => s.filter(c => c !== contextId)); // Filtro robusto por si acaso
        console.log("out Context Pop:", contextId, get(this.contextStack));
    }

    /**
     * Registra un handler para un comando en un contexto específico
     */
    registerHandler(contextId: string, command: string, handler: CommandHandler) {
        if (!this.handlers.has(contextId)) {
            this.handlers.set(contextId, new Map());
        }
        this.handlers.get(contextId)!.set(command, handler);

        return () => {
            const ctxHandlers = this.handlers.get(contextId);
            if (ctxHandlers) {
                ctxHandlers.delete(command);
                if (ctxHandlers.size === 0) {
                    this.handlers.delete(contextId);
                }
            }
        };
    }
}

export const shortcutService = new ShortcutService();
