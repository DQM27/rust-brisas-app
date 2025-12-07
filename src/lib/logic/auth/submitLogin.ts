// src/lib/logic/auth/submitLogin.ts
import { loginUser } from './loginUser';
import { parseAuthError } from './parseAuthErrors';
import type { UserResponse } from '$lib/types/user';
import { LoginSchema } from '$lib/schemas/userSchema';

export type SubmitLoginResult =
  | { ok: true; user: UserResponse }
  | { ok: false; error: string };

export async function submitLogin(
  email: string,
  password: string
): Promise<SubmitLoginResult> {
  // 1. Validar credenciales con Zod
  const validation = LoginSchema.safeParse({ email, password });

  if (!validation.success) {
    const firstError = validation.error.issues[0];
    return { ok: false, error: firstError ? firstError.message : "Error de validación" };
  }

  // 2. Intentar login
  const result = await loginUser(email, password);
  if (!result.ok) {
    const parsed = parseAuthError({ message: result.message, code: result.code });
    return { ok: false, error: parsed.message };
  }

  // 3. Éxito
  return { ok: true, user: result.user };
}