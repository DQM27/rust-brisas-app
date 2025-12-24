// ==========================================
// src/lib/services/keyringService.ts
// ==========================================
// Servicio para gestionar credenciales seguras via Tauri

import { invoke } from '@tauri-apps/api/core';

// ==========================================
// TIPOS
// ==========================================

export interface CredentialStatus {
  argon2_configured: boolean;
  fully_configured: boolean;
}

export interface Argon2Params {
  memory: number;
  iterations: number;
  parallelism: number;
  secret: string;
}

export interface Argon2ParamsSafe {
  memory: number;
  iterations: number;
  parallelism: number;
  has_secret: boolean;
}

export interface SetupCredentialsInput {
  argon2: Argon2Params;
  terminal_name: string;
  terminal_location: string;
}

export interface SetupResult {
  success: boolean;
  message: string;
}

// ==========================================
// FUNCIONES DE ESTADO
// ==========================================

export async function getCredentialStatus(): Promise<CredentialStatus> {
  return invoke<CredentialStatus>('get_credential_status');
}

export async function isAppConfigured(): Promise<boolean> {
  return invoke<boolean>('is_app_configured');
}

export async function needsSetup(): Promise<boolean> {
  return invoke<boolean>('needs_setup');
}

// ==========================================
// SETUP INICIAL
// ==========================================

export async function setupCredentials(input: SetupCredentialsInput): Promise<SetupResult> {
  return invoke<SetupResult>('setup_credentials', { input });
}



// ==========================================
// ARGON2
// ==========================================

export async function getArgon2Config(): Promise<Argon2ParamsSafe> {
  return invoke<Argon2ParamsSafe>('get_argon2_config');
}

export async function updateArgon2Params(params: Argon2Params): Promise<void> {
  return invoke('update_argon2_params', { params });
}

export async function generateArgon2Secret(): Promise<string> {
  return invoke<string>('generate_argon2_secret');
}


// ==========================================
// UTILIDADES
// ==========================================

export async function generateRandomSecret(): Promise<string> {
  return invoke<string>('generate_random_secret');
}

export async function resetAllCredentials(confirm: boolean): Promise<void> {
  return invoke('reset_all_credentials', { confirm });
}

export async function exitApp(): Promise<void> {
  return invoke('exit_app');
}

export async function setWindowDecorations(decorations: boolean): Promise<void> {
  return invoke('set_window_decorations', { decorations });
}

export async function setWindowSize(width: number, height: number): Promise<void> {
  return invoke('set_window_size', { width, height });
}
