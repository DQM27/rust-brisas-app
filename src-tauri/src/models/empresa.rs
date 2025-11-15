// src/models/empresa.rs

use serde::{Deserialize, Serialize};

/// Modelo de dominio - Representa una empresa en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Empresa {
    pub id: String,
    pub nombre: String,
    pub ruc: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs de entrada
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmpresaInput {
    pub nombre: String,
    pub ruc: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmpresaInput {
    pub nombre: Option<String>,
    pub ruc: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs de salida
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaResponse {
    pub id: String,
    pub nombre: String,
    pub ruc: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub is_active: bool,
    pub total_contratistas: usize,
    pub contratistas_activos: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaListResponse {
    pub empresas: Vec<EmpresaResponse>,
    pub total: usize,
    pub activas: usize,
}

// ==========================================
// Validaciones
// ==========================================

pub mod validaciones {
    pub fn validar_nombre(nombre: &str) -> Result<(), String> {
        let limpio = nombre.trim();
        
        if limpio.is_empty() {
            return Err("El nombre de la empresa no puede estar vacío".to_string());
        }
        
        if limpio.len() < 2 {
            return Err("El nombre debe tener al menos 2 caracteres".to_string());
        }
        
        if limpio.len() > 100 {
            return Err("El nombre no puede exceder 100 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_ruc(ruc: &str) -> Result<(), String> {
        let limpio = ruc.trim();
        
        if limpio.is_empty() {
            return Ok(()); // RUC es opcional
        }
        
        if !limpio.chars().all(|c| c.is_numeric() || c == '-') {
            return Err("El RUC solo puede contener números y guiones".to_string());
        }
        
        if limpio.len() > 20 {
            return Err("El RUC no puede exceder 20 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_email(email: &str) -> Result<(), String> {
        let limpio = email.trim();
        
        if limpio.is_empty() {
            return Ok(()); // Email es opcional
        }
        
        if !limpio.contains('@') {
            return Err("Email inválido".to_string());
        }
        
        if limpio.len() > 100 {
            return Err("El email no puede exceder 100 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_create_input(input: &super::CreateEmpresaInput) -> Result<(), String> {
        validar_nombre(&input.nombre)?;
        
        if let Some(ref ruc) = input.ruc {
            validar_ruc(ruc)?;
        }
        
        if let Some(ref email) = input.email {
            validar_email(email)?;
        }
        
        Ok(())
    }
}