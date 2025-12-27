<script lang="ts">
  import TrashListView from "$lib/components/trash/TrashListView.svelte";
  import * as contratistaService from "$lib/logic/contratista/contratistaService";
  import { ContratistaColumns } from "$lib/logic/contratista/contratistaColumns";

  interface Props {
    onBack: () => void;
  }
  let { onBack }: Props = $props();

  // Adapter for the generic service interface
  const trashService = {
    getArchived: async () => {
      const res = await contratistaService.getArchivedContratistas();
      if (res.ok) return { ok: true, data: res.data };
      return { ok: false, error: res.error, data: [] };
    },
    restore: async (id: string) => {
      const res = await contratistaService.restoreContratista(id);
      if (res.ok) return { ok: true };
      return { ok: false, error: res.error };
    },
  };

  const columns = ContratistaColumns.getTrashColumns();
</script>

<TrashListView
  gridId="contratista-trash"
  service={trashService}
  columnDefs={columns}
  {onBack}
  entityName="Contratista"
/>
