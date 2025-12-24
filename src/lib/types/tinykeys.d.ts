// Type declarations for tinykeys
declare module 'tinykeys' {
    type KeyBindingMap = Record<string, (event: KeyboardEvent) => void>;

    export function tinykeys(
        target: Window | HTMLElement,
        keyBindingMap: KeyBindingMap,
        options?: { event?: 'keydown' | 'keyup' }
    ): () => void;
}
