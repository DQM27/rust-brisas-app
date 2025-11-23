<script lang="ts">
  import { onMount } from "svelte";
  import ListaNegraListForm from "./ListaNegraListForm.svelte";
  import { submitFetchAllListaNegra } from "$lib/logic/listaNegra/submitFetchListaNegra";
  import { submitAddToListaNegra } from "$lib/logic/listaNegra/submitAddToListaNegra";
  import { submitUnblockListaNegra } from "$lib/logic/listaNegra/submitUnblockListaNegra";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  let bloqueados = $state<ListaNegraResponse[]>([]);
  let loading = $state(false);
  let error = $state("");

  async function loadListaNegra() {
    loading = true;
    error = "";

    const result = await submitFetchAllListaNegra();

    if (result.ok) {
      bloqueados = result.data.bloqueados;
    } else {
      error = result.error;
    }

    loading = false;
  }

  async function handleAddToBlacklist(data: any) {
    const result = await submitAddToListaNegra(data);

    if (result.ok) {
      // Recargar la lista
      await loadListaNegra();
      // TODO: Mostrar notificación de éxito
    } else {
      // TODO: Mostrar notificación de error
      console.error("Error al agregar a lista negra:", result.error);
    }
  }

  async function handleUnblock(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    const result = await submitUnblockListaNegra(
      data.id,
      data.motivoDesbloqueo,
      data.observaciones,
    );

    if (result.ok) {
      // Recargar la lista
      await loadListaNegra();
      // TODO: Mostrar notificación de éxito
    } else {
      // TODO: Mostrar notificación de error
      console.error("Error al desbloquear:", result.error);
    }
  }

  onMount(() => {
    loadListaNegra();
  });
</script>

<ListaNegraListForm
  {bloqueados}
  {loading}
  {error}
  onRefresh={loadListaNegra}
  onAddToBlacklist={handleAddToBlacklist}
  onUnblock={handleUnblock}
/>
