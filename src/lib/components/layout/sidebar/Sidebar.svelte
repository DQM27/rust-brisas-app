<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from "$lib/stores/ui";
  import { isAuthenticated, logout, currentUser } from "$lib/stores/auth";
  import { resetTabs, openTab } from "$lib/stores/tabs";
  import { modulesStore } from "$lib/stores/modules"; // Import modulesStore
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
    Users,
    Zap, // Importar icono de rayo
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
  import { ROLE_ADMIN_ID } from "$lib/types/role";
  import UserFormModal from "$lib/components/user/UserFormModal.svelte";
  import UpdateModal from "$lib/components/settings/modals/UpdateModal.svelte";
  import BackupModal from "$lib/components/settings/modals/BackupModal.svelte";
  import AboutModal from "$lib/components/settings/modals/AboutModal.svelte";
  import ProfileModal from "$lib/components/user/ProfileModal.svelte";
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
      permission: "VIEW_USER_DETAIL", // Changed from VIEW_USER_LIST so 'View' toggle controls sidebar
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
      permission: "VIEW_PROVIDER_DETAIL", // Changed from VIEW_PROVIDER_LIST
    },
    {
      id: "visitantes",
      icon: Users,
      label: "Visitantes",
      action: () => {
        openTab({
          componentKey: "visitante-list",
          title: "Lista de Visitantes",
          id: "visitante-list",
          focusOnOpen: true,
        });
      },
      permission: "VIEW_VISITOR_LIST",
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
      permission: "VIEW_BLACKLIST",
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
      permission: "VIEW_APPOINTMENT_LIST",
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
      permission: "VIEW_GAFETE_LIST",
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
      permission: "VIEW_ENTRY_LIST",
    },
    {
      id: "logs",
      icon: FileText,
      label: "Logs",
      roleId: [ROLE_ADMIN_ID],
    },
  ];

  // Estado del menú de configuración
  let showSettingsMenu = $state(false);
  let showUpdateModal = $state(false);
  let showBackupModal = $state(false);
  let showAboutModal = $state(false);
  let showProfileMenu = $state(false);

  function toggleSettingsMenu(e: MouseEvent) {
    e.stopPropagation();
    showSettingsMenu = !showSettingsMenu;
    showProfileMenu = false; // Close profile menu if open
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
    if (showProfileMenu) {
      showProfileMenu = false;
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

  // Mapa de IDs de sidebar a Keys de Módulos (Backend)
  const MODULE_KEY_MAP: Record<string, string> = {
    users: "users",
    contractors: "contractors",
    proveedores: "providers",
    visitantes: "visits",
    blacklist: "access_control",
    citas: "visits",
    gafetes: "access_control",
    ingresos: "access_control",
    logs: "reports",
  };

  function handleItemSelect(item: SidebarItem) {
    // 1. Verificar Estado del Módulo (Backend-Driven)
    const moduleKey = MODULE_KEY_MAP[item.id] || item.id;
    const status = modulesStore.getStatus(moduleKey, $modulesStore);

    // 2. Verificar Bloqueo (Dev/Maintenance) -> Solo GOD pasa
    if (
      (status === "development" || status === "maintenance") &&
      !$currentUser?.isSuperuser
    ) {
      openTab({
        componentKey: "under-construction",
        title: item.label,
        id: `locked-${item.id}`,
        data: {
          type: status,
          moduleName: item.label,
        },
        focusOnOpen: true,
      });
      return;
    }

    // 3. Flujo normal
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

  const userName = $derived(
    $currentUser
      ? `${$currentUser.nombre || ""} ${$currentUser.apellido || ""}`.trim() ||
          "Usuario"
      : "Usuario",
  );

  // Avatar
  let avatarUrl = $state<string | null>(null);

  async function loadUserAvatar(userId: string) {
    console.log("DEBUG: Loading avatar for user:", userId);
    const result = await userService.getUserAvatar(userId);
    console.log("DEBUG: Avatar result:", result.ok ? "Success" : result.error);
    if (result.ok) {
      avatarUrl = `data:image/webp;base64,${result.data}`;
    } else {
      avatarUrl = null;
    }
  }

  $effect(() => {
    if ($currentUser) {
      loadUserAvatar($currentUser.id);
      console.log("Current User Role Debug:", {
        id: $currentUser.id,
        roleId: $currentUser.roleId,
        roleName: $currentUser.roleName,
        isAdmin:
          $currentUser.roleId.toLowerCase() === ROLE_ADMIN_ID.toLowerCase() ||
          ["admin", "administrador", "supervisor"].includes(
            ($currentUser.roleName || "").toLowerCase(),
          ),
      });
    } else {
      avatarUrl = null;
    }
  });

  onMount(async () => {
    // 1. Hidratar sesión (asegurar que currentUser tenga datos frescos de DB)
    if ($currentUser) {
      try {
        const res = await userService.fetchUserById($currentUser.id);
        if (res.ok) {
          // Usamos reloadSession para actualizar store sin recargar página
          currentUser.set(res.data);
          loadUserAvatar(res.data.id);
        }
      } catch (e) {
        console.error("Error refreshing session:", e);
      }
    }
  });

  // Profile Modal Logic
  let showProfileModal = $state(false); // Edit Mode
  let showProfileViewModal = $state(false); // View Mode
  let profileLoading = $state(false);

  function openProfile() {
    showProfileViewModal = true;
  }

  function handleEditProfile() {
    showProfileViewModal = false;
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

    <!-- GOD MODE INTRINSIC ICON -->
    {#if $currentUser?.isSuperuser}
      <div class="mt-auto mb-2 flex justify-center">
        <button
          class="sidebar-icon-btn group text-yellow-500 hover:text-yellow-400"
          title="Modo Ingeniería (GOD)"
          onclick={() => openView("dev-settings", "Modo Ingeniería")}
        >
          <Zap
            size={24}
            strokeWidth={2.5}
            class="transition-all duration-300 group-hover:scale-110 group-hover:drop-shadow-[0_0_8px_rgba(250,204,21,0.5)]"
          />
          <span class="sidebar-icon-tooltip">Ingeniería</span>
        </button>
      </div>
    {/if}

    <div class="sidebar-bottom-actions">
      <!-- User Profile & Menu -->
      <div class="relative w-full flex items-center justify-center">
        <button
          class="user-avatar overflow-hidden flex items-center justify-center p-0"
          title={userName}
          onclick={(e) => {
            e.stopPropagation();
            showProfileMenu = !showProfileMenu;
            if (showProfileMenu) showSettingsMenu = false;
          }}
        >
          {#if avatarUrl}
            <img
              src={avatarUrl}
              alt="Avatar"
              class="w-full h-full object-cover"
            />
          {:else}
            {userInitials}
          {/if}
        </button>

        {#if showProfileMenu}
          <div
            class="settings-menu"
            onclick={(e) => e.stopPropagation()}
            role="menu"
            tabindex="-1"
            onkeydown={(e) => e.key === "Escape" && (showProfileMenu = false)}
            style="bottom: 48px;"
          >
            <!-- Profile Actions -->
            <div class="px-3 py-2 border-b border-[#454545] mb-1">
              <p class="text-xs font-semibold text-white">{userName}</p>
              <p class="text-[10px] text-gray-400">{$currentUser?.email}</p>
              {#if $currentUser?.roleName}
                {#if $currentUser.isSuperuser}
                  <div
                    class="mt-1 inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-purple-500/30 text-purple-300 uppercase tracking-wide"
                  >
                    ⚡ GOD MODE
                  </div>
                {:else}
                  <div
                    class="mt-1 inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-[#2da44e]/20 text-[#2da44e] uppercase tracking-wide"
                  >
                    {$currentUser.roleName}
                  </div>
                {/if}
              {/if}
              <!-- DEBUG HELP -->
              <div class="hidden">
                DEBUG ROLE: {$currentUser?.roleName} | SUPERUSER: {$currentUser?.isSuperuser}
              </div>
            </div>

            <button
              class="settings-menu-item"
              onclick={() => {
                openProfile();
                showProfileMenu = false;
              }}
            >
              Ver Perfil
            </button>

            <div class="settings-menu-separator"></div>

            <button
              class="settings-menu-item text-red-400 hover:text-red-300"
              onclick={() => {
                handleLogout();
                showProfileMenu = false;
              }}
            >
              Cerrar Sesión
            </button>
          </div>
        {/if}
      </div>

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
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_GENERAL")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("general-settings", "Ajustes Generales"),
                  )}
              >
                Ajustes Generales
              </button>
            {/if}
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_VISUAL")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("visual-settings", "Ajustes Gráficos"),
                  )}
              >
                Ajustes Gráficos
              </button>
            {/if}
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_SECURITY")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("security-settings", "Seguridad y Credenciales"),
                  )}
              >
                Seguridad y Credenciales
              </button>
            {/if}
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_SESSIONS")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("session-settings", "Gestión de Sesión"),
                  )}
              >
                Gestión de Sesión
              </button>
            {/if}

            {#if $currentUser && can($currentUser, "VIEW_ROLE_LIST")}
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
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_BACKUP")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("export-settings", "Configuración de Exportación"),
                  )}
              >
                Exportación
              </button>
            {/if}
            {#if $currentUser && can($currentUser, "VIEW_SETTINGS_BACKUP")}
              <button
                class="settings-menu-item"
                onclick={() =>
                  handleSettingsAction(() => (showBackupModal = true))}
              >
                Generar Respaldo
              </button>
            {/if}

            {#if $currentUser && ([ROLE_ADMIN_ID.toLowerCase()].includes($currentUser.roleId.toLowerCase()) || ["admin", "administrador"].includes($currentUser.roleName.toLowerCase()))}
              <div class="settings-menu-separator"></div>
              <button
                class="settings-menu-item text-red-400 hover:text-red-300"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("trash-settings", "Papelera de Reciclaje"),
                  )}
              >
                Papelera
              </button>
            {/if}

            <div class="settings-menu-separator"></div>

            <!-- Grupo 4: Sistema -->
            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() => (showUpdateModal = true))}
            >
              Buscar Actualizaciones...
            </button>

            <!-- GOD MODE EXCLUSIVE -->
            {#if $currentUser?.isSuperuser}
              <div class="settings-menu-separator"></div>
              <button
                class="settings-menu-item text-yellow-400 hover:text-yellow-300"
                onclick={() =>
                  handleSettingsAction(() =>
                    openView("dev-settings", "Modo Ingeniería"),
                  )}
              >
                ⚡ Gestión de Módulos
              </button>
            {/if}

            <button
              class="settings-menu-item"
              onclick={() =>
                handleSettingsAction(() => (showAboutModal = true))}
            >
              Acerca de Mega Brisas
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Panel lateral desplegable -->
  {#if activeItem}
    <SidebarPanel item={activeItem} onClose={handlePanelClose} />
  {/if}

  {#if $currentUser}
    <ProfileModal
      show={showProfileViewModal}
      user={$currentUser}
      onClose={() => (showProfileViewModal = false)}
      onEdit={handleEditProfile}
    />
    <UserFormModal
      show={showProfileModal}
      user={$currentUser}
      loading={profileLoading}
      onSave={handleSaveProfile}
      onClose={() => (showProfileModal = false)}
      isSelfEdit={true}
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
    left: 52px; /* 52px sidebar + 0px gap (flush) */
    z-index: 2000;
    min-width: 220px;
    padding: 0px 0;
    background: #1e1e1e; /* VS Code menu bg */
    border: 1px solid #454545;
    border-radius: 4px;
    /* Box-shadow removed as per user request ("quitale la sobra") */
    /* box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4); */
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
