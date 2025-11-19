// src/lib/logic/user/registerUser.ts

import { users } from '$lib/api/users';
import type { UserResponse } from '$lib/types/user';

/**
 * Wrapper simple del service users.create()
 * Lanza excepción si falla (no maneja errores)
 */
export async function registerUser(input: {
  email: string;
  password: string;
  nombre: string;
  apellido: string;
  role?: string;
}): Promise<UserResponse> {
  return await users.create(input);
}