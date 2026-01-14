<!-- src/lib/components/layout/sidebar/SettingsMenu.svelte -->
<script lang="ts">
  import { Settings } from "lucide-svelte";
  import type { UserResponse } from "$lib/types/user";
  import { can } from "$lib/logic/permissions";
  import { ROLE_ADMIN_ID } from "$lib/types/role";
  import { openView } from "$lib/stores/sidebar";

  interface Props {
    show: boolean;
    currentUser: UserResponse | null;
    onOpenUpdate: () => void;
    onOpenAbout: () => void;
    onRequestReindex: () => void;
    onClose: () => void;
  }

  let {
    show = $bindable(),
    currentUser,
    onOpenUpdate,
    onOpenAbout,
    onRequestReindex,
    onClose,
  }: Props = $props();

  function toggleMenu(e: MouseEvent) {
    e.stopPropagation();
    show = !show;
  }

  function handleAction(action: () => void) {
    action();
    show = false;
  }
</script>

<div class="relative w-full flex items-center justify-center">
  <button
    onclick={toggleMenu}
    class="sidebar-icon-btn group {show ? 'active' : ''}"
    title="Configuración"
  >
    <Settings
      size={22}
      class="transition-transform duration-200 group-hover:scale-110"
    />
    <span class="sidebar-icon-tooltip">Configuración</span>
  </button>

  {#if show}
    <div
      class="settings-menu"
      onclick={(e) => e.stopPropagation()}
      role="menu"
      tabindex="-1"
      onkeydown={(e) => e.key === "Escape" && onClose()}
    >
      <!-- Grupo 1: Configuración Principal -->
      {#if currentUser && can(currentUser, "VIEW_SETTINGS_GENERAL")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() =>
              openView("general-settings", "Ajustes Generales"),
            )}
        >
          Ajustes Generales
        </button>
      {/if}
      {#if currentUser && can(currentUser, "VIEW_SETTINGS_VISUAL")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() => openView("visual-settings", "Ajustes Gráficos"))}
        >
          Ajustes Gráficos
        </button>
      {/if}

      {#if currentUser && can(currentUser, "VIEW_SETTINGS_SESSIONS")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() =>
              openView("session-settings", "Gestión de Sesión"),
            )}
        >
          Gestión de Sesión
        </button>
      {/if}

      {#if currentUser && can(currentUser, "VIEW_ROLE_LIST")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() => openView("roles-settings", "Roles y Permisos"))}
        >
          Roles y Permisos
        </button>
      {/if}

      <!-- Grupo 2: Datos -->
      {#if currentUser && can(currentUser, "VIEW_SETTINGS_BACKUP")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() =>
              openView("export-settings", "Configuración de Exportación"),
            )}
        >
          Exportación
        </button>
      {/if}
      {#if currentUser && can(currentUser, "VIEW_SETTINGS_BACKUP")}
        <button
          class="settings-menu-item"
          onclick={() =>
            handleAction(() =>
              openView("backup-settings", "Copias de Seguridad"),
            )}
        >
          Copias de Seguridad
        </button>
      {/if}

      {#if currentUser && ([ROLE_ADMIN_ID.toLowerCase()].includes(currentUser.roleId.toLowerCase()) || ["admin", "administrador"].includes(currentUser.roleName.toLowerCase()))}
        <div class="settings-menu-separator"></div>
        <button
          class="settings-menu-item text-red-400 hover:text-red-300"
          onclick={() =>
            handleAction(() =>
              openView("trash-settings", "Papelera de Reciclaje"),
            )}
        >
          Papelera
        </button>
      {/if}

      {#if currentUser?.isSuperuser || ["admin", "administrador"].includes((currentUser?.roleName || "").toLowerCase())}
        <button
          class="settings-menu-item text-orange-400 hover:text-orange-300"
          onclick={() => handleAction(onRequestReindex)}
        >
          Reindexar Búsqueda
        </button>
      {/if}

      <div class="settings-menu-separator"></div>

      <!-- Grupo 3: Sistema -->
      <button
        class="settings-menu-item"
        onclick={() => handleAction(onOpenUpdate)}
      >
        Buscar Actualizaciones...
      </button>

      <!-- GOD MODE EXCLUSIVE -->
      {#if currentUser?.isSuperuser}
        <div class="settings-menu-separator"></div>
        <button
          class="settings-menu-item text-yellow-400 hover:text-yellow-300"
          onclick={() =>
            handleAction(() => openView("dev-settings", "Modo Ingeniería"))}
        >
          ⚡ Gestión de Módulos
        </button>
      {/if}

      <button
        class="settings-menu-item"
        onclick={() => handleAction(onOpenAbout)}
      >
        Acerca de Mega Brisas
      </button>
    </div>
  {/if}
</div>

<style>
  .settings-menu {
    position: absolute;
    bottom: 0;
    left: 52px;
    z-index: 2000;
    min-width: 220px;
    padding: 4px 0;
    background: #1e1e1e;
    border: 1px solid #454545;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-menu-item {
    display: block;
    width: calc(100% - 8px);
    margin: 2px 4px;
    padding: 6px 12px;
    text-align: left;
    font-size: 13px;
    color: #cccccc;
    background: transparent;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-family: "Segoe UI", system-ui, sans-serif;
    transition: all 0.15s ease;
  }

  .settings-menu-item:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: white;
  }

  .settings-menu-separator {
    height: 1px;
    background-color: #454545;
    margin: 4px 0;
  }
</style>
