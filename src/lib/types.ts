// ==========================================
// src/lib/types.ts
// ==========================================


export type UserRole = 'admin' | 'supervisor' | 'guardia';

// Tipo base de User (ya no se usa directamente, pero lo dejamos por compatibilidad)
export interface User {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  role: UserRole;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

// Response que ahora viene de Rust (con campos adicionales)
export interface UserResponse {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  nombreCompleto: string;      // NUEVO
  role: UserRole;
  roleDisplay: string;          // NUEVO - "Administrador", "Supervisor", "Guardia"
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface UserListResponse {
  users: UserResponse[];
  total: number;
  activos: number;
  porRol: {
    admins: number;
    supervisores: number;
    guardias: number;
  };
}

export interface CreateUserInput {
  email: string;
  password: string;
  nombre: string;
  apellido: string;
  role?: string;  // 'admin' | 'supervisor' | 'guardia'
}

export interface UpdateUserInput {
  email?: string;
  password?: string;
  nombre?: string;
  apellido?: string;
  role?: string;
  isActive?: boolean;
}