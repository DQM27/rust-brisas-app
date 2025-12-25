<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from "$lib/stores/ui";
  import { isAuthenticated, logout, currentUser } from "$lib/stores/auth";
  import { resetTabs, openTab } from "$lib/stores/tabs";
  import { onMount } from "svelte";

  // Importar iconos directamente
  import {
    User,
    Lock,
    FileText,
    Settings,
    LogOut,
    HardHat,
    Package,
    Calendar,
    BadgeCheck,
    LogIn,
    Ban,
  } from "lucide-svelte";

  // Componentes
  import SidebarIcon from "./SidebarIcon.svelte";
  import SidebarPanel from "./SidebarPanel.svelte";

  // Paneles
  // AccessPanel removed as per user request
  import LogsPanel from "./panels/LogsPanel.svelte";
  import SettingsPanel from "./panels/SettingsPanel.svelte";

  // Store y tipos
  import { activePanel, openView } from "$lib/stores/sidebar";
  import type { SidebarItem } from "../../../types/Sidebar";
  import { can } from "$lib/logic/permissions";
  import { ROLE_ADMIN_ID, ROLE_SUPERVISOR_ID } from "$lib/types/role";
  import UserFormModal from "$lib/components/user/UserFormModal.svelte";
  import UpdateModal from "$lib/components/settings/modals/UpdateModal.svelte";
  import BackupModal from "$lib/components/settings/modals/BackupModal.svelte";
  import AboutModal from "$lib/components/settings/modals/AboutModal.svelte";
  import * as userService from "$lib/logic/user/userService";
  import type { CreateUserInput, UpdateUserInput } from "$lib/types/user";
  import { toast } from "svelte-5-french-toast";

  // Items configurables - Definición base
  const allSidebarItems: SidebarItem[] = [
    {
      id: "users",
      icon: User,
      label: "Usuarios",
      action: () => {
        openTab({
          componentKey: "user-list", // Debe coincidir con el mapa en TabsContent
          title: "Lista de Usuarios",
          id: "users-list",
          focusOnOpen: true,
        });
      },
      permission: "VIEW_USER_LIST",
    },
    {
      id: "contractors",
      icon: HardHat,
      label: "Contratistas",
      action: () => {
        openTab({
          componentKey: "contratista-list",
          title: "Lista de Contratistas",
          id: "contratista-list",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "proveedores",
      icon: Package,
      label: "Proveedores",
      action: () => {
        openTab({
          componentKey: "proveedor-list",
          title: "Lista de Proveedores",
          id: "proveedores-list",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "blacklist",
      icon: Ban,
      label: "Lista Negra",
      action: () => {
        openTab({
          componentKey: "lista-negra-list",
          title: "Lista Negra",
          id: "lista-negra-list",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "citas",
      icon: Calendar,
      label: "Visitas",
      action: () => {
        openTab({
          componentKey: "citas-view",
          title: "Pre-registro Visitas",
          id: "citas-view",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "gafetes",
      icon: BadgeCheck,
      label: "Gafetes",
      action: () => {
        openTab({
          componentKey: "gafete-list",
          title: "Gestión de Gafetes",
          id: "gafete-list",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "ingresos",
      icon: LogIn,
      label: "Ingresos",
      action: () => {
        openTab({
          componentKey: "ingreso-list",
          title: "Control de Ingresos",
          id: "ingreso-list",
          focusOnOpen: true,
        });
      },
    },
    {
      id: "logs",
      icon: FileText,
      label: "Logs",
      roleId: [ROLE_ADMIN_ID, ROLE_SUPERVISOR_ID],
    },
  ];

  // Estado del menú de configuración
  let showSettingsMenu = $state(false);
  let showUpdateModal = $state(false);
  let showBackupModal = $state(false);
  let showAboutModal = $state(false);

  function toggleSettingsMenu(e: MouseEvent) {
    e.stopPropagation();
    showSettingsMenu = !showSettingsMenu;
  }

  function closeSettingsMenu() {
    showSettingsMenu = false;
  }

  function handleSettingsAction(action: () => void) {
    action();
    closeSettingsMenu();
  }

  function handleWindowClick(e: MouseEvent) {
    if (showSettingsMenu) {
      closeSettingsMenu();
    }
  }

  // Filtrar items según permisos
  const sidebarItems = $derived(
    allSidebarItems.filter((item) => {
      if (!item) return false;

      // 1. Check permission if defined
      // @ts-ignore
      if (item.permission && !can($currentUser, item.permission)) {
        return false;
      }

      // 2. Check roleId if defined
      // @ts-ignore
      if (item.roleId && $currentUser) {
        // @ts-ignore
        if (!item.roleId.includes($currentUser.roleId)) {
          return false;
        }
      }

      return true;
    }),
  );

  const currentActivePanel = $derived($activePanel);
  const activeItem = $derived(
    sidebarItems.find((item) => item.id === currentActivePanel),
  );

  function handleItemSelect(item: SidebarItem) {
    activeView.set(item.id);

    if (item.action) {
      item.action();
      // Cerrar el panel después de ejecutar la acción
      activePanel.set(null);
    } else if (item.panelComponent) {
      // Toggle panel
      if ($activePanel === item.id) {
        activePanel.set(null);
      } else {
        activePanel.set(item.id);
      }
    }
  }

  function handlePanelClose() {
    activePanel.set(null);
  }

  function handleLogout() {
    logout();
  }

  // Derivar iniciales del usuario actual
  const userInitials = $derived(
    $currentUser
      ? `${$currentUser.nombre?.[0] || ""}${$currentUser.apellido?.[0] || ""}`.toUpperCase()
      : "",
  );

  const userName = $derived($currentUser?.nombre || "Usuario");

  onMount(async () => {
    // 1. Hidratar sesión (asegurar que currentUser tenga datos frescos de DB)
    if ($currentUser) {
      try {
        const res = await userService.fetchUserById($currentUser.id);
        if (res.ok) {
          // Usamos reloadSession para actualizar store sin recargar página
          currentUser.set(res.data);
        }
      } catch (e) {
        console.error("Error refreshing session:", e);
      }
    }
  });

  // Profile Modal Logic
  let showProfileModal = $state(false);
  let profileLoading = $state(false);

  function openProfile() {
    showProfileModal = true;
  }

  async function handleSaveProfile(
    data: CreateUserInput | UpdateUserInput,
  ): Promise<boolean> {
    if (!$currentUser) return false;

    profileLoading = true;
    try {
      const result = await userService.updateUser(
        $currentUser.id,
        data as UpdateUserInput,
      );

      if (result.ok) {
        toast.success("Perfil actualizado correctamente");
        return true;
      } else {
        toast.error(result.error);
        return false;
      }
    } catch (err) {
      console.error(err);
      toast.error("Error al guardar perfil");
      return false;
    } finally {
      profileLoading = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="flex h-full">
  <!-- Barra lateral de iconos -->
  <div class="sidebar-icons">
    <div class="flex flex-col gap-1.5">
      {#each sidebarItems as item}
        <SidebarIcon
          {item}
          isActive={currentActivePanel === item.id}
          onSelect={handleItemSelect}
        />
      {/each}
    </div>

    <div class="sidebar-bottom-actions">
      <!-- Settings Button (VS Code style) -->
      <div class="relative w-full flex items-center justify-center">
        <button
          onclick={toggleSettingsMenu}
          class="sidebar-icon-btn group {showSettingsMenu ? 'active' : ''}"
          title="Configuración"
        >
          <Settings
            size={22}
            class="transition-transform duration-200 group-hover:scale-110"
          />
          <span class="sidebar-icon-tooltip">Configuración</span>
        </button>

        {#if showSettingsMenu}
          <!-- Menú contextual estilo VS Code -->
          <div
            class="settings-menu"
            onclick={(e) => e.stopPropagation()}
            role="menu"
            tabindex="-1"
            onkeydown={(e) => e.key === "Escape" && closeSettingsMenu()}
          >
            <!-- Grupo 1: Configuración Principal -->
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("general-settings", "Ajustes Generales"),
                )}
            >
              Ajustes Generales
            </button>
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("visual-settings", "Ajustes Gráficos"),
                )}
            >
              Ajustes Gráficos
            </button>
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("security-settings", "Seguridad y Credenciales"),
                )}
            >
              Seguridad y Credenciales
            </button>
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("session-settings", "Gestión de Sesión"),
                )}
            >
              Gestión de Sesión
            </button>

            {#if $currentUser && can($currentUser, "VIEW_USER_LIST")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("roles-settings", "Roles y Permisos"),
                  )}
              >
                Roles y Permisos
              </button>
            {/if}

            <div class="settings-menu-separator"></div>

            <!-- Grupo 2: Herramientas -->
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("shortcut-settings", "Atajos de Teclado"),
                )}
            >
              Atajos de Teclado
            </button>
            <!-- Logs removed from settings menu as it is main sidebar item -->

            <div class="settings-menu-separator"></div>

            <!-- Grupo 3: Datos -->
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() =>
                  openView("export-settings", "Configuración de Exportación"),
                )}
            >
              Exportación
            </button>
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() => (showBackupModal = true))}
            >
              Generar Respaldo
            </button>

            <div class="settings-menu-separator"></div>

            <!-- Grupo 4: Sistema -->
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() => (showUpdateModal = true))}
            >
              Buscar Actualizaciones...
            </button>

            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() => (showAboutModal = true))}
            >
              Acerca de Brisas App
            </button>
          </div>
        {/if}
      </div>

      <!-- User Avatar -->
      <button class="user-avatar" title={userName} onclick={openProfile}>
        {userInitials}
      </button>

      <!-- Logout Button -->
      <button
        onclick={handleLogout}
        class="sidebar-action-btn group"
        title="Cerrar sesión"
      >
        <LogOut size={20} />
        <span class="sidebar-tooltip"> Cerrar sesión </span>
      </button>
    </div>
  </div>

  <!-- Panel lateral desplegable -->
  {#if activeItem}
    <SidebarPanel item={activeItem} onClose={handlePanelClose} />
  {/if}

  {#if $currentUser}
    <UserFormModal
      show={showProfileModal}
      user={$currentUser}
      loading={profileLoading}
      onSave={handleSaveProfile}
      onClose={() => (showProfileModal = false)}
    />
  {/if}

  <UpdateModal
    show={showUpdateModal}
    onClose={() => (showUpdateModal = false)}
  />
  <BackupModal
    show={showBackupModal}
    onClose={() => (showBackupModal = false)}
  />
  <AboutModal show={showAboutModal} onClose={() => (showAboutModal = false)} />
</div>

<style>
  /* Menú de configuración estilo VS Code */
  .settings-menu {
    position: absolute;
    bottom: 0;
    left: 46px; /* Ajustado un poco más cerca de la sidebar */
    z-index: 2000;
    min-width: 220px;
    padding: 0px 0;
    background: #1e1e1e; /* VS Code menu bg */
    border: 1px solid #454545;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-menu-item {
    display: block;
    width: 100%;
    padding: 6px 12px; /* VS Code style padding */
    text-align: left;
    font-size: 13px;
    color: #cccccc;
    background: transparent;
    border: none;
    cursor: pointer;
    font-family: "Segoe UI", system-ui, sans-serif;
  }

  .settings-menu-item:hover {
    background-color: #094771; /* VS Code hover blue */
    color: white;
  }

  .settings-menu-separator {
    height: 1px;
    background-color: #454545;
    margin: 4px 0;
  }
</style>
