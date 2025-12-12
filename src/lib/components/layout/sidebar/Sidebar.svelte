<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
  import { activeView } from "$lib/stores/ui";
  import { isAuthenticated, logout, currentUser } from "$lib/stores/auth";
  import { resetTabs, openTab } from "$lib/stores/tabs";
  import { shortcutService } from "$lib/services/shortcutService";
  import { onMount } from "svelte";

  // Importar iconos directamente
  import {
    User,
    Lock,
    FileText,
    Settings,
    LogOut,
    HardHat,
  } from "lucide-svelte";

  // Componentes
  import SidebarIcon from "./SidebarIcon.svelte";
  import SidebarPanel from "./SidebarPanel.svelte";

  // Paneles
  import UsersPanel from "./panels/UsersPanel.svelte";
  import AccessPanel from "./panels/AccessPanel.svelte";
  import LogsPanel from "./panels/LogsPanel.svelte";
  import SettingsPanel from "./panels/SettingsPanel.svelte";
  import ContractorsPanel from "./panels/ContractorsPanel.svelte";

  // Store y tipos
  import { activePanel } from "$lib/stores/sidebar";
  import type { SidebarItem } from "../../../types/Sidebar";
  import { can } from "$lib/logic/permissions";

  // Items configurables - Definición base
  const allSidebarItems: SidebarItem[] = [
    {
      id: "users",
      icon: User,
      label: "Usuarios",
      panelComponent: UsersPanel,
      permission: "VIEW_USER_LIST",
    },
    {
      id: "contractors",
      icon: HardHat,
      label: "Contratistas",
      panelComponent: ContractorsPanel,
    },
    {
      id: "access",
      icon: Lock,
      label: "Accesos",
      panelComponent: AccessPanel,
    },
    {
      id: "logs",
      icon: FileText,
      label: "Logs",
      panelComponent: LogsPanel,
      role: ["admin", "supervisor"], // Explicit role check fallback or new permission
    },
    {
      id: "settings",
      icon: Settings,
      label: "Configuración",
      panelComponent: SettingsPanel,
      role: ["admin"],
    },
  ];

  // Filtrar items según permisos
  $: sidebarItems = allSidebarItems.filter((item) => {
    if (!item) return false;

    // 1. Check permission if defined
    // @ts-ignore
    if (item.permission && !can($currentUser, item.permission)) {
      return false;
    }

    // 2. Check strict role if defined (legacy/simple way)
    // @ts-ignore
    if (item.role && $currentUser) {
      // @ts-ignore
      if (!item.role.includes($currentUser.role)) {
        return false;
      }
    }

    return true;
  });

  $: currentActivePanel = $activePanel;
  $: activeItem = sidebarItems.find((item) => item.id === currentActivePanel);

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

  function openProfile() {
    openTab({
      componentKey: "user-profile",
      title: "Mi Perfil",
      id: "user-profile-tab",
    });
  }

  // Derivar iniciales del usuario actual
  $: userInitials = $currentUser
    ? `${$currentUser.nombre?.[0] || ""}${$currentUser.apellido?.[0] || ""}`.toUpperCase()
    : "";

  $: userName = $currentUser?.nombre || "Usuario";
  // ==========================================
  // ATAJOS GLOBALES (MÓDULOS)
  // ==========================================

  onMount(() => {
    // Registrar handlers globales para navegación de módulos
    const unregs = [
      shortcutService.registerHandler("root", "module.users", () => {
        // activePanel.set("users"); // Solo abre sidebar
        openTab({
          componentKey: "user-list",
          title: "Lista de Usuarios",
          id: "user-list",
          focusOnOpen: true,
        });
      }),
      shortcutService.registerHandler("root", "module.contractors", () => {
        openTab({
          componentKey: "contratista-list",
          title: "Lista de Contratistas",
          id: "contratista-list",
          focusOnOpen: true,
        });
      }),
      shortcutService.registerHandler("root", "module.access", () =>
        activePanel.set("access"),
      ), // Access suele ser solo panel? Verificar si tiene Tab
      shortcutService.registerHandler("root", "module.logs", () =>
        activePanel.set("logs"),
      ),
      shortcutService.registerHandler("root", "module.ingreso", () => {
        openTab({
          componentKey: "ingreso-list",
          title: "Control de Ingresos",
          id: "ingreso-list",
          focusOnOpen: true,
        });
      }),
      shortcutService.registerHandler("root", "app.settings", () =>
        activePanel.set("settings"),
      ),
      shortcutService.registerHandler("root", "user.profile", openProfile),
    ];

    return () => unregs.forEach((u) => u());
  });
</script>

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
      <!-- User Avatar -->
      <button class="user-avatar" title={userName} on:click={openProfile}>
        {userInitials}
      </button>

      <!-- Logout Button -->
      <button
        on:click={handleLogout}
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
</div>
