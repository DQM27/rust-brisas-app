<!-- src/lib/components/layout/sidebar/panels/ContractorsPanel.svelte -->
<script lang="ts">
  import {
    HardHat,
    UserPlus,
    FileText,
    UserX,
    ShieldAlert,
  } from "lucide-svelte";
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

  function openContractorList() {
    openTab({
      componentKey: "contratista-list",
      title: "Lista de Contratistas",
      id: "contratista-list",
      focusOnOpen: true,
    });
  }

  function openContractorRegister() {
    openTab({
      componentKey: "contratista",
      title: "Registro de Contratista",
      id: "contratista-register",
      focusOnOpen: true,
    });
  }

  function openBlacklistList() {
    openTab({
      componentKey: "lista-negra-list",
      title: "Lista Negra",
      id: "lista-negra-list",
      focusOnOpen: true,
    });
  }

  function openAddToBlacklist() {
    openTab({
      componentKey: "lista-negra",
      title: "Agregar a Lista Negra",
      id: "lista-negra-add",
      focusOnOpen: true,
    });
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">GESTIÓN DE CONTRATISTAS</div>
  <button
    class="panel-item"
    on:click={executeAndClose(openContractorList)}
    on:keydown={(e) => handleKeydown(e, executeAndClose(openContractorList))}
  >
    <svelte:component this={HardHat} size={16} />
    <span>Listar contratistas</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(openContractorRegister)}
    on:keydown={(e) =>
      handleKeydown(e, executeAndClose(openContractorRegister))}
  >
    <svelte:component this={UserPlus} size={16} />
    <span>Registrar contratista</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">LISTA NEGRA</div>
  <button
    class="panel-item"
    on:click={executeAndClose(openBlacklistList)}
    on:keydown={(e) => handleKeydown(e, executeAndClose(openBlacklistList))}
  >
    <svelte:component this={UserX} size={16} />
    <span>Listar bloqueados</span>
  </button>
  <button
    class="panel-item"
    on:click={executeAndClose(openAddToBlacklist)}
    on:keydown={(e) => handleKeydown(e, executeAndClose(openAddToBlacklist))}
  >
    <svelte:component this={ShieldAlert} size={16} />
    <span>Agregar a lista negra</span>
  </button>
</div>

<div class="panel-section">
  <div class="panel-section-title">DOCUMENTACIÓN</div>
  <div class="panel-item non-clickable">
    <svelte:component this={FileText} size={16} />
    <span>Documentos requeridos</span>
  </div>
</div>