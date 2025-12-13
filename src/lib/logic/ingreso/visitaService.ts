// ==========================================
// src/lib/logic/ingreso/visitaService.ts
// ==========================================
// Lógica de negocio para ingresos de VISITAS

import { ingreso } from '$lib/api/ingreso';
import type {
    CreateIngresoVisitaInput,
    IngresoResponse
} from '$lib/types/ingreso';

// ==========================================
// TIPOS INTERNOS
// ==========================================

export interface VisitaFormData {
    cedula: string;
    nombre: string;
    apellido: string;
    empresa?: string;
    anfitrion: string;
    areaVisitada: string;
    motivoVisita: string;
    tipoAutorizacion: 'praind' | 'correo';
    modoIngreso: 'caminando' | 'vehiculo';
    vehiculoPlaca?: string;
    gafeteNumero?: string;
    observaciones?: string;
}

export interface ValidacionVisitaResult {
    isValid: boolean;
    errors: Record<string, string>;
}

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

/**
 * Valida los datos del formulario de visita
 */
export function validarDatosVisita(data: VisitaFormData): ValidacionVisitaResult {
    const errors: Record<string, string> = {};

    // Validar cédula
    if (!data.cedula?.trim()) {
        errors.cedula = 'Cédula es requerida';
    }

    // Validar nombre
    if (!data.nombre?.trim()) {
        errors.nombre = 'Nombre es requerido';
    }

    // Validar apellido
    if (!data.apellido?.trim()) {
        errors.apellido = 'Apellido es requerido';
    }

    // Validar anfitrión
    if (!data.anfitrion?.trim()) {
        errors.anfitrion = 'Anfitrión es requerido';
    }

    // Validar área visitada
    if (!data.areaVisitada?.trim()) {
        errors.areaVisitada = 'Área visitada es requerida';
    }

    // Validar motivo de visita
    if (!data.motivoVisita?.trim()) {
        errors.motivoVisita = 'Motivo de visita es requerido';
    }

    // Validar modo y vehículo
    if (data.modoIngreso === 'vehiculo' && !data.vehiculoPlaca?.trim()) {
        errors.vehiculoPlaca = 'Placa de vehículo es requerida cuando ingresa en vehículo';
    }

    return {
        isValid: Object.keys(errors).length === 0,
        errors
    };
}

/**
 * Normaliza los datos antes de enviar al backend
 */
export function normalizarDatosVisita(data: VisitaFormData): VisitaFormData {
    return {
        ...data,
        cedula: data.cedula.trim(),
        nombre: data.nombre.trim(),
        apellido: data.apellido.trim(),
        anfitrion: data.anfitrion.trim(),
        areaVisitada: data.areaVisitada.trim(),
        motivoVisita: data.motivoVisita.trim(),
        vehiculoPlaca: data.vehiculoPlaca?.trim() || undefined,
        gafeteNumero: data.gafeteNumero?.trim().toUpperCase() || undefined,
        observaciones: data.observaciones?.trim() || undefined,
    };
}

// ==========================================
// OPERACIONES PRINCIPALES
// ==========================================

/**
 * Crea un ingreso de visita
 */
export async function crearIngresoVisita(
    data: VisitaFormData,
    usuarioId: string
): Promise<IngresoResponse> {
    // 1. Validar datos
    const validacion = validarDatosVisita(data);
    if (!validacion.isValid) {
        const primerError = Object.values(validacion.errors)[0];
        throw new Error(primerError);
    }

    // 2. Normalizar datos
    const datosNormalizados = normalizarDatosVisita(data);

    // 3. Preparar input para el backend
    const input: CreateIngresoVisitaInput = {
        cedula: datosNormalizados.cedula,
        nombre: datosNormalizados.nombre,
        apellido: datosNormalizados.apellido,
        anfitrion: datosNormalizados.anfitrion,
        areaVisitada: datosNormalizados.areaVisitada,
        motivoVisita: datosNormalizados.motivoVisita,
        tipoAutorizacion: datosNormalizados.tipoAutorizacion,
        modoIngreso: datosNormalizados.modoIngreso,
        vehiculoPlaca: datosNormalizados.vehiculoPlaca ?? null,
        gafeteNumero: datosNormalizados.gafeteNumero ?? null,
        observaciones: datosNormalizados.observaciones ?? null,
        usuarioIngresoId: usuarioId,
    };

    // 4. Invocar API
    // Usamos el nuevo servicio
    const { ingresoVisitaService } = await import('$lib/services/ingresoVisitaService');

    // Preparar input full
    const fullInput = {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        // empresa: input.empresa, // VisitaFormData no tiene empresa explícita? Revisar interfaz. 
        // Interfaz VisitaFormData (lines 16-28) NO tiene empresa.
        // Pero el backend lo soporta. Default undefined.

        anfitrion: input.anfitrion,
        area_visitada: input.areaVisitada,
        motivo: input.motivoVisita,
        gafete: input.gafeteNumero || undefined,
        observaciones: input.observaciones || undefined,
        usuario_ingreso_id: input.usuarioIngresoId,
        cita_id: undefined
    };

    const result = await ingresoVisitaService.createIngreso(fullInput);
    return result as any; // Cast to any/IngresoResponse shim to satisfy legacy interface for now
}

// ==========================================
// UTILIDADES
// ==========================================

/**
 * Verifica si un modo requiere placa de vehículo
 */
export function requierePlaca(modoIngreso: 'caminando' | 'vehiculo'): boolean {
    return modoIngreso === 'vehiculo';
}

/**
 * Obtiene el label apropiado para el tipo de autorización
 */
export function getLabelTipoAutorizacion(tipo: 'praind' | 'correo'): string {
    return tipo === 'praind' ? 'PRAIND' : 'Correo electrónico';
}
