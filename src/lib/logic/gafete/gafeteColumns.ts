import type { GafeteResponse } from '$lib/types/gafete';
import type { ColDef } from '@ag-grid-community/core';

export class GafeteColumns {
	static getColumns(handlers: {
		onResolve: (data: GafeteResponse) => void;
		onRecover: (data: GafeteResponse) => void;
		onLost: (data: GafeteResponse) => void;
		onDamage: (data: GafeteResponse) => void;
		onDelete: (data: GafeteResponse) => void;
		onEdit: (data: GafeteResponse) => void;
	}): ColDef<GafeteResponse>[] {
		return [
			{
				colId: 'numero',
				field: 'numero',
				headerName: 'Número',
				sortable: true,
				filter: true,
				cellStyle: { fontWeight: 'bold' },
				width: 120
			},
			{
				colId: 'tipoDisplay',
				field: 'tipoDisplay',
				headerName: 'Tipo',
				sortable: true,
				filter: true,
				width: 130,
				cellRenderer: (params: any) => {
					const tipo = params.data.tipo;
					const baseClass =
						'inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';
					let colorClass = '';

					switch (tipo) {
						case 'contratista':
							colorClass =
								'bg-indigo-500/10 text-indigo-600 border-indigo-500/20 dark:bg-indigo-500/10 dark:text-indigo-400 dark:border-indigo-500/20';
							break;
						case 'proveedor':
							colorClass =
								'bg-amber-500/10 text-amber-600 border-amber-500/20 dark:bg-amber-500/10 dark:text-amber-400 dark:border-amber-500/20';
							break;
						case 'visita':
							colorClass =
								'bg-violet-500/10 text-violet-600 border-violet-500/20 dark:bg-violet-500/10 dark:text-violet-400 dark:border-violet-500/20';
							break;
						default:
							colorClass =
								'bg-gray-500/10 text-gray-600 border-gray-500/20 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-700 shadow-none';
					}

					return `<span class="${baseClass} ${colorClass}">${params.value}</span>`;
				}
			},
			{
				colId: 'status',
				field: 'status',
				headerName: 'Estado',
				sortable: true,
				filter: true,
				width: 150,
				cellRenderer: (params: any) => {
					const status = params.value;
					const baseClass =
						'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';

					let classes = '';
					let icon = '';
					let label = '';

					switch (status) {
						case 'disponible':
						case 'activo':
							classes =
								'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20';
							icon = `<span class="mr-1.5 text-emerald-500">✔</span>`;
							label = 'Disponible';
							break;
						case 'en_uso':
							classes =
								'bg-blue-500/10 text-blue-600 border-blue-500/20 dark:bg-blue-500/10 dark:text-blue-400 dark:border-blue-500/20';
							icon = `<span class="mr-1.5 text-blue-500">◉</span>`;
							label = 'En Uso';
							break;
						case 'perdido':
							classes =
								'bg-red-500/10 text-red-600 border-red-500/20 dark:bg-red-500/10 dark:text-red-400 dark:border-red-500/20';
							icon = `<span class="mr-1.5 text-red-500">⚠</span>`;
							label = 'Perdido';
							break;
						case 'danado':
							classes =
								'bg-rose-500/10 text-rose-600 border-rose-500/20 dark:bg-rose-500/10 dark:text-rose-400 dark:border-rose-700/30';
							icon = `<span class="mr-1.5 text-rose-500">⚡</span>`;
							label = 'Dañado';
							break;
						case 'extraviado':
							classes =
								'bg-amber-500/10 text-amber-600 border-amber-500/20 dark:bg-amber-500/10 dark:text-amber-400 dark:border-amber-500/20';
							icon = `<span class="mr-1.5 text-amber-500">❓</span>`;
							label = 'Extraviado';
							break;
						default:
							classes =
								'bg-gray-500/10 text-gray-600 border-gray-500/20 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600';
							icon = `<span class="mr-1.5 opacity-75">-</span>`;
							label = status;
					}

					return `<span class="${baseClass} ${classes}">${icon} ${label}</span>`;
				}
			},
			// Column removed as per user request
			/*
            {
                colId: "asignadoA",
                field: "asignadoA",
                headerName: "Asignado A",
                sortable: true,
                filter: true,
                width: 180,
                valueFormatter: (params: any) => params.value || "-",
            },
            */
			{
				colId: 'fechaPerdido',
				field: 'fechaPerdido',
				headerName: 'Fecha Reporte',
				sortable: true,
				filter: true,
				width: 160,
				valueFormatter: (params: any) => {
					if (!params.value) return '-';
					const date = new Date(params.value);
					if (isNaN(date.getTime())) return '-';
					// DD/MM/YYYY format
					const day = date.getDate().toString().padStart(2, '0');
					const month = (date.getMonth() + 1).toString().padStart(2, '0');
					const year = date.getFullYear();
					return `${day}/${month}/${year}`;
				}
			},
			{
				colId: 'quienPerdio',
				field: 'quienPerdio',
				headerName: 'Persona que Perdió',
				sortable: true,
				filter: true,
				width: 180,
				valueFormatter: (params: any) => params.value || '-'
			},
			{
				colId: 'reportadoPorNombre',
				field: 'reportadoPorNombre',
				headerName: 'Reportado Por',
				sortable: true,
				filter: true,
				width: 160,
				valueFormatter: (params: any) => params.value || '-'
			},
			{
				colId: 'resueltoPorNombre',
				field: 'resueltoPorNombre',
				headerName: 'Resuelto Por',
				sortable: true,
				filter: true,
				width: 150,
				valueFormatter: (params: any) => params.value || '-'
			},
			{
				colId: 'fechaResolucion',
				field: 'fechaResolucion',
				headerName: 'Fecha Resolución',
				sortable: true,
				filter: true,
				width: 160,
				valueFormatter: (params: any) => {
					if (!params.value) return '-';
					return new Date(params.value).toLocaleDateString();
				}
			},
			{
				colId: 'notas',
				field: 'notas',
				headerName: 'Notas',
				sortable: true,
				filter: true,
				width: 200,
				valueFormatter: (params: any) => params.value || '-'
			},
			{
				colId: 'acciones',
				headerName: 'Acciones',
				width: 220,
				pinned: 'right',
				cellRenderer: (params: any) => {
					const status = params.data.status;
					let buttons = '';

					if (status === 'perdido') {
						buttons += `
              <button class="mr-2 px-2 py-1 bg-green-100 text-green-700 rounded hover:bg-green-200 text-xs font-medium resolve-btn">
                ✓ Resolver
              </button>
            `;
					}

					if (status !== 'perdido') {
						// Botón Editar
						buttons += `
                <button class="mr-2 px-2 py-1 bg-gray-100 text-gray-700 rounded hover:bg-gray-200 text-xs font-medium edit-btn" title="Editar">
                  ✏️
                </button>
              `;

						if (status === 'extraviado') {
							buttons += `
                <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn">
                  ↻ Recuperar
                </button>
              `;
						} else if (status !== 'danado') {
							buttons += `
                <button class="mr-2 px-2 py-1 bg-amber-100 text-amber-700 rounded hover:bg-amber-200 text-xs font-medium lost-btn" title="Marcar como Extraviado">
                  ❓ Ext
                </button>
                <button class="mr-2 px-2 py-1 bg-rose-100 text-rose-700 rounded hover:bg-rose-200 text-xs font-medium damage-btn" title="Marcar como Dañado">
                  ⚡ Dañ
                </button>
              `;
						}

						if (status === 'danado') {
							buttons += `
                <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn" title="Marcar como Reparado/Activo">
                  ↻ Rep
                </button>
              `;
						}

						if (status === 'danado' || status === 'disponible') {
							buttons += `
                <button class="px-2 py-1 bg-gray-100 text-gray-700 rounded hover:bg-red-100 hover:text-red-700 text-xs font-medium delete-btn" title="Eliminar">
                  🗑️
                </button>
              `;
						}
					}

					return buttons || `<span class="text-xs text-gray-400">-</span>`;
				},
				onCellClicked: (params: any) => {
					const event = params.event;
					const data = params.data;
					const target = event.target as HTMLElement;

					// Prevent event bubbling that could cause double-firing
					event.stopPropagation();

					if (target.classList.contains('resolve-btn')) {
						handlers.onResolve(data);
						return;
					}
					if (target.classList.contains('edit-btn')) {
						handlers.onEdit(data);
						return;
					}
					if (target.classList.contains('recover-btn')) {
						handlers.onRecover(data);
						return;
					}
					if (target.classList.contains('lost-btn')) {
						handlers.onLost(data);
						return;
					}
					if (target.classList.contains('damage-btn')) {
						handlers.onDamage(data);
						return;
					}
					if (target.classList.contains('delete-btn')) {
						handlers.onDelete(data);
						return;
					}
				}
			}
		];
	}
}
