// ==========================================
// src/lib/types.ts
// ==========================================
export interface User {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  role: string;
  isActive: boolean;  // camelCase para JS
  createdAt: string;  // camelCase para JS
  updatedAt: string;  // camelCase para JS
}

export interface CreateUserInput {
  email: string;
  password: string;
  nombre: string;
  apellido: string;
  role?: string;
}

export interface UpdateUserInput {
  email?: string;
  password?: string;
  nombre?: string;
  apellido?: string;
  role?: string;
  isActive?: boolean;  // camelCase para JS
}

// Tipo que viene de Rust (snake_case)
interface UserFromRust {
  id: string;
  email: string;
  nombre: string;
  apellido: string;
  role: string;
  isActive: boolean;   // Rust ya lo convierte a camelCase gracias a #[serde(rename_all = "camelCase")]
  createdAt: string;
  updatedAt: string;
}