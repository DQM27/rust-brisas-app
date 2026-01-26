<script lang="ts">
	import IngresoProveedorListView from './IngresoProveedorListView.svelte';

	interface Props {
		data?: any;
	}
	let { data }: Props = $props();
</script>

<IngresoProveedorListView tabId="proveedor-ingreso-list" {data} />
