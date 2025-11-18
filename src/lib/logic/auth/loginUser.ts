// loginUser.ts


import { validateCredentials } from './validateCredentials';
import { parseAuthError } from './parseAuthErrors';
import type { UserResponse } from '$lib/types/user';
import { auth as authService } from '$lib/api/auth';


export type LoginResult =
| { ok: true; user: UserResponse }
| { ok: false; message: string; code?: string };


/**
* Orquesta el login: valida, llama al servicio y normaliza el resultado.
* No toca stores ni UI. Retorna un objeto Result.
*/
export async function loginUser(email: string, password: string): Promise<LoginResult> {
// 1) Validación local básica
const v = validateCredentials(email, password);
if (!v.ok) {
return { ok: false, message: v.message };
}


// 2) Llamada al servicio
try {
const user = await authService.login(email, password);


// Aquí asumimos que el servicio lanza en caso de error y retorna user en caso de OK
return { ok: true, user };
} catch (err: any) {
// 3) Parsear error y devolver un mensaje amigable
const parsed = parseAuthError(err?.payload ?? err ?? (err?.message ?? err));
return { ok: false, message: parsed.message, code: parsed.code };
}
}