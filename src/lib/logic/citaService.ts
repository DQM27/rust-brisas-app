import { cita } from '$lib/api/cita';
import type {
	Cita,
	CitaPopulated,
	CreateCitaInput,
	CreateVisitanteInput,
	Visitante
} from '$lib/types/cita';

// ============================================
// TYPES
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// LOGIC
// ============================================

export async function createCita(
	citaData: {
		fechaCita: string; // "YYYY-MM-DD HH:MM" or similar
		anfitrion: string;
		areaVisitada: string;
		motivo?: string;
		registradoPor: string;
	},
	visitanteData: {
		existingId?: string; // If selecting existing visitor
		// If creating new:
		cedula?: string;
		nombre?: string;
		apellido?: string;
		segundoNombre?: string;
		segundoApellido?: string;
		empresa?: string;
		hasVehicle?: boolean;
	}
): Promise<ServiceResult<Cita>> {
	try {
		// 1. Validar input básico
		if (!citaData.fechaCita || !citaData.anfitrion) {
			return { ok: false, error: 'Faltan datos de la cita (Fecha, Anfitrión)' };
		}

		// 2. Preparar input Cita
		const inputCita: CreateCitaInput = {
			visitante_id: visitanteData.existingId || '', // Backend handle empty if sending visitor object
			fecha_cita: citaData.fechaCita,
			anfitrion: citaData.anfitrion,
			area_visitada: citaData.areaVisitada || 'General',
			motivo: citaData.motivo || '', // Optional
			registrado_por: citaData.registradoPor
		};

		// 3. Preparar input Visitante (si aplica)
		let inputVisitante: CreateVisitanteInput | undefined;

		if (!visitanteData.existingId) {
			// Validar datos de nuevo visitante
			if (!visitanteData.cedula || !visitanteData.nombre || !visitanteData.apellido) {
				return { ok: false, error: 'Para nuevo visitante se requiere Cédula, Nombre y Apellido.' };
			}

			inputVisitante = {
				cedula: visitanteData.cedula,
				nombre: visitanteData.nombre,
				apellido: visitanteData.apellido,
				segundo_nombre: visitanteData.segundoNombre,
				segundo_apellido: visitanteData.segundoApellido,
				empresa: visitanteData.empresa,
				has_vehicle: !!visitanteData.hasVehicle
			};
		}

		const result = await cita.create(inputCita, inputVisitante);
		return { ok: true, data: result };
	} catch (err: unknown) {
		console.error('Error creando cita:', err);
		return { ok: false, error: String(err) };
	}
}

export async function fetchVisitanteByCedula(
	cedula: string
): Promise<ServiceResult<Visitante | null>> {
	try {
		const result = await cita.getVisitanteByCedula(cedula);
		return { ok: true, data: result };
	} catch (err: unknown) {
		console.error('Error buscando visitante:', err);
		return { ok: false, error: String(err) };
	}
}

export async function fetchCitasHoy(): Promise<ServiceResult<CitaPopulated[]>> {
	try {
		const result = await cita.getHoy();
		return { ok: true, data: result };
	} catch (err: unknown) {
		console.error('Error cargando citas:', err);
		return { ok: false, error: String(err) };
	}
}
