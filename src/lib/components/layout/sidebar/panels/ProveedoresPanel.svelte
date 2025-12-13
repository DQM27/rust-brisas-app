<!-- src/lib/components/layout/sidebar/panels/ProveedoresPanel.svelte -->
<script lang="ts">
  import { Package, UserPlus, FileText } from "lucide-svelte";
  import { activePanel } from "../../../../stores/sidebar";
  import { openTab } from "$lib/stores/tabs";

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

  function openProveedorList() {
    openTab({
      componentKey: "proveedor-list",
      title: "Catálogo de Proveedores",
      id: "proveedor-list",
      focusOnOpen: true,
    });
  }

  function openProveedorRegister() {
    openTab({
      componentKey: "proveedor",
      title: "Registrar Proveedor",
      id: "proveedor-register",
      focusOnOpen: true,
    });
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">GESTIÓN DE PROVEEDORES</div>
  <button
    class="panel-item"
    on:click={executeAndClose(openProveedorList)}
    on:keydown={(e) => handleKeydown(e, executeAndClose(openProveedorList))}
  >
    <svelte:component this={Package} size={16} />
    <span>Listar proveedores</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(openProveedorRegister)}
    on:keydown={(e) => handleKeydown(e, executeAndClose(openProveedorRegister))}
  >
    <svelte:component this={UserPlus} size={16} />
    <span>Registrar proveedor</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">INFORMACIÓN</div>
  <div class="panel-item non-clickable">
    <svelte:component this={FileText} size={16} />
    <span>Empresas proveedoras</span>
  </div>
</div>
