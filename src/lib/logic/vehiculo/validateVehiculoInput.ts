// src/lib/logic/vehiculo/validateVehiculoInput.ts

import type { TipoVehiculo } from '$lib/types/vehiculo';

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

const TIPOS_VALIDOS: TipoVehiculo[] = ['motocicleta', 'automóvil'];

export function validateVehiculoInput(
  tipoVehiculo: string,
  placa: string,
  marca?: string
): ValidationResult {
  // Validar tipo de vehículo
  if (!tipoVehiculo || !tipoVehiculo.trim()) {
    return { ok: false, message: 'Debe seleccionar un tipo de vehículo.' };
  }

  if (!TIPOS_VALIDOS.includes(tipoVehiculo as TipoVehiculo)) {
    return { ok: false, message: 'Tipo de vehículo inválido.' };
  }

  // Validar placa
  const p = (placa || '').trim();
  if (!p) {
    return { ok: false, message: 'La placa no puede estar vacía.' };
  }

  if (p.length < 3 || p.length > 15) {
    return { ok: false, message: 'La placa debe tener entre 3 y 15 caracteres.' };
  }

  // Validar que solo contenga alfanuméricos, guiones y espacios
  if (!/^[a-zA-Z0-9\s\-]+$/.test(p)) {
    return { ok: false, message: 'La placa solo puede contener letras, números, guiones y espacios.' };
  }

  // Validar marca (obligatoria)
  const m = (marca || '').trim();
  if (!m) {
    return { ok: false, message: 'La marca no puede estar vacía.' };
  }

  if (m.length > 50) {
    return { ok: false, message: 'La marca no puede exceder 50 caracteres.' };
  }

  return { ok: true };
}