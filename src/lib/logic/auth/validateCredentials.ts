// validateCredentials.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateCredentials(
  email: string,
  password: string
): ValidationResult {
  const e = (email || "").trim();
  const p = password || "";

  if (e.length === 0) {
    return { ok: false, message: "El email no puede estar vacío." };
  }

  if (!e.includes("@")) {
    return { ok: false, message: "El email no tiene un formato válido." };
  }

  if (e.length > 100) {
    return { ok: false, message: "El email no puede exceder 100 caracteres." };
  }

  if (p.length === 0) {
    return { ok: false, message: "La contraseña no puede estar vacía." };
  }

  if (p.length < 6) {
    return { ok: false, message: "La contraseña debe tener al menos 6 caracteres." };
  }

  if (p.length > 100) {
    return { ok: false, message: "La contraseña no puede exceder 100 caracteres." };
  }

  return { ok: true };
}
