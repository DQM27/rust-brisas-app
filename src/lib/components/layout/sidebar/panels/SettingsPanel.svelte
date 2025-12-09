<!-- src/lib/components/layout/sidebar/panels/SettingsPanel.svelte -->
<script lang="ts">
  import {
    Settings,
    Bell,
    Download,
    RefreshCw,
    Info,
    Database,
    FileSpreadsheet,
  } from "lucide-svelte";
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
  <div class="panel-section-title">CONFIGURACIÓN GENERAL</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("device-settings", "Ajustes Generales"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("device-settings", "Ajustes Generales"),
        ),
      )}
  >
    <svelte:component this={Settings} size={16} />
    <span>Ajustes Generales</span>
  </button>

  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("general-settings", "Ajustes Gráficos"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("general-settings", "Ajustes Gráficos"),
        ),
      )}
  >
    <!-- Usamos FileSpreadsheet temporalmente como icono de visual/paleta si no hay otro, o mantenemos Settings pero cambiamos texto -->
    <!-- Mejor: Usar el mismo Settings para ambos por ahora o buscar uno de paleta si estuviera impoortado -->
    <svelte:component this={Settings} size={16} />
    <span>Ajustes Gráficos</span>
  </button>

  <div class="panel-item non-clickable">
    <svelte:component this={Bell} size={16} />
    <span>Notificaciones</span>
  </div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("backup-settings", "Copias de Seguridad"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("backup-settings", "Copias de Seguridad"),
        ),
      )}
  >
    <svelte:component this={Download} size={16} />
    <span>Backup y restore</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">DATOS</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("blacklist-import", "Importar Lista Negra"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("blacklist-import", "Importar Lista Negra"),
        ),
      )}
  >
    <svelte:component this={FileSpreadsheet} size={16} />
    <span>Importar Lista Negra desde Excel</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">SISTEMA</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("update-settings", "Actualizaciones"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("update-settings", "Actualizaciones")),
      )}
  >
    <svelte:component this={RefreshCw} size={16} />
    <span>Actualizaciones</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() => openView("welcome", "Acerca del Sistema"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("welcome", "Acerca del Sistema")),
      )}
  >
    <svelte:component this={Info} size={16} />
    <span>Acerca del sistema</span>
  </button>
</div>
