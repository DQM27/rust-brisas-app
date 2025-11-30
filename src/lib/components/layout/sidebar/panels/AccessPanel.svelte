<!-- src/lib/components/layout/sidebar/panels/AccessPanel.svelte -->
<script lang="ts">
  import { Lock, BarChart3, Shield, FileText, Ban } from "lucide-svelte";
  import { openView, activePanel } from "../../../../stores/sidebar";

  function handleKeydown(e: KeyboardEvent, action: () => void) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      action();
    }
  }

  function executeAndClose(action: () => void) {
    return () => {
      action();
      activePanel.set(null);
    };
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">CONTROLES DE ACCESO</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("user-list", "Gestión de Permisos"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("user-list", "Gestión de Permisos")),
      )}
  >
    <svelte:component this={Lock} size={16} />
    <span>Gestión de permisos</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() => openView("lista-negra", "Lista Negra"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("lista-negra", "Lista Negra")),
      )}
  >
    <svelte:component this={Ban} size={16} />
    <span>Lista negra</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() => openView("dashboard", "Panel de Control"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("dashboard", "Panel de Control")),
      )}
  >
    <svelte:component this={BarChart3} size={16} />
    <span>Panel de control</span>
  </button>
  <div class="panel-item non-clickable">
    <svelte:component this={Shield} size={16} />
    <span>Políticas de seguridad</span>
  </div>
</div>

<div class="panel-section">
  <div class="panel-section-title">HERRAMIENTAS</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() => openView("welcome", "Documentación"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("welcome", "Documentación")),
      )}
  >
    <svelte:component this={FileText} size={16} />
    <span>Documentación</span>
  </button>
</div>