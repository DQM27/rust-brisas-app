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
    FileText,
    Keyboard,
    Home,
    Shield,
    Palette,
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
      openView("general-settings", "Ajustes Generales"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("general-settings", "Ajustes Generales"),
        ),
      )}
  >
    <svelte:component this={Settings} size={16} />
    <span>Ajustes Generales</span>
  </button>

  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("visual-settings", "Ajustes Gráficos"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("visual-settings", "Ajustes Gráficos")),
      )}
  >
    <svelte:component this={Palette} size={16} />
    <span>Ajustes Gráficos</span>
  </button>

  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("shortcut-settings", "Atajos de Teclado"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("shortcut-settings", "Atajos de Teclado"),
        ),
      )}
  >
    <svelte:component this={Keyboard} size={16} />
    <span>Atajos de Teclado</span>
  </button>

  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("export-settings", "Configuración de Exportación"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("export-settings", "Configuración de Exportación"),
        ),
      )}
  >
    <svelte:component this={FileText} size={16} />
    <span>Exportación</span>
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
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("roles-settings", "Roles y Permisos"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("roles-settings", "Roles y Permisos")),
      )}
  >
    <svelte:component this={Shield} size={16} />
    <span>Roles y Permisos</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">SISTEMA</div>
  <button
    class="panel-item"
    on:click={executeAndClose(() => openView("welcome", "Bienvenida"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("welcome", "Bienvenida")),
      )}
  >
    <svelte:component this={Home} size={16} />
    <span>Pantalla de Bienvenida</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("security-settings", "Seguridad y Credenciales"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() =>
          openView("security-settings", "Seguridad y Credenciales"),
        ),
      )}
  >
    <svelte:component this={Shield} size={16} />
    <span>Seguridad y Credenciales</span>
  </button>
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
    on:click={executeAndClose(() => openView("about", "Acerca del Sistema"))}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("about", "Acerca del Sistema")),
      )}
  >
    <svelte:component this={Info} size={16} />
    <span>Acerca del sistema</span>
  </button>
</div>
