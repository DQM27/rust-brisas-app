// src/lib/logic/user/submitRegisterUser.ts

import { validateUserInput } from './validateUserInput';
import { registerUser } from './registerUser';
import { parseRegisterError } from './parseRegisterErrors';
import type { UserResponse, CreateUserInput } from '$lib/types/user';

export type SubmitRegisterResult =
  | { ok: true; user: UserResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de registro:
 * 1. Valida input (solo obligatorios)
 * 2. Llama al service (con todos los campos)
 * 3. Parsea errores si falló
 */
export async function submitRegisterUser(
  input: CreateUserInput
): Promise<SubmitRegisterResult> {
  // 1. Validar obligatorios
  const validation = validateUserInput(
    input.email,
    input.password,
    input.nombre,
    input.apellido
  );
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar registrar
  try {
    const user = await registerUser(input);
    return { ok: true, user };
  } catch (err: any) {
    // 3. Parsear error
    const errorMessage = parseRegisterError(err);
    return { ok: false, error: errorMessage };
  }
}
