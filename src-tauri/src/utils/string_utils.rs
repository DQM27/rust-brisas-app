// ==========================================
// src/utils/string_utils.rs
// ==========================================

/// Normaliza una cédula (trim + uppercase)
pub fn normalize_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

/// Normaliza una placa de vehículo
pub fn normalize_placa(placa: &str) -> String {
    placa.trim().to_uppercase().replace(" ", "")
}

/// Normaliza un nombre (trim + capitalize)
pub fn normalize_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    
    let mut chars = trimmed.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    let rest = chars.as_str().to_lowercase();
    
    format!("{}{}", first, rest)
}

/// Trunca un string a un largo máximo
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Verifica si un string contiene solo números
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_numeric())
}

/// Verifica si un string contiene solo letras y espacios
pub fn is_alphabetic_with_spaces(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_cedula() {
        assert_eq!(normalize_cedula("  123-456  "), "123-456");
    }
    
    #[test]
    fn test_normalize_placa() {
        assert_eq!(normalize_placa("abc 123"), "ABC123");
    }
    
    #[test]
    fn test_normalize_name() {
        assert_eq!(normalize_name("jUAN"), "Juan");
        assert_eq!(normalize_name("  MARÍA  "), "María");
    }
    
    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("123456"));
        assert!(!is_numeric("123a56"));
        assert!(!is_numeric(""));
    }
}