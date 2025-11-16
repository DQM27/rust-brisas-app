import { invoke } from '@tauri-apps/api/core';
import type { UserResponse } from '$lib/types/user';

export const auth = {
  login: async (email: string, password: string): Promise<UserResponse> => {
    return await invoke<UserResponse>('login', { email, password });
  },

  // Si tienes logout, getCurrentUser, etc.
  // logout: async (): Promise<void> => { ... },
  // getCurrentUser: async (): Promise<UserResponse> => { ... },
};