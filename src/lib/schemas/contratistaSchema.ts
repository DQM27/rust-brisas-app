import { z } from "zod";
import {
    CEDULA_MIN_LEN,
    CEDULA_MAX_LEN,
    NOMBRE_MAX_LEN
} from "./domainConstants";

// ==========================================
// REGLAS DE DOMINIO (Alineadas con backend domain/common.rs)
// ==========================================
// - Cédula: Solo números y guiones, CEDULA_MIN_LEN-CEDULA_MAX_LEN chars, sin letras
// - Nombre/Apellido: Solo letras (incluye acentos), espacios permitidos, max NOMBRE_MAX_LEN
// - Fecha: Display DD/MM/YYYY, Backend YYYY-MM-DD (transformación en formulario)

// ==========================================
// VALIDACIONES REUTILIZABLES (Alineadas con backend)
// ==========================================

/** 
 * Cédula: Solo números y guiones, sin letras.
 * Backend: validar_cedula_estandar() - solo ascii_digit y '-'
 */
const cedulaSchema = z.string()
    .trim()
    .min(CEDULA_MIN_LEN, `La cédula debe tener al menos ${CEDULA_MIN_LEN} caracteres`)
    .max(CEDULA_MAX_LEN, `La cédula no puede exceder ${CEDULA_MAX_LEN} caracteres`)
    .regex(/^[0-9-]+$/, "La cédula solo puede contener números y guiones (sin letras)")
    .refine(val => /\d/.test(val), "La cédula debe contener al menos un número");

/**
 * Nombre/Apellido: Solo letras (incluye acentos), espacios permitidos.
 * Backend: validar_nombre_estandar() - is_alphabetic() || is_whitespace()
 */
const nombreSchema = z.string()
    .trim()
    .min(1, "El nombre es obligatorio")
    .max(NOMBRE_MAX_LEN, `El nombre no puede exceder ${NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, "El nombre solo puede contener letras (sin números ni símbolos)");

const apellidoSchema = z.string()
    .trim()
    .min(1, "El apellido es obligatorio")
    .max(NOMBRE_MAX_LEN, `El apellido no puede exceder ${NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, "El apellido solo puede contener letras (sin números ni símbolos)");

const nombreOpcionalSchema = z.string()
    .trim()
    .max(NOMBRE_MAX_LEN, `No puede exceder ${NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]*$/, "Solo puede contener letras")
    .optional()
    .or(z.literal(''));

/**
 * Fecha de vencimiento PRAIND - Display: DD/MM/YYYY
 * NOTA: El formulario transforma a YYYY-MM-DD antes de enviar al backend
 */
const fechaPraindSchema = z.string()
    .min(10, "Fecha requerida (DD/MM/YYYY)")
    .refine((val) => {
        const regex = /^\d{2}\/\d{2}\/\d{4}$/;
        if (!regex.test(val)) return false;

        const [day, month, year] = val.split('/').map(Number);
        const date = new Date(year, month - 1, day);
        return date.getDate() === day && date.getMonth() === month - 1 && date.getFullYear() === year;
    }, "Fecha inválida (DD/MM/YYYY)");

// ==========================================
// BASE SCHEMA (Para defaults de Superforms)
// ==========================================

export const contratistaSchemaBase = z.object({
    cedula: z.string().default(''),
    nombre: z.string().default(''),
    segundoNombre: z.string().default(''),
    apellido: z.string().default(''),
    segundoApellido: z.string().default(''),
    empresaId: z.string().default(''),
    fechaVencimientoPraind: z.string().default(''),
});

// ==========================================
// SCHEMA CON VALIDACIONES (Para validators)
// ==========================================

export const contratistaSchema = z.object({
    cedula: cedulaSchema,
    nombre: nombreSchema,
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema,
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().min(1, "Debe seleccionar una empresa"),
    fechaVencimientoPraind: fechaPraindSchema,
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type ContratistaFormData = z.infer<typeof contratistaSchemaBase>;
export type ContratistaFormSchema = typeof contratistaSchema;
