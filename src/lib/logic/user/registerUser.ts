// src/lib/logic/user/registerUser.ts

import { users } from '$lib/api/users';
import type { ValidationResult } from './validateUserInput';

export type RegisterResult =
  | { ok: true; user: any }
  | { ok: false; error: string };

export async function registerUser(input: {
  email: string;
  password: string;
  nombre: string;
  apellido: string;
  role?: string;
}): Promise<RegisterResult> {
  try {
    const user = await users.create(input);
    return { ok: true, user };
  } catch (err: any) {
    const msg = err?.message || 'Ocurrió un error al crear el usuario.';
    return { ok: false, error: msg };
  }
}
