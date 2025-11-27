import { validateContratistaInput } from './validateContratistaInput';
import { registerContratista } from './registerContratista';
import { parseContratistaError } from './parseContratistaErrors';
import { validateVehiculoInput } from '$lib/logic/vehiculo/validateVehiculoInput';
import { submitRegisterVehiculo } from '$lib/logic/vehiculo/submitRegisterVehiculo';
import { reindexAllContratistas } from '$lib/api/searchService';

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
 * 1. Valida input del contratista (validateContratistaInput)
 * 2. Si tieneVehiculo, valida input del vehículo (validateVehiculoInput)
 * 3. Llama al servicio de registro del contratista (registerContratista)
 * 4. Si tieneVehiculo, registra el vehículo (submitRegisterVehiculo)
 * 5. Parsea errores si falla (parseContratistaError)
 * 
 * @param input - Objeto con los datos del nuevo contratista y vehículo (opcional).
 * @returns Promesa que resuelve con SubmitRegisterContratistaResult.
 */
export async function submitRegisterContratista(
  input: CreateContratistaInput
): Promise<SubmitRegisterContratistaResult> {

  const { nombre, apellido, cedula, empresaId, fechaVencimientoPraind, tieneVehiculo, tipoVehiculo, placa, marca, modelo, color } = input;

  // 1. Validar campos requeridos del contratista
  const contratistaValidation = validateContratistaInput(
    nombre,
    apellido,
    cedula,
    empresaId,
    fechaVencimientoPraind
  );

  if (!contratistaValidation.ok) {
    return { ok: false, error: contratistaValidation.message };
  }

  // 2. Si tieneVehiculo, validar campos del vehículo
  if (tieneVehiculo) {
    if (!tipoVehiculo) {
      return { ok: false, error: 'Debe seleccionar un tipo de vehículo.' };
    }

    const vehiculoValidation = validateVehiculoInput(
      tipoVehiculo,
      placa || '',
      marca
    );

    if (!vehiculoValidation.ok) {
      return { ok: false, error: vehiculoValidation.message };
    }
  }

  // 3. Intentar crear el contratista
  try {
    const contratista = await registerContratista(input);

    // 4. Si tieneVehiculo, registrar el vehículo
    if (tieneVehiculo && tipoVehiculo && placa) {
      const vehiculoResult = await submitRegisterVehiculo({
        contratistaId: contratista.id,
        tipoVehiculo,
        placa,
        marca,
        modelo,
        color
      });

      if (!vehiculoResult.ok) {
        return { ok: false, error: `Error al registrar vehículo: ${vehiculoResult.error}` };
      }
    }

    // 5. Reindexar búsqueda
    await reindexAllContratistas();

    // Éxito completo
    return { ok: true, contratista };
  } catch (err: any) {
    // 5. Parsea el error para devolver un mensaje amigable al usuario
    const msg = parseContratistaError(err);
    return { ok: false, error: msg };
  }
}