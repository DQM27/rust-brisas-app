// src/commands/astro_commands.rs

use chrono::{Datelike, Offset, TimeZone, Timelike, Utc};
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

    // Solar Cycle (Local Hours, e.g., 6.25 = 6:15 AM)
    pub sunrise: f64,
    pub sunset: f64,
}

#[tauri::command]
pub async fn get_astro_data(lat: Option<f64>, lon: Option<f64>) -> Result<AstroData, String> {
    let latitude = lat.unwrap_or(DEFAULT_LAT);
    let longitude = lon.unwrap_or(DEFAULT_LON);

    // Obtener fecha actual
    let now_utc = Utc::now();
    let now_local = chrono::Local::now();

    // Calcular datos lunares
    let (age, fraction) = calculate_moon_data(now_utc);
    let phase_id = get_phase_name(age);

    // Calcular datos solares (Salida y Puesta)
    // Usamos el offset local para devolver la hora en el reloj del usuario
    let offset_seconds = now_local.offset().fix().local_minus_utc();
    let offset_hours = f64::from(offset_seconds) / 3600.0;

    let (sunrise_utc, sunset_utc) = calculate_sun_times(now_utc, latitude, longitude);

    // Convertir a hora local y normalizar (0-24)
    let sunrise_local = (sunrise_utc + offset_hours).rem_euclid(24.0);
    let sunset_local = (sunset_utc + offset_hours).rem_euclid(24.0);

    Ok(AstroData {
        moon_phase: phase_id,
        illumination: fraction,
        age_days: age,
        location: format!("Lat: {latitude:.4}, Lon: {longitude:.4}"),
        sunrise: sunrise_local,
        sunset: sunset_local,
    })
}

// ==========================================
// ALGORITMOS ASTRONÓMICOS INTERNOS
// ==========================================

fn get_phase_name(age: f64) -> String {
    // Ciclo lunar promedio: 29.53059 días
    // Normalizamos el ciclo a 8 fases
    // New Moon: 29.5/0 - 1
    if !(1.0..=28.5).contains(&age) {
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

    let a = (f64::from(y) / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    let jd_day = (365.25 * (f64::from(y) + 4716.0)).floor()
        + (30.6001 * (f64::from(m) + 1.0)).floor()
        + f64::from(day)
        + b
        - 1524.5;

    // Agregar fracción del día
    let fraction = (f64::from(date.hour()) / 24.0)
        + (f64::from(date.minute()) / 1440.0)
        + (f64::from(date.second()) / 86400.0);

    jd_day + fraction
}

/// Calcula Sunrise y Sunset (UTC Hours) usando Algoritmo NOAA
fn calculate_sun_times(date: chrono::DateTime<Utc>, lat: f64, lon: f64) -> (f64, f64) {
    // 1. Convertir a Día Juliano del año (1-366)
    let start_of_year = Utc.with_ymd_and_hms(date.year(), 1, 1, 0, 0, 0).unwrap();
    #[allow(clippy::cast_precision_loss)]
    let day_of_year = (date - start_of_year).num_days() as f64 + 1.0;

    // 2. Convertir Longitud a hora (-Oeste, +Este)
    let lng_hour = lon / 15.0;

    // 3. Calcular tiempos aproximados
    // Sunrise (t) = N + ((6 - lngHour) / 24)
    // Sunset (t) = N + ((18 - lngHour) / 24)
    let t_rise = day_of_year + ((6.0 - lng_hour) / 24.0);
    let t_set = day_of_year + ((18.0 - lng_hour) / 24.0);

    // Helper para cálculos solares
    let sun_calc = |t: f64| -> (f64, f64) {
        // Mean Anomaly
        let m = 0.9856f64.mul_add(t, -3.289);

        // True Longitude
        let l = 0.020f64
            .mul_add((2.0 * m).to_radians().sin(), 1.916f64.mul_add((m.to_radians()).sin(), m))
            + 282.634;
        let l = (l + 360.0) % 360.0;

        // Right Ascension
        let mut ra = (l.to_radians().tan()).atan().to_degrees();
        ra = (ra + 360.0) % 360.0;

        // Ajustar cuadrante de RA
        let l_quad = (l / 90.0).floor() * 90.0;
        let ra_quad = (ra / 90.0).floor() * 90.0;
        ra += l_quad - ra_quad;
        ra /= 15.0; // Convertir a horas

        // Sin Declination & Cos Declination
        let sin_dec = 0.39782 * l.to_radians().sin();
        (ra, sin_dec)
    };

    // Zenith oficial = 90 grados 50 minutos
    let zenith: f64 = 90.833;

    // Calcular Sunrise
    let (ra_rise, sin_dec_rise) = sun_calc(t_rise);
    let cos_dec_rise = sin_dec_rise.asin().cos();

    let cos_h_rise = sin_dec_rise.mul_add(-lat.to_radians().sin(), zenith.to_radians().cos())
        / (cos_dec_rise * lat.to_radians().cos());

    // Validar noche polar o sol de medianoche
    if !(-1.0..=1.0).contains(&cos_h_rise) {
        return (6.0, 18.0); // Fallback si no hay sunrise/sunset (polos)
    }

    // Sunrise Hour Angle
    let h_rise = (360.0 - (cos_h_rise.acos().to_degrees())) / 15.0;

    // Local Mean Time UTC
    let t_rise_mid = 0.06571f64.mul_add(-t_rise, h_rise + ra_rise) - 6.622;
    let mut ut_rise = t_rise_mid - lng_hour;
    ut_rise = (ut_rise + 24.0) % 24.0;

    // Calcular Sunset (mismo proceso, diferente Hour Angle)
    let (ra_set, sin_dec_set) = sun_calc(t_set);
    let cos_dec_set = sin_dec_set.asin().cos();
    let cos_h_set = sin_dec_set.mul_add(-lat.to_radians().sin(), zenith.to_radians().cos())
        / (cos_dec_set * lat.to_radians().cos());

    let h_set = (cos_h_set.acos().to_degrees()) / 15.0;

    let t_set_mid = 0.06571f64.mul_add(-t_set, h_set + ra_set) - 6.622;
    let mut ut_set = t_set_mid - lng_hour;
    ut_set = (ut_set + 24.0) % 24.0;

    (ut_rise, ut_set)
}
