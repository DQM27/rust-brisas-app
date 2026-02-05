// src/commands/astro_commands.rs

use chrono::{Datelike, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// Coordenadas de San José, Costa Rica (Default)
const DEFAULT_LAT: f64 = 9.9281;
const DEFAULT_LON: f64 = -84.0907;

#[derive(Debug, Serialize, Deserialize)]
pub struct AstroData {
    pub moon_phase: String, // 'new', 'full', etc.
    pub illumination: f64,  // 0.0 a 1.0 (Porcentaje iluminado)
    pub age_days: f64,      // Días desde luna nueva
    pub location: String,   // Debug info
}

#[tauri::command]
pub async fn get_astro_data(lat: Option<f64>, lon: Option<f64>) -> Result<AstroData, String> {
    let latitude = lat.unwrap_or(DEFAULT_LAT);
    let longitude = lon.unwrap_or(DEFAULT_LON);

    // Obtener fecha actual UTC
    let now_utc = Utc::now();

    // Calcular datos lunares usando algoritmo astronómico (Meeus/Julian Date)
    let (age, fraction) = calculate_moon_data(now_utc);

    // Mapear a nombre de fase
    let phase_id = get_phase_name(age);

    Ok(AstroData {
        moon_phase: phase_id,
        illumination: fraction,
        age_days: age,
        location: format!("Lat: {:.4}, Lon: {:.4}", latitude, longitude),
    })
}

// ==========================================
// ALGORITMOS ASTRONÓMICOS INTERNOS
// ==========================================

fn get_phase_name(age: f64) -> String {
    // Ciclo lunar promedio: 29.53059 días
    // Normalizamos el ciclo a 8 fases
    // New Moon: 29.5/0 - 1
    if age < 1.0 || age > 28.5 {
        return "new".to_string();
    }
    if age < 6.5 {
        return "waxing-crescent".to_string();
    }
    if age < 8.5 {
        return "first-quarter".to_string();
    }
    if age < 13.5 {
        return "waxing-gibbous".to_string();
    }
    if age < 16.5 {
        return "full".to_string();
    }
    if age < 21.5 {
        return "waning-gibbous".to_string();
    }
    if age < 23.5 {
        return "last-quarter".to_string();
    }
    "waning-crescent".to_string()
}

/// Calcula edad lunar y fracción iluminada
fn calculate_moon_data(date: chrono::DateTime<Utc>) -> (f64, f64) {
    let jd = julian_date(date);
    let days_since_new = (jd - 2451550.1) / 29.53058867;
    let cycles = days_since_new.trunc();
    let current_cycle_progress = days_since_new - cycles;

    // Edad en días (0 a 29.53)
    let age = current_cycle_progress * 29.53058867;

    // Ángulo de fase (0 a 2PI)
    // 0 = New, PI = Full
    let phase_angle = current_cycle_progress * 2.0 * PI;

    // Fracción iluminada: (1 - cos(angle)) / 2
    let illumination = (1.0 - phase_angle.cos()) / 2.0;

    (age, illumination)
}

fn julian_date(date: chrono::DateTime<Utc>) -> f64 {
    let year = date.year();
    let month = date.month();
    let day = date.day();

    let (y, m) = if month <= 2 { (year - 1, month + 12) } else { (year, month) };

    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    let jd_day = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + day as f64
        + b
        - 1524.5;

    // Agregar fracción del día
    let fraction = (date.hour() as f64 / 24.0)
        + (date.minute() as f64 / 1440.0)
        + (date.second() as f64 / 86400.0);

    jd_day + fraction
}
