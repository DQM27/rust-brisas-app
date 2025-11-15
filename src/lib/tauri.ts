// ==========================================
// src/lib/tauri.ts
// ==========================================
import { invoke } from '@tauri-apps/api/core';
import type { 
  UserResponse, 
  UserListResponse, 
  CreateUserInput, 
  UpdateUserInput 
} from './types';

export const tauri = {
  // ==========================================
  // User Commands
  // ==========================================
  
  createUser: async (data: CreateUserInput): Promise<UserResponse> => {
    const user = await invoke<UserResponse>('create_user', { input: data });
    return user;
  },

  login: async (email: string, password: string): Promise<UserResponse> => {
    const user = await invoke<UserResponse>('login', { email, password });
    return user;
  },

  listUsers: async (): Promise<UserListResponse> => {
    const response = await invoke<UserListResponse>('get_all_users');
    return response;
  },

  getUserById: async (id: string): Promise<UserResponse> => {
    const user = await invoke<UserResponse>('get_user_by_id', { id });
    return user;
  },

  updateUser: async (id: string, data: UpdateUserInput): Promise<UserResponse> => {
    const user = await invoke<UserResponse>('update_user', { id, input: data });
    return user;
  },

  deleteUser: async (id: string): Promise<void> => {
    await invoke('delete_user', { id });
  },
};