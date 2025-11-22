// src/lib/api/contratistaApi.ts
import { invoke } from "@tauri-apps/api/core";
import type {
  ContratistaResponse,
  ContratistaListResponse,
  CreateContratistaInput,
  UpdateContratistaInput
} from "$lib/types/contratista";

// Nota: los nombres de comando (create_contratista, get_all_contratistas, etc.)
// deben coincidir con los comandos que registraste en Rust (tauri::command).

export async function createContratista(
  input: CreateContratistaInput
): Promise<ContratistaResponse> {
  return await invoke("create_contratista", { input });
}

export async function listContratistas(): Promise<ContratistaListResponse> {
  return await invoke("get_all_contratistas");
}

export async function getContratista(id: string): Promise<ContratistaResponse> {
  return await invoke("get_contratista", { id });
}

export async function updateContratista(
  input: UpdateContratistaInput
): Promise<ContratistaResponse> {
  return await invoke("update_contratista", { id: input.id, input });
}

export async function deleteContratista(id: string): Promise<boolean> {
  return await invoke("delete_contratista", { id });
}
