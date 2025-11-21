// src/lib/logic/contratista/registerContratista.ts

import { createContratista } from "$lib/api/contratista";
import type { CreateContratistaInput, ContratistaResponse } from "$lib/types/contratista";

/**
 * Wrapper simple del service createContratista()
 * Devuelve el contratista creado tal como viene del backend (ContratistaResponse)
 */
export async function registerContratista(
  input: CreateContratistaInput
): Promise<ContratistaResponse> {
  return await createContratista({
    cedula: input.cedula,
    nombre: input.nombre,
    apellido: input.apellido,
    empresaId: input.empresaId,
    fechaVencimientoPraind: input.fechaVencimientoPraind
  });
}
