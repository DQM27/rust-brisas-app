
// ==========================================
// src/lib/tauri.ts
// ==========================================
import { invoke } from '@tauri-apps/api/core';
import type { User, CreateUserInput, UpdateUserInput } from './types';

export const tauri = {
  createUser: async (data: CreateUserInput): Promise<User> => {
    const user = await invoke<User>('create_user', { input: data });
    return user;
  },

  login: async (email: string, password: string): Promise<User> => {
    const user = await invoke<User>('login', { email, password });
    return user;
  },

  listUsers: async (): Promise<User[]> => {
    const users = await invoke<User[]>('get_all_users');
    return users;
  },

  getUserById: async (id: string): Promise<User> => {
    const user = await invoke<User>('get_user_by_id', { id });
    return user;
  },

  updateUser: async (id: string, data: UpdateUserInput): Promise<User> => {
    const user = await invoke<User>('update_user', { id, input: data });
    return user;
  },

  deleteUser: async (id: string): Promise<void> => {
    await invoke('delete_user', { id });
  },
};