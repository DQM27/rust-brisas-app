import type { ColDef, ValueFormatterParams } from '@ag-grid-community/core';
import type { CitaPopulated } from '$lib/types/cita';

// Formateador de Fecha Larga (ej. "Jueves 24 Oct, 10:30 AM")
const dateFormatter = (params: ValueFormatterParams) => {
	if (!params.value) return '';
	const date = new Date(params.value);
	return date.toLocaleDateString('es-ES', {
		weekday: 'short',
		day: 'numeric',
		month: 'short',
		hour: '2-digit',
		minute: '2-digit'
	});
};

export const VISITA_COLUMNS: ColDef<CitaPopulated>[] = [
	{
		colId: 'fecha_cita',
		field: 'fecha_cita',
		headerName: 'Fecha y Hora',
		valueFormatter: dateFormatter,
		minWidth: 180,
		sort: 'asc', // Ordenar por fecha default
		filter: 'agDateColumnFilter'
	},
	{
		colId: 'visitante_nombre_completo',
		field: 'visitante_nombre_completo',
		headerName: 'Visitante',
		flex: 1,
		minWidth: 200,
		filter: 'agTextColumnFilter'
	},
	{
		colId: 'visitante_empresa',
		field: 'visitante_empresa',
		headerName: 'Empresa / Procedencia',
		minWidth: 150,
		valueFormatter: (params: ValueFormatterParams) => params.value || 'Particular',
		filter: 'agTextColumnFilter'
	},
	{
		colId: 'anfitrion',
		field: 'anfitrion',
		headerName: 'Anfitrión',
		minWidth: 150,
		filter: 'agTextColumnFilter'
	},
	{
		colId: 'area_visitada',
		field: 'area_visitada',
		headerName: 'Área',
		minWidth: 120,
		hide: true // Oculto por defecto para limpiar vista
	},
	{
		colId: 'motivo',
		field: 'motivo',
		headerName: 'Motivo',
		minWidth: 150,
		hide: true
	}
	// Estado o Acciones se manejan via cellRenderer o botones
];
