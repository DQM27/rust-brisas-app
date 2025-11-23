<script lang="ts">
  import { onMount } from "svelte";
  import ContratistaListForm from "./ContratistaListForm.svelte";
  import { submitFetchAllContratistas } from "$lib/logic/contratista/submitFetchContratistas";
  import { submitFetchAllListaNegra } from "$lib/logic/listaNegra/submitFetchListaNegra";
  import { submitAddToListaNegra } from "$lib/logic/listaNegra/submitAddToListaNegra";
  import { submitUnblockListaNegra } from "$lib/logic/listaNegra/submitUnblockListaNegra";
  import { listaNegra } from "$lib/api/listaNegra";
  import { currentUser } from "$lib/stores/auth";
  import type { ContratistaResponse } from "$lib/types/contratista";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let blockedContratistas = $state<Set<string>>(new Set());

  async function loadContratistas() {
    loading = true;
    error = "";

    const result = await submitFetchAllContratistas();

    if (result.ok) {
      contratistas = result.contratistas;
      // Cargar lista de bloqueados
      await loadBlockedContratistas();
    } else {
      error = result.error;
    }

    loading = false;
  }

  async function loadBlockedContratistas() {
    const result = await submitFetchAllListaNegra();

    if (result.ok) {
      const blocked = new Set<string>();
      result.data.bloqueados.forEach((b) => {
        if (b.isActive && b.contratistaId) {
          blocked.add(b.contratistaId);
        }
      });
      blockedContratistas = blocked;
    }
  }

  async function handleBlock(data: {
    contratistaId: string;
    motivoBloqueo: string;
    observaciones?: string;
  }) {
    const usuario = $currentUser;
    if (!usuario) {
      console.error("No hay usuario autenticado");
      return;
    }

    const bloqueadoPor = `${usuario.nombre} ${usuario.apellido}`;

    const result = await submitAddToListaNegra({
      ...data,
      bloqueadoPor,
    });

    if (result.ok) {
      // Actualizar lista de bloqueados
      await loadBlockedContratistas();
      // Recargar contratistas para actualizar estado
      await loadContratistas();
    } else {
      console.error("Error al bloquear:", result.error);
      // TODO: Mostrar notificación de error
    }
  }

  async function handleUnblock(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    // El ID que recibimos es el contratistaId, necesitamos encontrar el ID del registro de lista negra
    try {
      const contratista = contratistas.find((c) => c.id === data.id);
      if (!contratista) {
        console.error("Contratista no encontrado");
        return;
      }

      // Buscar el registro de lista negra por cédula
      const bloqueado = await listaNegra.getByCedula(contratista.cedula);
      if (!bloqueado) {
        console.error("Registro de lista negra no encontrado");
        return;
      }

      const result = await submitUnblockListaNegra(
        bloqueado.id,
        data.motivoDesbloqueo,
        data.observaciones,
      );

      if (result.ok) {
        // Actualizar lista de bloqueados
        await loadBlockedContratistas();
        // Recargar contratistas para actualizar estado
        await loadContratistas();
      } else {
        console.error("Error al desbloquear:", result.error);
        // TODO: Mostrar notificación de error
      }
    } catch (error) {
      console.error("Error en handleUnblock:", error);
    }
  }

  onMount(() => {
    loadContratistas();
  });
</script>

<ContratistaListForm
  {contratistas}
  {loading}
  {error}
  {blockedContratistas}
  onRefresh={loadContratistas}
  onBlock={handleBlock}
  onUnblock={handleUnblock}
/>
