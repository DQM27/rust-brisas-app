<!-- src/lib/components/layout/KeyboardShortcuts.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { tinykeys } from "tinykeys";
  import { emitCommand } from "$lib/stores/keyboardCommands";

  let unsubscribe: (() => void) | null = null;

  onMount(() => {
    // Registrar atajos globales
    unsubscribe = tinykeys(window, {
      // Ctrl+N: Crear nuevo (según módulo activo)
      "$mod+n": (event) => {
        event.preventDefault();

        // No ejecutar si está enfocado en un input de texto
        const target = event.target as HTMLElement;
        const isInput =
          target.tagName === "INPUT" ||
          target.tagName === "TEXTAREA" ||
          target.isContentEditable;

        // Solo prevenir en ciertos tipos de input
        if (isInput && target.tagName !== "INPUT") {
          return;
        }

        emitCommand("create-new");
      },

      // Ctrl+F: Enfocar búsqueda (futuro)
      // "$mod+f": (event) => {
      //   event.preventDefault();
      //   emitCommand("search");
      // },

      // Ctrl+R / F5: Refrescar datos (futuro)
      // "$mod+r": (event) => {
      //   event.preventDefault();
      //   emitCommand("refresh");
      // },
    });
  });

  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
  });
</script>
