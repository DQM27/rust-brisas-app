import { invoke } from '@tauri-apps/api/core';
import type { 
  UserResponse, 
  UserListResponse, 
  CreateUserInput, 
  UpdateUserInput 
} from '$lib/types/user';

export const users = {
  create: async (data: CreateUserInput): Promise<UserResponse> => {
    return await invoke<UserResponse>('create_user', { input: data });
  },

  list: async (): Promise<UserListResponse> => {
    return await invoke<UserListResponse>('get_all_users');
  },

  getById: async (id: string): Promise<UserResponse> => {
    return await invoke<UserResponse>('get_user_by_id', { id });
  },

  update: async (id: string, data: UpdateUserInput): Promise<UserResponse> => {
    return await invoke<UserResponse>('update_user', { id, input: data });
  },

  delete: async (id: string): Promise<void> => {
    await invoke('delete_user', { id });
  },
};