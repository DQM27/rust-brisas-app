// src/lib/logic/user/submitRegisterUser.ts

import { registerUser } from './registerUser';
import { parseRegisterError } from './parseRegisterErrors';
import type { UserResponse, CreateUserInput } from '$lib/types/user';
import { CreateUserSchema } from '$lib/schemas/userSchema';

export type SubmitRegisterResult =
  | { ok: true; user: UserResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de registro:
 * 1. Valida input con Zod
 * 2. Llama al service (con todos los campos)
 * 3. Parsea errores si falló
 */
export async function submitRegisterUser(
  input: CreateUserInput
): Promise<SubmitRegisterResult> {
  // 1. Validar input con Zod
  const validation = CreateUserSchema.safeParse(input);

  if (!validation.success) {
    // Tomamos el primer error para mostrar
    const firstError = validation.error.issues[0];
    return { ok: false, error: firstError ? firstError.message : "Error de validación" };
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
