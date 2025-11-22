<!-- src/lib/components/layout/sidebar/panels/UsersPanel.svelte -->
<script lang="ts">
  import { Users, UserPlus, Edit3, BarChart3, Home } from "lucide-svelte";
  import { openView, activePanel } from "../../../../stores/sidebar";

  // Tipos para las acciones del panel
  type PanelAction = () => void;

  // Función para manejar teclado
  function handleKeydown(e: KeyboardEvent, action: PanelAction): void {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      action();
    }
  }

  // Wrapper para acciones que cierra el panel después de ejecutar
  function executeAndClose(action: PanelAction): PanelAction {
    return () => {
      action();
      // Cerrar inmediatamente - la animación se encarga de la suavidad
      activePanel.set(null);
    };
  }

  // Definición de items del panel para mejor mantenibilidad
  interface PanelMenuItem {
    icon: typeof Users;
    label: string;
    action: PanelAction;
  }

  interface PanelSection {
    title: string;
    items: PanelMenuItem[];
  }

  const sections: PanelSection[] = [
    {
      title: "GESTIÓN DE USUARIOS",
      items: [
        {
          icon: Users,
          label: "Lista de usuarios",
          action: executeAndClose(() =>
            openView("user-list", "Lista de Usuarios"),
          ),
        },
        {
          icon: UserPlus,
          label: "Registrar usuario",
          action: executeAndClose(() =>
            openView("user-register", "Registrar Usuario"),
          ),
        },
        {
          icon: Edit3,
          label: "Editor de usuarios",
          action: executeAndClose(() =>
            openView("user-editor", "Editor de Usuarios"),
          ),
        },
      ],
    },
    {
      title: "VISTAS",
      items: [
        {
          icon: BarChart3,
          label: "Dashboard",
          action: executeAndClose(() => openView("dashboard", "Dashboard")),
        },
        {
          icon: Home,
          label: "Página de bienvenida",
          action: executeAndClose(() => openView("welcome", "Bienvenida")),
        },
      ],
    },
  ];

  // Clases compartidas
  const sectionTitleClasses =
    "px-[15px] pb-1 pt-2 text-[11px] font-semibold uppercase tracking-wider text-[#858585]";
  const menuItemClasses = `
    group flex w-full items-center gap-2 rounded-none border-none bg-transparent 
    px-[15px] py-1.5 text-left text-[13px] text-[#cccccc] cursor-pointer
    transition-all duration-100 ease-in-out
    hover:bg-[#2a2d2e]
    focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-blue-500
    active:scale-[0.99]
  `;
</script>

{#each sections as section}
  <div class="mb-4">
    <div class={sectionTitleClasses}>
      {section.title}
    </div>

    {#each section.items as item}
      <button
        class={menuItemClasses}
        on:click={item.action}
        on:keydown={(e) => handleKeydown(e, item.action)}
        type="button"
      >
        <svelte:component
          this={item.icon}
          size={16}
          class="transition-transform duration-150 group-hover:scale-110"
        />
        <span class="transition-colors duration-150">
          {item.label}
        </span>
      </button>
    {/each}
  </div>
{/each}
