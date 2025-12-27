// ==========================================
// SurrealDB User Repository (Idiomatic)
// ==========================================
// Operaciones CRUD usando patrones nativos de SurrealDB

use crate::models::user::{CreateUserInput, UpdateUserInput, User};
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ==========================================
// TIPOS PARA SURREALDB
// ==========================================

/// Usuario con relación a rol (SurrealDB nativo)
#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealUser {
    pub id: Thing,
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub apellido: String,
    pub role: Thing, // Referencia nativa: roles:admin
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub cedula: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub must_change_password: bool,
    pub deleted_at: Option<String>,
}

/// Usuario con rol expandido (después de FETCH)
#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealUserWithRole {
    pub id: Thing,
    pub email: String,
    pub password: String,
    pub nombre: String,
    pub apellido: String,
    pub role: RoleData, // Rol completo después de FETCH
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub cedula: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub fecha_inicio_labores: Option<String>,
    pub numero_gafete: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub contacto_emergencia_nombre: Option<String>,
    pub contacto_emergencia_telefono: Option<String>,
    pub must_change_password: bool,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleData {
    pub id: Thing,
    pub name: String,
}

impl SurrealUser {
    /// Convierte a User de dominio
    pub fn to_domain(self) -> User {
        User {
            id: self.id.id.to_raw(),
            email: self.email,
            nombre: self.nombre,
            apellido: self.apellido,
            role_id: self.role.id.to_raw(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            cedula: self.cedula,
            segundo_nombre: self.segundo_nombre,
            segundo_apellido: self.segundo_apellido,
            fecha_inicio_labores: self.fecha_inicio_labores,
            numero_gafete: self.numero_gafete,
            fecha_nacimiento: self.fecha_nacimiento,
            telefono: self.telefono,
            direccion: self.direccion,
            contacto_emergencia_nombre: self.contacto_emergencia_nombre,
            contacto_emergencia_telefono: self.contacto_emergencia_telefono,
            must_change_password: self.must_change_password,
            deleted_at: self.deleted_at,
        }
    }
}

impl SurrealUserWithRole {
    /// Convierte a User de dominio con nombre de rol
    pub fn to_domain_with_role(self) -> (User, String) {
        let role_name = self.role.name.clone();
        let user = User {
            id: self.id.id.to_raw(),
            email: self.email,
            nombre: self.nombre,
            apellido: self.apellido,
            role_id: self.role.id.id.to_raw(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            cedula: self.cedula,
            segundo_nombre: self.segundo_nombre,
            segundo_apellido: self.segundo_apellido,
            fecha_inicio_labores: self.fecha_inicio_labores,
            numero_gafete: self.numero_gafete,
            fecha_nacimiento: self.fecha_nacimiento,
            telefono: self.telefono,
            direccion: self.direccion,
            contacto_emergencia_nombre: self.contacto_emergencia_nombre,
            contacto_emergencia_telefono: self.contacto_emergencia_telefono,
            must_change_password: self.must_change_password,
            deleted_at: self.deleted_at,
        };
        (user, role_name)
    }
}

// ==========================================
// QUERIES (Idiomáticas SurrealDB)
// ==========================================

/// Obtiene todos los usuarios activos con su rol (usando FETCH)
pub async fn get_all_users() -> Result<Vec<(User, String)>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // FETCH expande la relación del rol automáticamente
    let mut result = client
        .query(
            r#"
            SELECT * FROM usuarios 
            WHERE deleted_at IS NONE 
            FETCH role
            "#,
        )
        .await?;

    let users: Vec<SurrealUserWithRole> = result.take(0)?;
    Ok(users.into_iter().map(|u| u.to_domain_with_role()).collect())
}

/// Obtiene un usuario por ID con su rol
pub async fn get_user_by_id(id: &str) -> Result<Option<(User, String)>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    println!("[DEBUG] get_user_by_id: buscando id={}", id);

    let mut result = client
        .query(
            r#"
            SELECT * FROM type::thing('usuarios', $id) 
            WHERE deleted_at IS NONE
            FETCH role
            "#,
        )
        .bind(("id", id.to_string()))
        .await?;

    let users: Vec<SurrealUserWithRole> = result.take(0)?;
    println!("[DEBUG] get_user_by_id: encontrados={}", users.len());
    Ok(users.into_iter().next().map(|u| u.to_domain_with_role()))
}

/// Obtiene un usuario por email
pub async fn get_user_by_email(email: String) -> Result<Option<User>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let mut result = client
        .query("SELECT * FROM usuarios WHERE email = $email AND deleted_at IS NONE")
        .bind(("email", email))
        .await?;

    let users: Vec<SurrealUser> = result.take(0)?;
    Ok(users.into_iter().next().map(|u| u.to_domain()))
}

/// Crea un nuevo usuario
/// El índice UNIQUE en email/cedula rechazará duplicados automáticamente
pub async fn create_user(
    input: CreateUserInput,
    password_hash: String,
) -> Result<(User, String), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id = uuid::Uuid::new_v4().to_string();
    let role_id = input.role_id.clone().unwrap_or_else(|| "guardia".to_string());
    let now = chrono::Utc::now().to_rfc3339();
    let must_change_password = input.must_change_password.unwrap_or(true);

    // Crear usuario y retornar con FETCH del rol
    let mut result = client
        .query(
            r#"
            CREATE type::thing('usuarios', $id) CONTENT {
                email: $email,
                password: $password,
                nombre: $nombre,
                apellido: $apellido,
                role: type::thing('roles', $role_id),
                is_active: true,
                created_at: $now,
                updated_at: $now,
                cedula: $cedula,
                segundo_nombre: $segundo_nombre,
                segundo_apellido: $segundo_apellido,
                fecha_inicio_labores: $fecha_inicio_labores,
                numero_gafete: $numero_gafete,
                fecha_nacimiento: $fecha_nacimiento,
                telefono: $telefono,
                direccion: $direccion,
                contacto_emergencia_nombre: $contacto_emergencia_nombre,
                contacto_emergencia_telefono: $contacto_emergencia_telefono,
                must_change_password: $must_change_password,
                deleted_at: NONE
            } RETURN AFTER FETCH role
            "#,
        )
        .bind(("id", id))
        .bind(("email", input.email))
        .bind(("password", password_hash))
        .bind(("nombre", input.nombre))
        .bind(("apellido", input.apellido))
        .bind(("role_id", role_id))
        .bind(("now", now))
        .bind(("cedula", input.cedula))
        .bind(("segundo_nombre", input.segundo_nombre))
        .bind(("segundo_apellido", input.segundo_apellido))
        .bind(("fecha_inicio_labores", input.fecha_inicio_labores))
        .bind(("numero_gafete", input.numero_gafete))
        .bind(("fecha_nacimiento", input.fecha_nacimiento))
        .bind(("telefono", input.telefono))
        .bind(("direccion", input.direccion))
        .bind(("contacto_emergencia_nombre", input.contacto_emergencia_nombre))
        .bind(("contacto_emergencia_telefono", input.contacto_emergencia_telefono))
        .bind(("must_change_password", must_change_password))
        .await?;

    let users: Vec<SurrealUserWithRole> = result.take(0)?;
    users
        .into_iter()
        .next()
        .map(|u| u.to_domain_with_role())
        .ok_or_else(|| SurrealDbError::Query("Error creando usuario".to_string()))
}

/// Actualiza un usuario usando MERGE (solo campos presentes)
pub async fn update_user(
    id: String,
    input: UpdateUserInput,
) -> Result<(User, String), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    // Construir objeto de actualización dinámicamente
    #[derive(Serialize)]
    struct UpdateData {
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        nombre: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        apellido: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cedula: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segundo_nombre: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segundo_apellido: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        must_change_password: Option<bool>,
        updated_at: String,
    }

    let update_data = UpdateData {
        email: input.email,
        nombre: input.nombre,
        apellido: input.apellido,
        is_active: input.is_active,
        cedula: input.cedula,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        must_change_password: input.must_change_password,
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    // MERGE solo actualiza campos presentes
    let mut result = client
        .query(
            r#"
            UPDATE type::thing('usuarios', $id) MERGE $data
            RETURN AFTER FETCH role
            "#,
        )
        .bind(("id", id.clone()))
        .bind(("data", update_data))
        .await?;

    let users: Vec<SurrealUserWithRole> = result.take(0)?;
    users
        .into_iter()
        .next()
        .map(|u| u.to_domain_with_role())
        .ok_or_else(|| SurrealDbError::Query("Usuario no encontrado".to_string()))
}

/// Soft delete de usuario
pub async fn delete_user(id: String) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = chrono::Utc::now().to_rfc3339();

    client
        .query(
            r#"
            UPDATE type::thing('usuarios', $id) SET 
                deleted_at = $now,
                is_active = false
            "#,
        )
        .bind(("id", id))
        .bind(("now", now))
        .await?;

    Ok(())
}

/// Verifica credenciales para login (retorna usuario con rol)
pub async fn verify_credentials(
    email: String,
    password: String,
) -> Result<Option<(User, String)>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    println!("[DEBUG] verify_credentials: buscando email={}", email);

    let mut result = client
        .query(
            r#"
            SELECT * FROM usuarios 
            WHERE email = $email 
            AND deleted_at IS NONE 
            AND is_active = true
            FETCH role
            "#,
        )
        .bind(("email", email.clone()))
        .await?;

    let users: Vec<SurrealUserWithRole> = result.take(0)?;
    println!("[DEBUG] verify_credentials: usuarios encontrados={}", users.len());

    if let Some(user) = users.into_iter().next() {
        println!("[DEBUG] Usuario encontrado: {}", user.email);
        println!(
            "[DEBUG] Password hash almacenado: {}",
            &user.password[..20.min(user.password.len())]
        );

        // Usar auth::verify_password que incluye el secret del keyring
        match crate::services::auth::verify_password(&password, &user.password) {
            Ok(true) => {
                println!("[DEBUG] Password verificado correctamente");
                return Ok(Some(user.to_domain_with_role()));
            }
            Ok(false) => {
                println!("[DEBUG] Password NO coincide");
            }
            Err(e) => {
                println!("[DEBUG] Error verificando password: {}", e);
            }
        }
    } else {
        println!("[DEBUG] Usuario NO encontrado en la DB");
    }

    Ok(None)
}

/// Actualiza solo la contraseña de un usuario
pub async fn update_password(id: String, password_hash: String) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    client
        .query(
            r#"
            UPDATE type::thing('usuarios', $id) SET 
                password = $password,
                must_change_password = false,
                updated_at = $now
            "#,
        )
        .bind(("id", id))
        .bind(("password", password_hash))
        .bind(("now", chrono::Utc::now().to_rfc3339()))
        .await?;

    Ok(())
}
