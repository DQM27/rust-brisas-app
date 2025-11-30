<script lang="ts">
  import { toast } from "svelte-5-french-toast";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import { ingresoStore } from "$lib/stores/ingresoStore";

  import IngresoForm from "./IngresoForm.svelte";
  import IngresosActivosTable from "./IngresosActivosTable.svelte";

  let formLoading = false;

  async function handleRegistrarEntrada(event: CustomEvent) {
    formLoading = true;
    const result = await ingresoService.registrarEntrada(event.detail);

    if (result.ok) {
      toast.success("Entrada registrada correctamente");
      ingresoStore.add(result.data); // Actualizar store localmente
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 h-full p-4">
  <!-- Panel Izquierdo: Formulario de Entrada -->
  <div class="lg:col-span-1">
    <IngresoForm loading={formLoading} on:submit={handleRegistrarEntrada} />
  </div>

  <!-- Panel Derecho: Lista de Activos -->
  <div class="lg:col-span-2 h-full min-h-0">
    <IngresosActivosTable />
  </div>
</div>
