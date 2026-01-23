<script lang="ts">
	import { Grid, Toolbar, WillowDark } from '@svar-ui/svelte-grid';
	import { Editor } from '@svar-ui/svelte-editor';
	import type { IColumnConfig } from '@svar-ui/svelte-grid';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';

	import GridActionsCell from './GridActionsCell.svelte';

	interface Props {
		pendientes: PreRegistroVisita[];
		onIngreso: (p: PreRegistroVisita) => void;
		onCancel: (id: string) => void;
	}

	let { pendientes, onIngreso, onCancel }: Props = $props();

	let selectedValues = $state<any>(null);
	let showEditor = $state(false);

	const columns: IColumnConfig[] = [
		{ id: 'acciones', header: 'Acciones', width: 90, cell: GridActionsCell },
		{ id: 'nombre', header: 'Nombre', width: 130, editor: 'text' },
		{ id: 'apellido', header: 'Apellido', width: 130, editor: 'text' },
		{ id: 'cedula', header: 'Cédula', width: 110, editor: 'text' },
		{ id: 'empresa_nombre', header: 'Empresa', width: 160, editor: 'text' },
		{ id: 'anfitrion', header: 'Anfitrión', width: 130, editor: 'text' },
		{ id: 'area_visitada', header: 'Área', width: 130, editor: 'text' },
		{ id: 'fecha_esperada', header: 'Fecha', width: 110, editor: 'text' },
		{ id: 'hora_esperada', header: 'Hora', width: 90, editor: 'text' }
	];

	const schema = [
		{
			id: 'main',
			label: 'Datos del Pre-Registro',
			items: [
				{ key: 'nombre', label: 'Nombre', type: 'text' },
				{ key: 'apellido', label: 'Apellido', type: 'text' },
				{ key: 'cedula', label: 'Cédula', type: 'text' },
				{ key: 'empresa_nombre', label: 'Empresa', type: 'text' },
				{ key: 'anfitrion', label: 'Anfitrión', type: 'text' },
				{ key: 'area_visitada', label: 'Área', type: 'text' },
				{ key: 'fecha_esperada', label: 'Fecha', type: 'text' },
				{ key: 'hora_esperada', label: 'Hora', type: 'text' }
			]
		}
	];
</script>

<div class="h-full w-full bg-surface-1 flex flex-col">
	<WillowDark>
		<div class="flex-1 flex min-h-0 relative">
			<div class="flex-1 flex flex-col">
				<Toolbar />
				<Grid
					data={pendientes}
					{columns}
					select={true}
					onrowdblclick={(ev) => {
						selectedValues = ev.row;
						showEditor = true;
					}}
					onaction={(ev) => {
						if (ev.action === 'ingreso') onIngreso(ev.data);
						if (ev.action === 'cancel') onCancel(ev.data.id);
					}}
				/>
			</div>
			{#if showEditor}
				<Editor
					items={schema}
					values={selectedValues}
					placement="sidebar"
					topBar={{ items: [{ id: 'close', icon: 'wxi-close' }] }}
					onaction={(ev) => {
						if (ev.item.id === 'close') showEditor = false;
					}}
				/>
			{/if}
		</div>
	</WillowDark>
</div>

<style>
	:global(.wx-grid) {
		height: 100%;
		border: none !important;
	}
	:global(.wx-toolbar) {
		background: var(--wx-background-2) !important;
		border-bottom: 1px solid var(--wx-border) !important;
	}
</style>
