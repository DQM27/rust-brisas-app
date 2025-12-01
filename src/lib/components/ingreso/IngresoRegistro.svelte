<script lang="ts">
  import { toast } from "svelte-5-french-toast";
  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import { ingresoStore } from "$lib/stores/ingresoStore";
  import { ingresoFormStore } from "$lib/stores/ingresoFormStore";

  import IngresoForm from "./IngresoForm.svelte";

  let formLoading = false;

  async function handleRegistrarEntrada(event: CustomEvent) {
    formLoading = true;
    const result = await ingresoService.registrarEntrada(event.detail);

    if (result.ok) {
      toast.success("Entrada registrada correctamente");
      ingresoStore.add(result.data); // Actualizar store localmente
      ingresoFormStore.resetIngresoFields(); // Limpiar campos pero mantener contratista (opcional, o reset total)
      ingresoFormStore.reset(); // Reset total mejor para flujo limpio
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }
</script>

<div class="h-full p-4 overflow-y-auto">
  <IngresoForm loading={formLoading} on:submit={handleRegistrarEntrada} />
</div>
