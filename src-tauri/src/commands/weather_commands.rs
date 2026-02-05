// src/commands/weather_commands.rs

use serde::{Deserialize, Serialize};

const DEFAULT_LAT: f64 = 9.9281;
const DEFAULT_LON: f64 = -84.0907;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub weather_code: u32, // WMO Code
    pub wind_speed: f64,
    pub is_day: u8, // 1 = Day, 0 = Night
    pub condition_text: String,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: u32,
    is_day: u8,
}

#[tauri::command]
pub async fn get_weather_data(lat: Option<f64>, lon: Option<f64>) -> Result<WeatherData, String> {
    let latitude = lat.unwrap_or(DEFAULT_LAT);
    let longitude = lon.unwrap_or(DEFAULT_LON);

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        latitude, longitude
    );

    // Hacemos el request síncrono (blocking) para simplicidad en este comando async
    // Reqwest blocking feature debe estar habilitada
    let response = reqwest::blocking::get(&url).map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API Error: {}", response.status()));
    }

    let weather_res: OpenMeteoResponse =
        response.json().map_err(|e| format!("Parse error: {}", e))?;

    let current = weather_res.current_weather;
    let condition = get_condition_text(current.weathercode);

    Ok(WeatherData {
        temperature: current.temperature,
        weather_code: current.weathercode,
        wind_speed: current.windspeed,
        is_day: current.is_day,
        condition_text: condition,
    })
}

fn get_condition_text(code: u32) -> String {
    match code {
        0 => "Cielo Despejado".to_string(),
        1..=3 => "Parcialmente Nublado".to_string(),
        45 | 48 => "Neblina".to_string(),
        51..=55 => "Llovizna".to_string(),
        56 | 57 => "Llovizna Helada".to_string(),
        61..=65 => "Lluvia".to_string(),
        66 | 67 => "Lluvia Helada".to_string(),
        71..=77 => "Nieve".to_string(),
        80..=82 => "Chubascos".to_string(),
        85 | 86 => "Chubascos de Nieve".to_string(),
        95 => "Tormenta Eléctrica".to_string(),
        96 | 99 => "Tormenta con Granizo".to_string(),
        _ => "Desconocido".to_string(),
    }
}
