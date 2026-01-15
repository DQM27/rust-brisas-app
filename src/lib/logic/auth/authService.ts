// src/lib/logic/auth/authService.ts
import { auth as authApi } from '$lib/api/auth';
import type { UserResponse } from '$lib/types/user';
import type { LoginForm, ChangePasswordForm } from '$lib/schemas/userSchema';

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string; code?: string };

// Helper para parsear errores
function parseAuthError(err: any): { message: string; code?: string } {
	if (!err) return { message: 'Ocurrió un error desconocido.' };

	if (typeof err === 'string') {
		if (/credential/i.test(err))
			return { message: 'Credenciales inválidas.', code: 'INVALID_CREDENTIALS' };
		if (/inactive/i.test(err)) return { message: 'Usuario desactivado.', code: 'USER_INACTIVE' };
		return { message: err };
	}

	if (typeof err === 'object') {
		// Intentar extraer mensaje del objeto
		let msg = 'Error desconocido';

		if (err.message) {
			msg = typeof err.message === 'string' ? err.message : JSON.stringify(err.message);
		} else if (err.error) {
			msg = typeof err.error === 'string' ? err.error : JSON.stringify(err.error);
		} else {
			msg = JSON.stringify(err);
		}

		const code = err.code || undefined;

		if (/credential/i.test(msg))
			return { message: 'Credenciales inválidas.', code: 'INVALID_CREDENTIALS' };
		if (/inactive/i.test(msg))
			return {
				message: 'Tu cuenta está desactivada. Contacta al administrador.',
				code: 'USER_INACTIVE'
			};

		return { message: msg, code };
	}

	return { message: 'Ocurrió un error inesperado.' };
}

export const authService = {
	/**
	 * Iniciar sesión
	 */
	login: async (data: LoginForm): Promise<ServiceResult<UserResponse>> => {
		try {
			const user = await authApi.login(data.email, data.password);
			return { ok: true, data: user };
		} catch (err: any) {
			const { message, code } = parseAuthError(err?.payload ?? err);
			return { ok: false, error: message, code };
		}
	},

	/**
	 * Cambiar contraseña
	 */
	changePassword: async (
		userId: string,
		data: ChangePasswordForm
	): Promise<ServiceResult<void>> => {
		try {
			await authApi.changePassword(userId, data);
			return { ok: true, data: undefined };
		} catch (err: any) {
			const { message, code } = parseAuthError(err?.payload ?? err);
			return { ok: false, error: message, code };
		}
	}
};
