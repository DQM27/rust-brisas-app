<script lang="ts">
  import { onMount } from "svelte";
  import ListaNegraListForm from "./ListaNegraListForm.svelte";
  import { listaNegra } from "$lib/api/listaNegra";
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

    try {
      const result = await listaNegra.getAll();
      bloqueados = result.bloqueados;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  }

  async function handleAddToBlacklist(data: any) {
    try {
      await listaNegra.add(data);
      await loadListaNegra();
      // TODO: Mostrar notificación de éxito
    } catch (err) {
      // TODO: Mostrar notificación de error
      console.error("Error al agregar a lista negra:", err);
    }
  }

  /**
   * Handler unificado que detecta automáticamente si es desbloqueo o re-bloqueo
   * según el estado actual de isActive
   */
  async function handleUnblockOrReblock(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    // Encontrar el bloqueado actual para verificar su estado
    const bloqueado = bloqueados.find((b) => b.id === data.id);

    if (!bloqueado) {
      console.error("Bloqueado no encontrado");
      return;
    }

    try {
      if (bloqueado.isActive) {
        // Caso 1: Está bloqueado → DESBLOQUEAR
        console.log("Desbloqueando persona:", bloqueado.nombreCompleto);
        await listaNegra.remove(
          data.id,
          data.motivoDesbloqueo,
          data.observaciones,
        );
      } else {
        // Caso 2: Está desbloqueado → RE-BLOQUEAR
        console.log("Re-bloqueando persona:", bloqueado.nombreCompleto);
        await listaNegra.reactivate(
          data.id,
          data.motivoDesbloqueo, // En contexto de re-bloqueo es "motivoBloqueo"
          data.observaciones,
        );
      }

      // Recargar la lista para reflejar cambios
      await loadListaNegra();
      // TODO: Mostrar notificación de éxito
      console.log("Operación exitosa");
    } catch (err) {
      // TODO: Mostrar notificación de error
      console.error("Error en operación:", err);
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
  onUnblock={handleUnblockOrReblock}
/>