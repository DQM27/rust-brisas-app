<!-- src/lib/components/layout/KeyboardShortcuts.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { tinykeys } from "tinykeys";
  import { emitCommand, activeContext } from "$lib/stores/keyboardCommands";
  import { get } from "svelte/store";

  let unsubscribe: (() => void) | null = null;

  /**
   * Helper para verificar si el target es un elemento de entrada de texto
   */
  function isTextInput(target: HTMLElement): boolean {
    if (target.isContentEditable) return true;
    if (target.tagName === "TEXTAREA") return true;
    if (target.tagName === "INPUT") {
      const inputType = (target as HTMLInputElement).type.toLowerCase();
      // Permitir atajos en checkboxes, radios, buttons
      const nonTextTypes = [
        "checkbox",
        "radio",
        "button",
        "submit",
        "reset",
        "file",
        "image",
      ];
      return !nonTextTypes.includes(inputType);
    }
    return false;
  }

  /**
   * Helper para verificar si hay un modal abierto
   */
  function isModalOpen(): boolean {
    // Buscar elementos con role="dialog" o clases comunes de modales
    return (
      document.querySelector(
        '[role="dialog"], .modal-overlay, [data-modal="true"]',
      ) !== null
    );
  }

  onMount(() => {
    // Registrar atajos globales
    unsubscribe = tinykeys(window, {
      // ============================================
      // CRUD SHORTCUTS
      // ============================================

      // Ctrl+N: Crear nuevo (según módulo activo)
      "$mod+n": (event) => {
        event.preventDefault();

        const target = event.target as HTMLElement;
        if (isTextInput(target)) return;

        // No ejecutar si hay modal abierto
        if (isModalOpen()) return;

        // Solo ejecutar si hay un contexto activo
        if (get(activeContext)) {
          emitCommand("create-new");
        }
      },

      // Ctrl+E: Editar seleccionado
      "$mod+e": (event) => {
        event.preventDefault();

        const target = event.target as HTMLElement;
        if (isTextInput(target)) return;
        if (isModalOpen()) return;

        if (get(activeContext)) {
          emitCommand("edit");
        }
      },

      // Delete: Eliminar seleccionado
      Delete: (event) => {
        const target = event.target as HTMLElement;
        if (isTextInput(target)) return;

        // Solo en listas, no en modales
        if (isModalOpen()) return;

        if (get(activeContext)) {
          event.preventDefault();
          emitCommand("delete");
        }
      },

      // ============================================
      // NAVIGATION & UI SHORTCUTS
      // ============================================

      // Escape: Cerrar modal o deseleccionar
      Escape: (event) => {
        // Escape siempre funciona, incluso en inputs
        emitCommand("escape");
      },

      // Ctrl+F: Enfocar búsqueda
      "$mod+f": (event) => {
        event.preventDefault();

        // No ejecutar dentro de modales
        if (isModalOpen()) return;

        emitCommand("search");
      },

      // Ctrl+R / F5: Refrescar datos
      "$mod+r": (event) => {
        event.preventDefault();

        if (get(activeContext)) {
          emitCommand("refresh");
        }
      },

      // ============================================
      // FORM SHORTCUTS
      // ============================================

      // Ctrl+S: Guardar (en formularios)
      "$mod+s": (event) => {
        event.preventDefault();
        emitCommand("save");
      },
    });
  });

  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
  });
</script>
