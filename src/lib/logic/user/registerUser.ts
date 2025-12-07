// src/lib/logic/user/registerUser.ts

import { users } from '$lib/api/users';
import type { UserResponse, CreateUserInput } from '$lib/types/user';

/**
 * Wrapper simple del service users.create()
 * Lanza excepción si falla (no maneja errores)
 */
export async function registerUser(input: CreateUserInput): Promise<UserResponse> {
  return await users.create(input);
}