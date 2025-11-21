// src/lib/logic/contratista/submitRegisterContratista.ts

import { validateContratistaInput } from './validateContratistaInput';
import { registerContratista } from './registerContratista';
import { parseContratistaError } from './parseContratistaErrors';

import type {
  ContratistaResponse,
  CreateContratistaInput
} from '$lib/types/contratista';

export type SubmitRegisterContratistaResult =
  | { ok: true; contratista: ContratistaResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de registrar un contratista:
 * 1. Valida input
 * 2. Llama al service
 * 3. Parsea errores si falla
 */
export async function submitRegisterContratista(
  input: CreateContratistaInput
): Promise<SubmitRegisterContratistaResult> {

  const { nombre, apellido, cedula, empresaId, fechaVencimientoPraind } = input;

  // 1. Validar
  const validation = validateContratistaInput(
    nombre,
    apellido,
    cedula,
    empresaId,
    fechaVencimientoPraind
  );

  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar crear
  try {
    const contratista = await registerContratista(input);
    return { ok: true, contratista };
  } catch (err: any) {
    // 3. Parsear error
    const msg = parseContratistaError(err);
    return { ok: false, error: msg };
  }
}
