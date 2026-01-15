// src/lib/types/ingreso.types.ts
// Tipos compartidos para el módulo de ingresos

import { z } from 'zod';
import type { ContratistaResponse } from './contratista';
import type { ProveedorResponse } from './proveedor';
import type { VisitanteResponse } from './visitante';

// ==========================================
// ESQUEMAS (VALIDACIÓN)
// ==========================================

export const TipoIngresoEnum = z.enum(['contratista', 'visita', 'proveedor']);
export const TipoAutorizacionEnum = z.enum(['praind', 'correo']);
export const ModoIngresoEnum = z.enum(['caminando', 'vehiculo']);

// Esquema para crear ingreso de contratista
export const CreateIngresoContratistaSchema = z.object({
	contratistaId: z.string().uuid('ID de contratista inválido'),
	vehiculoId: z.string().uuid().optional().nullable(),
	gafeteNumero: z.string().optional().nullable(),
	tipoAutorizacion: z.string().refine((val) => ['praind', 'correo'].includes(val.toLowerCase()), {
		message: 'Tipo de autorización inválido'
	}),
	modoIngreso: z.string().refine((val) => ['caminando', 'vehiculo'].includes(val.toLowerCase()), {
		message: 'Modo de ingreso inválido'
	}),
	observaciones: z.string().max(500).optional().nullable(),
	usuarioIngresoId: z.string().uuid('ID de usuario inválido')
});

// Esquema para crear ingreso de visita
export const CreateIngresoVisitaSchema = z.object({
	cedula: z.string().min(1, 'Cédula es requerida'),
	nombre: z.string().min(1, 'Nombre es requerido'),
	apellido: z.string().min(1, 'Apellido es requerido'),
	anfitrion: z.string().min(1, 'Anfitrión es requerido'),
	areaVisitada: z.string().min(1, 'Área visitada es requerida'),
	motivoVisita: z.string().min(1, 'Motivo de visita es requerido'),
	tipoAutorizacion: z.string().refine((val) => ['praind', 'correo'].includes(val.toLowerCase()), {
		message: 'Tipo de autorización inválido'
	}),
	modoIngreso: z.string().refine((val) => ['caminando', 'vehiculo'].includes(val.toLowerCase()), {
		message: 'Modo de ingreso inválido'
	}),
	vehiculoPlaca: z.string().optional().nullable(),
	gafeteNumero: z.string().optional().nullable(),
	observaciones: z.string().max(500).optional().nullable(),
	usuarioIngresoId: z.string().uuid('ID de usuario inválido')
});

// Esquema para crear ingreso de proveedor
export const CreateIngresoProveedorSchema = z.object({
	cedula: z.string().min(1, 'Cédula es requerida'),
	nombre: z.string().min(1, 'Nombre es requerido'),
	apellido: z.string().min(1, 'Apellido es requerido'),
	empresaId: z.string().uuid('ID de empresa inválido'),
	areaVisitada: z.string().min(1, 'Área visitada es requerida'),
	motivo: z.string().min(1, 'Motivo es requerido'),
	tipoAutorizacion: z.string().refine((val) => ['praind', 'correo'].includes(val.toLowerCase()), {
		message: 'Tipo de autorización inválido'
	}),
	modoIngreso: z.string().refine((val) => ['caminando', 'vehiculo'].includes(val.toLowerCase()), {
		message: 'Modo de ingreso inválido'
	}),
	vehiculoPlaca: z.string().optional().nullable(),
	gafeteNumero: z.string().optional().nullable(),
	observaciones: z.string().max(500).optional().nullable(),
	usuarioIngresoId: z.string().uuid('ID de usuario inválido')
});

// Esquema para registrar salida
export const RegistrarSalidaSchema = z.object({
	ingresoId: z.string().uuid('ID de ingreso inválido'),
	devolvioGafete: z.boolean(),
	usuarioSalidaId: z.string().uuid('ID de usuario inválido'),
	observacionesSalida: z.string().max(500).optional().nullable()
});

// Esquema para resolver alerta
export const ResolverAlertaSchema = z.object({
	alertaId: z.string().uuid(),
	notas: z.string().max(500).optional().nullable()
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type CreateIngresoContratistaInput = z.infer<typeof CreateIngresoContratistaSchema>;
export type CreateIngresoVisitaInput = z.infer<typeof CreateIngresoVisitaSchema>;
export type CreateIngresoProveedorInput = z.infer<typeof CreateIngresoProveedorSchema>;
export type CreateIngresoInput = CreateIngresoContratistaInput | CreateIngresoVisitaInput | CreateIngresoProveedorInput | any; // Fallback for UI form data
export type RegistrarSalidaInput = z.infer<typeof RegistrarSalidaSchema>;
export type ResolverAlertaInput = z.infer<typeof ResolverAlertaSchema>;

// ==========================================
// TIPOS DE RESPUESTA
// ==========================================

export interface IngresoResponse {
	id: string;
	contratistaId?: string;
	cedula: string;
	nombre: string;
	apellido: string;
	nombreCompleto: string;
	empresaNombre: string;
	tipoIngreso: string;
	tipoIngresoDisplay: string;
	tipoAutorizacion: string;
	tipoAutorizacionDisplay: string;
	modoIngreso: string;
	modoIngresoDisplay: string;
	vehiculoId?: string;
	vehiculoPlaca?: string;
	placaTemporal?: string;
	gafeteNumero?: string;
	fechaHoraIngreso: string;
	fechaHoraSalida?: string;
	tiempoPermanenciaMinutos?: number;
	tiempoPermanenciaTexto?: string;
	usuarioIngresoId: string;
	usuarioIngresoNombre: string;
	usuarioSalidaId?: string;
	usuarioSalidaNombre?: string;
	praindVigenteAlIngreso?: boolean;
	estadoContratistaAlIngreso?: string;
	observaciones?: string;
	estaAdentro: boolean;
	tieneGafeteAsignado: boolean;
	createdAt: string;
	updatedAt: string;
}

export interface IngresoListResponse {
	ingresos: IngresoResponse[];
	total: number;
	adentro: number;
	salieron: number;
}

export interface ValidacionIngresoResponse {
	puedeIngresar: boolean;
	motivoRechazo?: string;
	severidadListaNegra?: string;
	alertas: string[];
	contratista?: ContratistaResponse;
	proveedor?: ProveedorResponse;
	visitante?: VisitanteResponse;
	tieneIngresoAbierto: boolean;
	ingresoAbierto?: IngresoResponse;
}

export interface AlertaGafeteResponse {
	id: string;
	personaId?: string;
	cedula: string;
	nombreCompleto: string;
	gafeteNumero: string;
	ingresoId: string;
	fechaReporte: string;
	resuelto: boolean;
	fechaResolucion?: string;
	notas?: string;
	reportadoPor: string;
	reportadoPorNombre: string;
	createdAt: string;
	updatedAt: string;
}

// ==========================================
// TIPOS DE PERMANENCIA Y ALERTAS
// ==========================================

export type EstadoPermanencia = 'normal' | 'alerta_temprana' | 'tiempo_excedido';

export interface AlertaTiempo {
	estado: EstadoPermanencia;
	minutosTranscurridos: number;
	minutosRestantes: number;
	mensaje?: string;
}

export interface IngresoConEstadoResponse extends IngresoResponse {
	alertaTiempo: AlertaTiempo;
}

export interface AlertaTiempoExcedido {
	ingresoId: string;
	cedula: string;
	nombreCompleto: string;
	empresaNombre: string;
	fechaHoraIngreso: string;
	minutosTranscurridos: number;
	minutosExcedidos: number;
	estado: EstadoPermanencia;
}

export interface AlertaListaNegra {
	ingresoId: string;
	cedula: string;
	nombreCompleto: string;
	bloqueado: boolean;
	motivo?: string;
}

export interface ResumenPermanencias {
	totalAdentro: number;
	normal: number;
	alertaTemprana: number;
	tiempoExcedido: number;
	bloqueadosDurantePermanencia: number;
}
