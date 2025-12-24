<script lang="ts">
  import IngresoLayout from "./layout/IngresoLayout.svelte";
  import IngresoDashboard from "./dashboard/IngresoDashboard.svelte";
  import ContratistaIngresoList from "./lists/ContratistaIngresoList.svelte";
  import VisitaIngresoList from "./lists/VisitaIngresoList.svelte";
  import ProveedorIngresoList from "./lists/ProveedorIngresoList.svelte";

  let activeTab = "dashboard";
</script>

<IngresoLayout bind:activeTab>
  {#if activeTab === "dashboard"}
    <IngresoDashboard />
  {:else if activeTab === "contratistas"}
    <ContratistaIngresoList />
  {:else if activeTab === "visitas"}
    <VisitaIngresoList />
  {:else if activeTab === "proveedores"}
    <ProveedorIngresoList />
  {/if}
</IngresoLayout>
