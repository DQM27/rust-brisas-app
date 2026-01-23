<script lang="ts">
	import { Grid, Toolbar, WillowDark } from '@svar-ui/svelte-grid';
	import { Editor } from '@svar-ui/svelte-editor';
	import type { IColumnConfig } from '@svar-ui/svelte-grid';
	import { defaultToolbarButtons } from '@svar-ui/grid-store';
	import { setContext } from 'svelte';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';

	import GridActionsCell from './GridActionsCell.svelte';

	interface Props {
		pendientes: PreRegistroVisita[];
		onIngreso: (p: PreRegistroVisita) => void;
		onCancel: (id: string) => void;
	}

	let { pendientes, onIngreso, onCancel }: Props = $props();

	setContext('gridActions', {
		ingreso: (row: PreRegistroVisita) => onIngreso(row),
		cancel: (row: PreRegistroVisita) => onCancel(row.id) // Ensure we pass generic row or id handled properly
	});

	// Normalize data to ensure IDs are strings (handling SurrealDB object IDs)
	let gridData = $derived(
		pendientes.map((p) => {
			let id = p.id;
			// Handle potential object ID from SurrealDB
			if (typeof p.id === 'object' && p.id !== null) {
				// @ts-ignore - Runtime check for specific structure seen in logs
				if (p.id.tb && p.id.id && p.id.id.String) {
					// @ts-ignore
					id = `${p.id.tb}:${p.id.id.String}`;
				} else {
					// Fallback to string if structure is different but still object
					id = (p.id as any).toString();
				}
			}

			return {
				...p,
				id: id,
				_original: p // Keep original for reference if needed
			};
		})
	);

	let selectedValues = $state<any>(null);
	let showEditor = $state(false);

	const columns: IColumnConfig[] = [
		{
			id: 'nombre_completo',
			header: 'Nombre Completo',
			width: 200,
			template: (v: any, r: any) => {
				const row = r as PreRegistroVisita;
				return `${row.nombre} ${row.apellido}`;
			}
		},
		{ id: 'cedula', header: 'Cédula', width: 110, editor: 'text' },
		{ id: 'empresa_nombre', header: 'Empresa', width: 160, editor: 'text' },
		{ id: 'anfitrion', header: 'Anfitrión', width: 130, editor: 'text' },
		{ id: 'area_visitada', header: 'Área', width: 130, editor: 'text' },
		{ id: 'fecha_esperada', header: 'Fecha', width: 110, editor: 'text' },
		{ id: 'hora_esperada', header: 'Hora', width: 90, editor: 'text' },
		{ id: 'acciones', header: 'Acciones', width: 90, cell: GridActionsCell }
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
				<Toolbar
					items={defaultToolbarButtons.filter((b) => b.id === 'search' || b.id === 'export')}
				/>
				<!-- Native Event Capture Wrapper -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="w-full h-full"
					ondblclick={(e) => {
						// Custom implementation to bypass grid event issues
						const target = e.target as HTMLElement;
						const rowEl = target.closest('[data-id]');
						if (rowEl) {
							const id = rowEl.getAttribute('data-id');
							const row = pendientes.find((p) => p.id === id);
							if (row) {
								console.log('Native capture: Row found', row);
								selectedValues = row;
								showEditor = true;
							}
						}
					}}
				>
					<Grid
						data={pendientes}
						{columns}
						select={true}
						onaction={(ev) => {
							if (ev.action === 'ingreso') onIngreso(ev.data);
							if (ev.action === 'cancel') onCancel(ev.data.id);
						}}
					/>
				</div>
			</div>
			{#if showEditor}
				<Editor
					items={schema}
					values={selectedValues}
					placement="sidebar"
					topBar={{
						items: [
							{ id: 'save', text: 'Salvar', css: 'wx-button--primary' },
							{ id: 'delete', text: 'Borrar', css: 'wx-button--danger' },
							{ id: 'close', icon: 'wxi-close' }
						]
					}}
					onaction={(ev) => {
						if (ev.item.id === 'close') showEditor = false;
						if (ev.item.id === 'delete') {
							// Call the cancel handler passed via props/context
							// Note: reusing the onCancel prop which takes an ID
							if (selectedValues && selectedValues.id) {
								onCancel(selectedValues.id);
								showEditor = false;
							}
						}
						if (ev.item.id === 'save') {
							console.log('Save clicked - Backend functionality pending', selectedValues);
							// Here we would call the update service
						}
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
