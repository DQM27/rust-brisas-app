<!-- src/lib/components/layout/sidebar/panels/ContractorsPanel.svelte -->
<script lang="ts">
  import { HardHat, UserPlus, FileText } from "lucide-svelte";
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

  function openContractorRegister() {
    openTab({
      componentKey: "contratista",
      title: "Registro de Contratista",
      id: "contratista-register", // Singleton ID to prevent duplicates
      focusOnOpen: true,
    });
  }
</script>

<div class="panel-section">
  <div class="panel-section-title">GESTIÓN DE CONTRATISTAS</div>
  <div class="panel-item non-clickable">
    <svelte:component this={HardHat} size={16} />
    <span>Lista de contratistas</span>
  </div>
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
  <div class="panel-section-title">DOCUMENTACIÓN</div>
  <div class="panel-item non-clickable">
    <svelte:component this={FileText} size={16} />
    <span>Documentos requeridos</span>
  </div>
</div>

<style>
  .panel-section {
    margin-bottom: 16px;
  }

  .panel-section-title {
    font-size: 11px;
    text-transform: uppercase;
    padding: 8px 15px 4px;
    color: #858585;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  .panel-item {
    padding: 6px 15px;
    cursor: pointer;
    font-size: 13px;
    display: flex;
    align-items: center;
    color: #cccccc;
    transition: background-color 0.1s;
    width: 100%;
    text-align: left;
    border-radius: 0;
    gap: 8px;
    background: none;
    border: none;
  }

  .panel-item.non-clickable {
    cursor: default;
    color: #858585;
  }

  .panel-item.non-clickable:hover {
    background-color: transparent;
  }

  .panel-item:hover:not(.non-clickable) {
    background-color: #2a2d2e;
  }
</style>
