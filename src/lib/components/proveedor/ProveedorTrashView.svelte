<script lang="ts">
  import TrashListView from "$lib/components/trash/TrashListView.svelte";
  import * as proveedorService from "$lib/logic/proveedor/proveedorService";
  import { ProveedorColumns } from "$lib/logic/proveedor/proveedorColumns"; // Updated import

  interface Props {
    onBack: () => void;
  }
  let { onBack }: Props = $props();

  const trashService = {
    getArchived: async () => {
      const res = await proveedorService.getArchivedProveedores();
      if (res.ok) return { ok: true, data: res.data };
      return { ok: false, error: res.error, data: [] };
    },
    restore: async (id: string) => {
      const res = await proveedorService.restoreProveedor(id);
      if (res.ok) return { ok: true };
      return { ok: false, error: res.error };
    },
  };

  const columns = ProveedorColumns.getTrashColumns();
</script>

<TrashListView
  gridId="universal-trash"
  service={trashService}
  columnDefs={columns}
  {onBack}
  entityName="Proveedor"
/>
