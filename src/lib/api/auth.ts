import { invoke } from '@tauri-apps/api/core';
import type { UserResponse } from '$lib/types/user';
import type { ChangePasswordForm } from '$lib/schemas/userSchema';

export const auth = {
	login: async (email: string, password: string): Promise<UserResponse> => {
		return await invoke<UserResponse>('login', { email, password });
	},

	changePassword: async (id: string, input: ChangePasswordForm): Promise<void> => {
		return await invoke('change_password', { id, input });
	}
};
