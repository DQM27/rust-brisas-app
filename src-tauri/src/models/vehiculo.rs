// ==========================================
// src/models/vehiculo.rs
// ==========================================
use serde::{Deserialize, Serialize};

/// Modelo de dominio - Representa un vehículo registrado
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehiculo {
    pub id: String,
    pub contratista_id: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVehiculoInput {
    pub contratista_id: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehiculoInput {
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiculoResponse {
    pub id: String,
    pub contratista_id: String,
    pub contratista_nombre: String,      // JOIN con contratistas
    pub contratista_cedula: String,      // Para mostrar en UI
    pub empresa_nombre: String,          // JOIN con empresas
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub descripcion_completa: String,    // "Placa ABC123 - Toyota Corolla (Rojo)"
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Vehiculo> for VehiculoResponse {
    fn from(v: Vehiculo) -> Self {
        let marca_str = v.marca.clone().unwrap_or_else(|| "N/A".to_string());
        let modelo_str = v.modelo.clone().unwrap_or_else(|| "N/A".to_string());
        let color_str = v.color.clone().unwrap_or_else(|| "N/A".to_string());
        
        let descripcion_completa = if v.marca.is_some() || v.modelo.is_some() {
            format!(
                "Placa {} - {} {} ({})",
                v.placa,
                marca_str,
                modelo_str,
                color_str
            )
        } else {
            format!("Placa {}", v.placa)
        };
        
        Self {
            id: v.id,
            contratista_id: v.contratista_id,
            contratista_nombre: String::new(),  // Se llena en el comando con JOIN
            contratista_cedula: String::new(),  // Se llena en el comando con JOIN
            empresa_nombre: String::new(),      // Se llena en el comando con JOIN
            placa: v.placa,
            marca: v.marca,
            modelo: v.modelo,
            color: v.color,
            descripcion_completa,
            is_active: v.is_active,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiculoListResponse {
    pub vehiculos: Vec<VehiculoResponse>,
    pub total: usize,
    pub activos: usize,
    pub inactivos: usize,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use super::*;
    
    pub fn validar_placa(placa: &str) -> Result<(), String> {
        let limpia = placa.trim().to_uppercase();
        
        if limpia.is_empty() {
            return Err("La placa no puede estar vacía".to_string());
        }
        
        // Validación flexible: solo alfanuméricos y guiones
        if !limpia.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ' ') {
            return Err("La placa solo puede contener letras, números, guiones y espacios".to_string());
        }
        
        if limpia.len() < 3 || limpia.len() > 15 {
            return Err("La placa debe tener entre 3 y 15 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_contratista_id(contratista_id: &str) -> Result<(), String> {
        let limpia = contratista_id.trim();
        
        if limpia.is_empty() {
            return Err("Debe especificar un contratista".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_texto_opcional(texto: &str, campo: &str, max_len: usize) -> Result<(), String> {
        let limpio = texto.trim();
        
        if limpio.len() > max_len {
            return Err(format!("{} no puede exceder {} caracteres", campo, max_len));
        }
        
        Ok(())
    }
    
    pub fn validar_create_input(input: &CreateVehiculoInput) -> Result<(), String> {
        validar_contratista_id(&input.contratista_id)?;
        validar_placa(&input.placa)?;
        
        if let Some(ref marca) = input.marca {
            validar_texto_opcional(marca, "Marca", 50)?;
        }
        
        if let Some(ref modelo) = input.modelo {
            validar_texto_opcional(modelo, "Modelo", 50)?;
        }
        
        if let Some(ref color) = input.color {
            validar_texto_opcional(color, "Color", 30)?;
        }
        
        Ok(())
    }
}