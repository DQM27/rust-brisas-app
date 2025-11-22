// src/lib/logic/contratista/fetchContratistas.ts

import { listContratistas } from '$lib/api/contratista';
import type { ContratistaResponse } from '$lib/types/contratista';

/**
 * Wrapper simple del service listContratistas()
 * Lanza excepción si falla (no maneja errores)
 */
export async function fetchAllContratistas(): Promise<ContratistaResponse[]> {
  const result = await listContratistas();
  return result.contratistas;
}

/**
 * Obtiene solo contratistas activos (estado === "activo")
 * Lanza excepción si falla (no maneja errores)
 */
export async function fetchActivosContratistas(): Promise<ContratistaResponse[]> {
  const result = await listContratistas();
  return result.contratistas.filter(c => c.estado === "activo");
}