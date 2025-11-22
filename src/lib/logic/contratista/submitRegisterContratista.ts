import { validateContratistaInput } from './validateContratistaInput';
import { registerContratista } from './registerContratista';
import { parseContratistaError } from './parseContratistaErrors';

import type {
  ContratistaResponse,
  CreateContratistaInput
} from '$lib/types/contratista';

/**
 * Define el tipo de resultado de la función submitRegisterContratista.
 * Puede ser exitoso (ok: true) con el objeto del contratista,
 * o fallido (ok: false) con un mensaje de error.
 */
export type SubmitRegisterContratistaResult =
  | { ok: true; contratista: ContratistaResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de registrar un contratista:
 * 1. Valida input (validateContratistaInput)
 * 2. Llama al servicio de registro (registerContratista)
 * 3. Parsea errores si falla (parseContratistaError)
 * * @param input - Objeto con los datos del nuevo contratista.
 * @returns Promesa que resuelve con SubmitRegisterContratistaResult.
 */
export async function submitRegisterContratista(
  input: CreateContratistaInput
): Promise<SubmitRegisterContratistaResult> {

  const { nombre, apellido, cedula, empresaId, fechaVencimientoPraind } = input;

  // 1. Validar campos requeridos
  const validation = validateContratistaInput(
    nombre,
    apellido,
    cedula,
    empresaId,
    fechaVencimientoPraind
  );

  if (!validation.ok) {
    // Retorna el error de validación si la entrada es inválida
    return { ok: false, error: validation.message };
  }

  // 2. Intentar crear el contratista
  try {
    // Asume que registerContratista maneja la lógica de la API/Base de Datos
    const contratista = await registerContratista(input);
    // Éxito
    return { ok: true, contratista };
  } catch (err: any) {
    // 3. Parsea el error para devolver un mensaje amigable al usuario
    const msg = parseContratistaError(err);
    return { ok: false, error: msg };
  }
}