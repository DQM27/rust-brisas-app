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
  nombreCompleto: string;
  role: UserRole;
  roleDisplay: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;

  // Nuevos campos opcionales
  cedula: string;
  segundoNombre?: string | null;
  segundoApellido?: string | null;
  fechaInicioLabores?: string | null;
  numeroGafete?: string | null;
  fechaNacimiento?: string | null;
  telefono?: string | null;
  direccion?: string | null;
  contactoEmergenciaNombre?: string | null;
  contactoEmergenciaTelefono?: string | null;
  temporaryPassword?: string | null;
  mustChangePassword: boolean;
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
  password?: string;
  nombre: string;
  apellido: string;
  role?: string;

  // Nuevos campos
  cedula: string;
  segundoNombre?: string;
  segundoApellido?: string;
  fechaInicioLabores?: string;
  numeroGafete?: string;
  fechaNacimiento?: string;
  telefono?: string;
  direccion?: string;
  contactoEmergenciaNombre?: string;
  contactoEmergenciaTelefono?: string;
}

export interface UpdateUserInput {
  email?: string;
  password?: string;
  nombre?: string;
  apellido?: string;
  role?: string;
  isActive?: boolean;

  // Nuevos campos
  cedula?: string;
  segundoNombre?: string;
  segundoApellido?: string;
  fechaInicioLabores?: string;
  numeroGafete?: string;
  fechaNacimiento?: string;
  telefono?: string;
  direccion?: string;
  contactoEmergenciaNombre?: string;
  contactoEmergenciaTelefono?: string;
  mustChangePassword?: boolean;
}