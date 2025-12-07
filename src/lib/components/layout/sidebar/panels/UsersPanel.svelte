<!-- src/lib/components/layout/sidebar/panels/UsersPanel.svelte -->
<script lang="ts">
  import { Users, UserPlus } from "lucide-svelte";
  import { openView, activePanel } from "../../../../stores/sidebar";

  type PanelAction = () => void;

  function handleKeydown(e: KeyboardEvent, action: PanelAction): void {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      action();
    }
  }

  function executeAndClose(action: PanelAction): PanelAction {
    return () => {
      action();
      activePanel.set(null);
    };
  }

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
      ],
    },
  ];
</script>

{#each sections as section}
  <div class="panel-section">
    <div class="panel-section-title">
      {section.title}
    </div>

    {#each section.items as item}
      <button
        class="panel-item"
        on:click={item.action}
        on:keydown={(e) => handleKeydown(e, item.action)}
        type="button"
      >
        <svelte:component this={item.icon} size={16} />
        <span>
          {item.label}
        </span>
      </button>
    {/each}
  </div>
{/each}
