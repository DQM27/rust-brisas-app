import { invoke } from "@tauri-apps/api/core";
import type {
  ContratistaResponse,
  ContratistaListResponse,
  CreateContratistaInput,
  UpdateContratistaInput
} from "$lib/types/contratista";

// ==========================================
// API: funciones que conectan con Tauri
// Todas usan el DTO real del backend: ContratistaResponse
// ==========================================

// Crear contratista
export async function createContratista(
  input: CreateContratistaInput
): Promise<ContratistaResponse> {
  return await invoke("create_contratista", { input });
}

// Listar contratistas (usa ContratistaResponse[])
export async function listContratistas(): Promise<ContratistaListResponse> {
  return await invoke("list_contratistas");
}

// Obtener un contratista por ID
export async function getContratista(id: string): Promise<ContratistaResponse> {
  return await invoke("get_contratista", { id });
}

// Actualizar contratista
export async function updateContratista(
  input: UpdateContratistaInput
): Promise<ContratistaResponse> {
  return await invoke("update_contratista", { input });
}

// Eliminar contratista
export async function deleteContratista(id: string): Promise<boolean> {
  return await invoke("delete_contratista", { id });
}
