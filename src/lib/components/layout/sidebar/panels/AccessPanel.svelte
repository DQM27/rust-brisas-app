<!-- src/lib/components/layout/sidebar/panels/AccessPanel.svelte -->
<script lang="ts">
  import {
    Lock,
    FileText,
    Ban,
    BadgeCheck,
    LogIn,
    ShieldAlert,
  } from "lucide-svelte";
  import { openView, activePanel } from "../../../../stores/sidebar";
  import { currentUser } from "$lib/stores/auth";
  import { can } from "$lib/logic/permissions";

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
  {#if $currentUser && can($currentUser, "VIEW_USER_LIST")}
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
  {/if}

  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("gafete-list", "Gestión de Gafetes"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("gafete-list", "Gestión de Gafetes")),
      )}
  >
    <svelte:component this={BadgeCheck} size={16} />
    <span>Gafetes</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(() =>
      openView("ingreso-list", "Control de Ingresos"),
    )}
    on:keydown={(e) =>
      handleKeydown(
        e,
        executeAndClose(() => openView("ingreso-list", "Control de Ingresos")),
      )}
  >
    <svelte:component this={LogIn} size={16} />
    <span>Ingresos</span>
  </button>
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
