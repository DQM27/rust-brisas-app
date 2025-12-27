<script lang="ts">
  import TrashView from "$lib/components/trash/TrashView.svelte";
  import * as visitanteService from "$lib/logic/visitante/visitanteService";
  import { VisitanteColumns } from "$lib/logic/visitante/visitanteColumns";

  interface Props {
    onBack: () => void;
  }
  let { onBack }: Props = $props();

  const trashService = {
    getArchived: async () => {
      const res = await visitanteService.getArchivedVisitantes();
      if (res.ok) return { ok: true, data: res.data };
      return { ok: false, error: res.error, data: [] };
    },
    restore: async (id: string) => {
      const res = await visitanteService.restoreVisitante(id);
      if (res.ok) return { ok: true };
      return { ok: false, error: res.error };
    },
  };

  const columns = VisitanteColumns.getTrashColumns();
</script>

<TrashView
  gridId="universal-trash"
  service={trashService}
  columnDefs={columns}
  {onBack}
  entityName="Visitante"
/>
