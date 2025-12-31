import { z } from "zod";

// ==========================================
// BASE SCHEMA (Para defaults de Superforms)
// ==========================================

export const vehiculoSchemaBase = z.object({
    tipoVehiculo: z.string().default(''),
    placa: z.string().default(''),
    marca: z.string().default(''),
    modelo: z.string().default(''),
    color: z.string().default(''),
});

// ==========================================
// SCHEMA CON VALIDACIONES
// ==========================================

export const vehiculoSchema = z.object({
    tipoVehiculo: z.enum(["motocicleta", "automovil", "camioneta", "camion", "otro"]),
    placa: z
        .string()
        .min(3, "Mínimo 3 caracteres")
        .max(15, "Máximo 15 caracteres")
        .regex(/^[A-Za-z0-9\s-]+$/, "Solo letras, números y guiones")
        .transform((val) => val.toUpperCase().trim()),
    marca: z.string().max(50, "Máximo 50 caracteres").optional().or(z.literal('')),
    modelo: z.string().max(50, "Máximo 50 caracteres").optional().or(z.literal('')),
    color: z.string().max(30, "Máximo 30 caracteres").optional().or(z.literal('')),
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type VehiculoFormData = z.infer<typeof vehiculoSchemaBase>;
export type VehiculoSchema = typeof vehiculoSchema;
