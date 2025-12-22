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
  } from "lucide-svelte";

  // Componentes
  import SidebarIcon from "./SidebarIcon.svelte";
  import SidebarPanel from "./SidebarPanel.svelte";

  // Paneles
  import AccessPanel from "./panels/AccessPanel.svelte";
  import LogsPanel from "./panels/LogsPanel.svelte";
  import SettingsPanel from "./panels/SettingsPanel.svelte";
  import ContractorsPanel from "./panels/ContractorsPanel.svelte";
  import ProveedoresPanel from "./panels/ProveedoresPanel.svelte";

  // Store y tipos
  import { activePanel } from "$lib/stores/sidebar";
  import type { SidebarItem } from "../../../types/Sidebar";
  import { can } from "$lib/logic/permissions";
  import UserFormModal from "$lib/components/user/UserFormModal.svelte";
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
      panelComponent: ContractorsPanel,
    },
    {
      id: "proveedores",
      icon: Package,
      label: "Proveedores",
      panelComponent: ProveedoresPanel,
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
  const sidebarItems = $derived(
    allSidebarItems.filter((item) => {
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
  // ==========================================
  // ATAJOS GLOBALES (MÓDULOS)
  // ==========================================

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

    // Registrar handlers globales para navegación de módulos
    // (Atajos eliminados a petición del usuario)
  });

  // Profile Modal Logic
  let showProfileModal = $state(false);
  let profileLoading = $state(false);

  function openProfile() {
    showProfileModal = true;
  }

  async function handleSaveProfile(data: CreateUserInput | UpdateUserInput) {
    if (!$currentUser) return;

    profileLoading = true;
    try {
      const result = await userService.updateUser(
        $currentUser.id,
        data as UpdateUserInput,
      );

      if (result.ok) {
        toast.success("Perfil actualizado correctamente");

        // Update global store
        // currentUser.set(result.data); // Handled by userService now

        showProfileModal = false;
      } else {
        toast.error(result.error);
      }
    } catch (err) {
      console.error(err);
      toast.error("Error al guardar perfil");
    } finally {
      profileLoading = false;
    }
  }
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
</div>
