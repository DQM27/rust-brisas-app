// ==========================================
// src/lib/types/user.ts
// ==========================================

// Response de usuario del backend
export interface UserResponse {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  nombreCompleto: string;
  roleId: string;
  roleName: string;  // Nombre del rol para display
  isActive: boolean;
  createdAt: string;
  updatedAt: string;

  // Campos adicionales
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
}

export interface CreateUserInput {
  email: string;
  password?: string;
  nombre: string;
  apellido: string;
  roleId?: string;  // FK a roles

  // Campos adicionales
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
  mustChangePassword?: boolean;
}

export interface UpdateUserInput {
  email?: string;
  password?: string;
  nombre?: string;
  apellido?: string;
  roleId?: string;  // FK a roles
  isActive?: boolean;

  // Campos adicionales
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