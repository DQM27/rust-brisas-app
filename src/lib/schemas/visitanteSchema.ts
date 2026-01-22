import { z } from 'zod';
import {
    CEDULA_MIN_LEN,
    CEDULA_MAX_LEN,
    NOMBRE_MAX_LEN,
    SEGUNDO_NOMBRE_MAX_LEN,
    PLACA_MIN_LEN,
    PLACA_MAX_LEN,
    MARCA_MODELO_MAX_LEN,
    COLOR_MAX_LEN
} from './domainConstants';

// ==========================================
// REGLAS DE DOMINIO (Alineadas con backend domain/common.rs)
// ==========================================
// - Cédula: Solo números y guiones, CEDULA_MIN_LEN-CEDULA_MAX_LEN dígitos, sin letras
// - Nombre/Apellido: Solo letras (incluye acentos), espacios permitidos, max NOMBRE_MAX_LEN
// - Empresa: Obligatoria (backend rechaza visitantes sin empresa)
// - Vehículo: Si hasVehicle=true, placa y tipo son obligatorios

// ==========================================
// VALIDACIONES REUTILIZABLES (Alineadas con backend)
// ==========================================

/**
 * Cédula: Solo números y guiones, sin letras.
 * Backend: validar_cedula_estandar() - solo ascii_digit y '-'
 */
const cedulaSchema = z
    .string()
    .trim()
    .min(1, 'Cédula requerida')
    .pipe(
        z
            .string()
            .min(CEDULA_MIN_LEN, `Mínimo ${CEDULA_MIN_LEN} caracteres`)
            .max(CEDULA_MAX_LEN, `Máximo ${CEDULA_MAX_LEN} caracteres`)
            .regex(/^[0-9-]+$/, 'Solo números y guiones')
            .refine((val) => /\d/.test(val), 'Debe contener números')
    );

/**
 * Nombre/Apellido: Solo letras (incluye acentos), espacios y puntos permitidos.
 * Backend: validar_nombre_estandar() - is_alphabetic() || is_whitespace() || '.'
 */
const nombreSchema = z
    .string()
    .trim()
    .min(1, 'Nombre requerido')
    .pipe(
        z
            .string()
            .max(NOMBRE_MAX_LEN, `Máximo ${NOMBRE_MAX_LEN} caracteres`)
            .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s.]+$/, 'Solo letras')
    );

const apellidoSchema = z
    .string()
    .trim()
    .min(1, 'Apellido requerido')
    .pipe(
        z
            .string()
            .max(NOMBRE_MAX_LEN, `Máximo ${NOMBRE_MAX_LEN} caracteres`)
            .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s.]+$/, 'Solo letras')
    );

const nombreOpcionalSchema = z
    .string()
    .trim()
    .max(SEGUNDO_NOMBRE_MAX_LEN, `No puede exceder ${SEGUNDO_NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s.]*$/, 'Solo puede contener letras')
    .optional()
    .or(z.literal(''));

/**
 * Placa de vehículo: Alfanumérico, guiones y espacios.
 * Backend: validar_placa_estandar() - alphanumeric || '-' || ' '
 */
const placaSchema = z
    .string()
    .trim()
    .max(PLACA_MAX_LEN, `Máximo ${PLACA_MAX_LEN} caracteres`)
    .regex(/^[a-zA-Z0-9\-\s]*$/, 'Solo letras, números, guiones y espacios')
    .optional()
    .or(z.literal(''));

const placaRequeridaSchema = z
    .string()
    .trim()
    .min(PLACA_MIN_LEN, `Mínimo ${PLACA_MIN_LEN} caracteres`)
    .max(PLACA_MAX_LEN, `Máximo ${PLACA_MAX_LEN} caracteres`)
    .regex(/^[a-zA-Z0-9\-\s]+$/, 'Solo letras, números, guiones y espacios');

const tipoVehiculoSchema = z.enum(['motocicleta', 'automovil', 'camioneta', 'camion', 'otro'], {
    message: 'Seleccione tipo de vehículo'
});

const marcaModeloOpcionalSchema = z
    .string()
    .trim()
    .max(MARCA_MODELO_MAX_LEN, `Máximo ${MARCA_MODELO_MAX_LEN} caracteres`)
    .optional()
    .or(z.literal(''));

const colorOpcionalSchema = z
    .string()
    .trim()
    .max(COLOR_MAX_LEN, `Máximo ${COLOR_MAX_LEN} caracteres`)
    .optional()
    .or(z.literal(''));

// ==========================================
// BASE SCHEMA (Para defaults de Superforms)
// ==========================================

export const visitanteSchemaBase = z.object({
    cedula: z.string().default(''),
    nombre: z.string().default(''),
    segundoNombre: z.string().default(''),
    apellido: z.string().default(''),
    segundoApellido: z.string().default(''),
    empresaId: z.string().default(''),
    hasVehicle: z.boolean().default(false),
    tipoVehiculo: z.string().default(''),
    placa: z.string().default(''),
    marca: z.string().default(''),
    modelo: z.string().default(''),
    color: z.string().default('')
});

// ==========================================
// SCHEMA CON VALIDACIONES (Para validators)
// ==========================================

export const visitanteSchema = z
    .object({
        cedula: cedulaSchema,
        nombre: nombreSchema,
        segundoNombre: nombreOpcionalSchema,
        apellido: apellidoSchema,
        segundoApellido: nombreOpcionalSchema,
        empresaId: z.string().min(1, 'Empresa requerida'),
        hasVehicle: z.boolean(),
        tipoVehiculo: z.string().optional().or(z.literal('')),
        placa: placaSchema,
        marca: marcaModeloOpcionalSchema,
        modelo: marcaModeloOpcionalSchema,
        color: colorOpcionalSchema
    })
    .superRefine((data, ctx) => {
        // Validación condicional: Si tiene vehículo, placa y tipo son obligatorios
        if (data.hasVehicle) {
            if (!data.tipoVehiculo || data.tipoVehiculo === '') {
                ctx.addIssue({
                    code: z.ZodIssueCode.custom,
                    message: 'Seleccione tipo de vehículo',
                    path: ['tipoVehiculo']
                });
            }
            if (!data.placa || data.placa.trim().length < PLACA_MIN_LEN) {
                ctx.addIssue({
                    code: z.ZodIssueCode.custom,
                    message: `Placa requerida (mín. ${PLACA_MIN_LEN} caracteres)`,
                    path: ['placa']
                });
            }
        }
    });

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type VisitanteFormData = z.infer<typeof visitanteSchemaBase>;
export type VisitanteFormSchema = typeof visitanteSchema;
