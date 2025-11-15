// ==========================================
// src/utils/date_utils.rs
// ==========================================

use chrono::{NaiveDate, NaiveDateTime, Utc};

/// Parsea una fecha en formato YYYY-MM-DD
pub fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| format!("Formato de fecha inválido: {}", e))
}

/// Parsea un datetime en formato YYYY-MM-DD HH:MM:SS
pub fn parse_datetime(datetime_str: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Formato de datetime inválido: {}", e))
}

/// Obtiene el timestamp actual en formato RFC3339
pub fn now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Obtiene el timestamp actual en formato YYYY-MM-DD HH:MM:SS
pub fn now_sql_format() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Calcula días entre dos fechas
pub fn days_between(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days()
}

/// Verifica si una fecha está vencida
pub fn is_expired(date: NaiveDate) -> bool {
    date < Utc::now().date_naive()
}

/// Calcula días hasta una fecha (negativo si ya pasó)
pub fn days_until(date: NaiveDate) -> i64 {
    days_between(Utc::now().date_naive(), date)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-01-15").is_ok());
        assert!(parse_date("invalid").is_err());
    }
    
    #[test]
    fn test_days_between() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 11).unwrap();
        assert_eq!(days_between(date1, date2), 10);
    }
}