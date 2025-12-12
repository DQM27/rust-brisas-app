<!-- src/lib/components/layout/sidebar/panels/LogsPanel.svelte -->
<script lang="ts">
  import { Search, Activity, Download, Settings, MessageSquare } from "lucide-svelte";
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
  <div class="panel-section-title">REGISTROS DEL SISTEMA</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("dashboard", "Registros del Sistema"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("dashboard", "Registros del Sistema")),
      )}
  >
    <svelte:component this={Search} size={16} />
    <span>Buscar en registros</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("user-list", "Actividad de Usuarios"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("user-list", "Actividad de Usuarios")),
      )}
  >
    <svelte:component this={Activity} size={16} />
    <span>Actividad de usuarios</span>
  </button>
  <div class="panel-item non-clickable">
    <svelte:component this={Download} size={16} />
    <span>Exportar registros</span>
  </div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("reportes-list", "Historial de Reportes"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("reportes-list", "Historial de Reportes")),
      )}
  >
    <svelte:component this={MessageSquare} size={16} />
    <span>Historial de Reportes</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">CONFIGURACIÓN</div>
  <div class="panel-item non-clickable">
    <svelte:component this={Settings} size={16} />
    <span>Configuración de logs</span>
  </div>
</div>