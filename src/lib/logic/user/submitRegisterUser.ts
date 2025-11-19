// src/lib/logic/user/submitRegisterUser.ts

import { validateUserInput } from './validateUserInput';
import { registerUser } from './registerUser';
import { parseRegisterError } from './parseRegisterErrors';
import type { UserResponse, UserRole } from '$lib/types/user';

export type SubmitRegisterResult =
  | { ok: true; user: UserResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de registro:
 * 1. Valida input
 * 2. Llama al service
 * 3. Parsea errores si falló
 */
export async function submitRegisterUser(
  email: string,
  password: string,
  nombre: string,
  apellido: string,
  role: UserRole
): Promise<SubmitRegisterResult> {
  // 1. Validar
  const validation = validateUserInput(email, password, nombre, apellido);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar registrar
  try {
    const user = await registerUser({ email, password, nombre, apellido, role });
    return { ok: true, user };
  } catch (err: any) {
    // 3. Parsear error
    const errorMessage = parseRegisterError(err);
    return { ok: false, error: errorMessage };
  }
}
