// ==========================================
// src/models/proveedor.rs
// ==========================================

use serde::{Deserialize, Serialize};

use sqlx::FromRow;

/// Modelo de dominio - Representa un proveedor en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Proveedor {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa_id: String,
    // sqlx no maneja automáticamente enums complejos sin Type, pero como texto sí si coincide.
    // Si EstadoProveedor es texto en DB, necesitamos que sqlx lo pueda leer.
    // Lo más fácil es sqlx::Type si está soportado, o string y conversión manual,
    // pero si usamos FromRow, tiene que mappear directo.
    // Usualmente scan o try_from.
    // Asumiremos que en DB es TEXT.
    // Para simplificar, marcaremos 'estado' como String en el struct DB o implementaremos Type.
    // Revisando el error original: "trait bound not satisfied", falta FromRow.
    // Ojo: EstadoProveedor necesita implementar sqlx::Type o FromRow parará ahí.
    // Mejor cambiamos estado a String en struct y hacemos parsing en el service/impl, o implementamos sqlx::Type.
    // Dado que el error es solo "FromRow not implemented for Proveedor", iniciamos con eso.
    // Si falla en runtime por el enum, lo corregiremos.
    // Pero espera, sqlx map_err dice "Decode".
    // Vamos a derivar sqlx::Type para EstadoProveedor si es posible, o usar String.
    // Por simplicidad y robustez rápida: Usar String en el struct ORM y convertir luego,
    // O derivar sqlx::Type.
    // Vamos a intentar Type primero.
    pub estado: EstadoProveedor,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoProveedor {
    Activo,
    Inactivo,
    Suspendido,
}

impl EstadoProveedor {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoProveedor::Activo => "ACTIVO",
            EstadoProveedor::Inactivo => "INACTIVO",
            EstadoProveedor::Suspendido => "SUSPENDIDO",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "ACTIVO" => Ok(EstadoProveedor::Activo),
            "INACTIVO" => Ok(EstadoProveedor::Inactivo),
            "SUSPENDIDO" => Ok(EstadoProveedor::Suspendido),
            _ => Err(format!("Estado desconocido: {}", s)),
        }
    }
}

impl TryFrom<String> for EstadoProveedor {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        EstadoProveedor::from_str(&s)
    }
}

// Para que sqlx pueda leerlo como String desde la DB y convertirlo
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for EstadoProveedor {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(EstadoProveedor::from_str(&s)?)
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for EstadoProveedor {
    fn encode_by_ref(
        &self,
        buf: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> sqlx::encode::IsNull {
        let s = self.as_str().to_string();
        <String as sqlx::Encode<sqlx::Sqlite>>::encode(s, buf)
    }
}

impl sqlx::Type<sqlx::Sqlite> for EstadoProveedor {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa_id: String,

    // Opcional: Vehículo al crear
    pub tiene_vehiculo: Option<bool>,
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProveedorInput {
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub estado: Option<String>,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProveedorResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: String,
    pub empresa_nombre: String,
    pub estado: EstadoProveedor,
    pub puede_ingresar: bool,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Proveedor> for ProveedorResponse {
    fn from(p: Proveedor) -> Self {
        let mut nombre_completo = p.nombre.clone();
        if let Some(ref s) = p.segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&p.apellido);
        if let Some(ref s) = p.segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        let puede_ingresar = p.estado == EstadoProveedor::Activo;

        Self {
            id: p.id,
            cedula: p.cedula,
            nombre: p.nombre,
            segundo_nombre: p.segundo_nombre,
            apellido: p.apellido,
            segundo_apellido: p.segundo_apellido,
            nombre_completo,
            empresa_id: p.empresa_id,
            empresa_nombre: String::new(), // Se llena en el servicio/query
            estado: p.estado,
            puede_ingresar,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
