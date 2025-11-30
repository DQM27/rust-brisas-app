import { z } from 'zod';

// ==========================================
// ESQUEMAS (VALIDACIÓN)
// ==========================================

export const TipoIngresoEnum = z.enum(['contratista']);
export const TipoAutorizacionEnum = z.enum(['praind', 'correo']);
export const ModoIngresoEnum = z.enum(['caminando', 'vehiculo']);

// Esquema para crear ingreso de contratista (CreateIngresoContratistaInput)
export const CreateIngresoContratistaSchema = z.object({
    contratistaId: z.string().uuid("ID de contratista inválido"),
    vehiculoId: z.string().uuid().optional().nullable(),
    gafeteNumero: z.string().optional().nullable(),
    tipoAutorizacion: z.string().refine((val) => ['praind', 'correo'].includes(val.toLowerCase()), {
        message: "Tipo de autorización inválido"
    }),
    modoIngreso: z.string().refine((val) => ['caminando', 'vehiculo'].includes(val.toLowerCase()), {
        message: "Modo de ingreso inválido"
    }),
    observaciones: z.string().max(500).optional().nullable(),
    usuarioIngresoId: z.string().uuid("ID de usuario inválido"),
});

// Esquema para registrar salida (RegistrarSalidaInput)
export const RegistrarSalidaSchema = z.object({
    ingresoId: z.string().uuid("ID de ingreso inválido"),
    devolvioGafete: z.boolean(),
    usuarioSalidaId: z.string().uuid("ID de usuario inválido"),
    observacionesSalida: z.string().max(500).optional().nullable(),
});

// Esquema para resolver alerta (ResolverAlertaInput)
export const ResolverAlertaSchema = z.object({
    alertaId: z.string().uuid(),
    notas: z.string().max(500).optional().nullable(),
});

// ==========================================
// TIPOS INFERIDOS (TYPESCRIPT)
// ==========================================

export type CreateIngresoContratistaInput = z.infer<typeof CreateIngresoContratistaSchema>;
export type RegistrarSalidaInput = z.infer<typeof RegistrarSalidaSchema>;
export type ResolverAlertaInput = z.infer<typeof ResolverAlertaSchema>;

export interface IngresoResponse {
    id: string;
    contratistaId?: string;
    cedula: string;
    nombre: string;
    apellido: string;
    nombreCompleto: string;
    empresaNombre: string;
    tipoIngreso: string; // "contratista"
    tipoIngresoDisplay: string;
    tipoAutorizacion: string; // "praind" | "correo"
    tipoAutorizacionDisplay: string;
    modoIngreso: string; // "caminando" | "vehiculo"
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
    alertas: string[];
    contratista?: any; // JSON Value
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
