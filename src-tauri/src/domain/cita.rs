use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cita {
    pub id: RecordId,
    pub visitante_id: RecordId,
    pub fecha_cita: surrealdb::Datetime,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub estado: String, // PENDIENTE, COMPLETADA, CANCELADA, EXPIRADA
    pub registrado_por: String,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CitaPopulated {
    pub id: String,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub estado: String,
    // Datos del visitante "aplanados" para la UI
    pub visitante_id: String,
    pub visitante_cedula: String,
    pub visitante_nombre: String,
    pub visitante_apellido: String,
    pub visitante_nombre_completo: String,
    pub visitante_empresa: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCitaInput {
    pub visitante_id: String,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub registrado_por: String,
}

// ==========================================
// VALIDACIONES
// ==========================================

pub fn validar_create_input(input: &CreateCitaInput) -> Result<(), String> {
    if input.visitante_id.trim().is_empty() {
        return Err("El ID del visitante es obligatorio".to_string());
    }
    if input.anfitrion.trim().is_empty() {
        return Err("Debe especificar un anfitrión".to_string());
    }
    if input.area_visitada.trim().is_empty() {
        return Err("Debe especificar el área visitada".to_string());
    }
    if input.motivo.trim().is_empty() {
        return Err("Debe especificar el motivo de la cita".to_string());
    }
    Ok(())
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_create_input_valido() {
        let input = CreateCitaInput {
            visitante_id: "uuid-1".to_string(),
            fecha_cita: "2023-12-25".to_string(),
            anfitrion: "John Doe".to_string(),
            area_visitada: "Sistemas".to_string(),
            motivo: "Reunión técnica".to_string(),
            registrado_por: "admin".to_string(),
        };
        assert!(validar_create_input(&input).is_ok());
    }

    #[test]
    fn test_validar_create_input_incompleto() {
        let input = CreateCitaInput {
            visitante_id: "".to_string(), // Invalido
            fecha_cita: "2023-12-25".to_string(),
            anfitrion: "".to_string(), // Invalido
            area_visitada: "".to_string(),
            motivo: "".to_string(),
            registrado_por: "".to_string(),
        };
        assert!(validar_create_input(&input).is_err());
    }
}
