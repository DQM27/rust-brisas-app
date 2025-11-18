import { validateCredentials } from './validateCredentials';
import { loginUser } from './loginUser';
import { parseAuthError } from './parseAuthErrors';
import type { User } from '$lib/types/user';

export type SubmitLoginResult =
  | { ok: true; user: User }
  | { ok: false; error: string };

export async function submitLogin(
  email: string,
  password: string
): Promise<SubmitLoginResult> {
  // 1. Validar credenciales
  const validation = validateCredentials(email, password);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
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