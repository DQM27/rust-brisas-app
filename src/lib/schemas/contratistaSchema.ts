import { z } from "zod";

// ==========================================
// BASE SCHEMA (Para defaults de Superforms)
// ==========================================
// Usamos .default('') para que Superforms pueda inferir los valores iniciales

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
    cedula: z
        .string()
        .min(4, "Mínimo 4 caracteres")
        .max(20, "Máximo 20 caracteres")
        .regex(/^[a-zA-Z0-9-]+$/, "Solo letras, números y guiones"),
    nombre: z.string().min(2, "Mínimo 2 caracteres"),
    segundoNombre: z.string().optional(),
    apellido: z.string().min(2, "Mínimo 2 caracteres"),
    segundoApellido: z.string().optional(),
    empresaId: z.string().min(1, "Debe seleccionar una empresa"),
    fechaVencimientoPraind: z
        .string()
        .min(10, "Fecha inválida (DD/MM/YYYY)")
        .refine((val) => {
            // Validación básica de fecha DD/MM/YYYY
            const regex = /^\d{2}\/\d{2}\/\d{4}$/;
            if (!regex.test(val)) return false;

            const [day, month, year] = val.split('/').map(Number);
            const date = new Date(year, month - 1, day);
            return date.getDate() === day && date.getMonth() === month - 1 && date.getFullYear() === year;
        }, "Fecha inválida"),
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type ContratistaFormData = z.infer<typeof contratistaSchemaBase>;
export type ContratistaFormSchema = typeof contratistaSchema;
