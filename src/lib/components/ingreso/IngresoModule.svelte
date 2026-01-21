<script lang="ts">
	import IngresoListView from './IngresoListView.svelte';

	interface Props {
		data?: any;
	}
	let { data }: Props = $props();
</script>

<IngresoListView tabId="ingreso-list" {data} />



