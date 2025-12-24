// src/config/seed_demo.rs
//
// ==========================================
// SEED DE DEMOSTRACIÓN
// ==========================================
// Este módulo genera datos mockeados para demostrar
// todas las funcionalidades del sistema.
//
// USO: Solo se ejecuta bajo demanda (Modo Demo)
// NO se ejecuta en startup normal.

use chrono::{Duration, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::domain::role::{ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID};
use crate::services::auth::hash_password;

/// Ejecuta todos los seeds de demostración
pub async fn run_demo_seed(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    seed_demo_users(pool).await?;
    seed_demo_empresas(pool).await?;
    seed_demo_gafetes(pool).await?;
    seed_demo_contratistas(pool).await?;
    seed_demo_proveedores(pool).await?;
    seed_demo_visitantes(pool).await?;
    seed_demo_ingresos_contratistas(pool).await?;
    seed_demo_alertas_gafete(pool).await?;
    seed_demo_lista_negra(pool).await?;
    Ok(())
}

// ==========================================
// USUARIOS DE PRUEBA
// ==========================================

async fn seed_demo_users(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password("demo123")?;

    let users = [
        ("demo-supervisor-1", "supervisor@demo.com", "María", "González", ROLE_SUPERVISOR_ID),
        ("demo-guardia-1", "guardia@demo.com", "Carlos", "Rodríguez", ROLE_GUARDIA_ID),
        ("demo-guardia-2", "guardia2@demo.com", "Ana", "Martínez", ROLE_GUARDIA_ID),
    ];

    for (id, email, nombre, apellido, role_id) in users {
        sqlx::query(
            r#"INSERT OR IGNORE INTO users 
               (id, email, password_hash, nombre, apellido, role_id, is_active, created_at, updated_at, cedula, must_change_password)
               VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?, ?, 0)"#,
        )
        .bind(id)
        .bind(email)
        .bind(&password_hash)
        .bind(nombre)
        .bind(apellido)
        .bind(role_id)
        .bind(&now)
        .bind(&now)
        .bind(format!("DEMO{}", Uuid::new_v4().to_string().split('-').next().unwrap()))
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// EMPRESAS DE PRUEBA
// ==========================================

async fn seed_demo_empresas(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    let empresas = [
        ("demo-emp-1", "Construcciones del Norte S.A.", "contratista"),
        ("demo-emp-2", "Mantenimiento Industrial Corp", "contratista"),
        ("demo-emp-3", "Servicios Eléctricos Unidos", "contratista"),
        ("demo-emp-4", "Distribuidora de Materiales Ltda", "proveedor"),
        ("demo-emp-5", "Suministros Técnicos S.A.", "proveedor"),
        ("demo-emp-6", "Consultores Ambientales", "visitante"),
    ];

    for (id, nombre, tipo) in empresas {
        sqlx::query(
            r#"INSERT OR IGNORE INTO empresas 
               (id, nombre, tipo, is_active, created_at, updated_at)
               VALUES (?, ?, ?, 1, ?, ?)"#,
        )
        .bind(id)
        .bind(nombre)
        .bind(tipo)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// GAFETES DE PRUEBA
// ==========================================

async fn seed_demo_gafetes(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    // Gafetes de contratista (C-001 a C-020)
    for i in 1..=20 {
        let id = format!("demo-gafete-c-{:03}", i);
        let numero = format!("C-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (id, numero, tipo, estado, created_at, updated_at)
               VALUES (?, ?, 'contratista', 'disponible', ?, ?)"#,
        )
        .bind(&id)
        .bind(&numero)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Gafetes de proveedor (P-001 a P-010)
    for i in 1..=10 {
        let id = format!("demo-gafete-p-{:03}", i);
        let numero = format!("P-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (id, numero, tipo, estado, created_at, updated_at)
               VALUES (?, ?, 'proveedor', 'disponible', ?, ?)"#,
        )
        .bind(&id)
        .bind(&numero)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Gafetes de visita (V-001 a V-010)
    for i in 1..=10 {
        let id = format!("demo-gafete-v-{:03}", i);
        let numero = format!("V-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (id, numero, tipo, estado, created_at, updated_at)
               VALUES (?, ?, 'visita', 'disponible', ?, ?)"#,
        )
        .bind(&id)
        .bind(&numero)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// CONTRATISTAS DE PRUEBA
// ==========================================

async fn seed_demo_contratistas(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();
    let hoy = Utc::now().date_naive();

    // Contratistas con diferentes estados de PRAIND
    let contratistas = [
        // (id, cedula, nombre, apellido, empresa_id, praind_vence_en_dias, estado)
        ("demo-cont-1", "12345678", "Juan", "Pérez", "demo-emp-1", 180, "activo"), // PRAIND OK
        ("demo-cont-2", "23456789", "Pedro", "García", "demo-emp-1", 15, "activo"), // PRAIND por vencer (< 30 días)
        ("demo-cont-3", "34567890", "Luis", "Hernández", "demo-emp-2", -5, "activo"), // PRAIND VENCIDO
        ("demo-cont-4", "45678901", "Miguel", "López", "demo-emp-2", 90, "activo"),   // PRAIND OK
        ("demo-cont-5", "56789012", "Roberto", "Díaz", "demo-emp-3", 60, "suspendido"), // Suspendido
        ("demo-cont-6", "67890123", "Fernando", "Torres", "demo-emp-3", 120, "activo"), // PRAIND OK
        ("demo-cont-7", "78901234", "Andrés", "Ramírez", "demo-emp-1", 45, "activo"),   // PRAIND OK
        ("demo-cont-8", "89012345", "Jorge", "Castillo", "demo-emp-2", 7, "activo"), // PRAIND por vencer (7 días)
    ];

    for (id, cedula, nombre, apellido, empresa_id, praind_dias, estado) in contratistas {
        let praind_fecha = (hoy + Duration::days(praind_dias)).format("%Y-%m-%d").to_string();

        sqlx::query(
            r#"INSERT OR IGNORE INTO contratistas 
               (id, cedula, nombre, apellido, empresa_id, praind_vencimiento, estado, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id)
        .bind(cedula)
        .bind(nombre)
        .bind(apellido)
        .bind(empresa_id)
        .bind(&praind_fecha)
        .bind(estado)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// PROVEEDORES DE PRUEBA
// ==========================================

async fn seed_demo_proveedores(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    let proveedores = [
        ("demo-prov-1", "11111111", "Ricardo", "Mendoza", "demo-emp-4"),
        ("demo-prov-2", "22222222", "Alejandro", "Vargas", "demo-emp-4"),
        ("demo-prov-3", "33333333", "Héctor", "Flores", "demo-emp-5"),
    ];

    for (id, cedula, nombre, apellido, empresa_id) in proveedores {
        sqlx::query(
            r#"INSERT OR IGNORE INTO proveedores 
               (id, cedula, nombre, apellido, empresa_id, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, 1, ?, ?)"#,
        )
        .bind(id)
        .bind(cedula)
        .bind(nombre)
        .bind(apellido)
        .bind(empresa_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// VISITANTES DE PRUEBA
// ==========================================

async fn seed_demo_visitantes(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    let visitantes = [
        ("demo-visit-1", "44444444", "Laura", "Sánchez", Some("demo-emp-6")),
        ("demo-visit-2", "55555555", "Patricia", "Morales", None),
        ("demo-visit-3", "66666666", "Gabriela", "Ortiz", None),
    ];

    for (id, cedula, nombre, apellido, empresa_id) in visitantes {
        sqlx::query(
            r#"INSERT OR IGNORE INTO visitantes 
               (id, cedula, nombre, apellido, empresa_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id)
        .bind(cedula)
        .bind(nombre)
        .bind(apellido)
        .bind(empresa_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ==========================================
// INGRESOS DE CONTRATISTAS (CASOS DE USO)
// ==========================================

async fn seed_demo_ingresos_contratistas(
    pool: &SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let now_str = now.to_rfc3339();

    // ==========================================
    // CASO 1: Ingreso ACTIVO normal (entró hace 2 horas)
    // ==========================================
    let ingreso_2h_ago = (now - Duration::hours(2)).to_rfc3339();
    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-1")
    .bind("demo-cont-1")
    .bind("12345678")
    .bind(&ingreso_2h_ago)
    .bind("C-001")
    .bind("demo-guardia-1")
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    // Marcar gafete como en uso
    sqlx::query("UPDATE gafetes SET estado = 'en_uso' WHERE numero = 'C-001'")
        .execute(pool)
        .await?;

    // ==========================================
    // CASO 2: Ingreso ACTIVO con ALERTA TEMPRANA (13h 35min)
    // ==========================================
    let ingreso_13h = (now - Duration::hours(13) - Duration::minutes(35)).to_rfc3339();
    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-2")
    .bind("demo-cont-4")
    .bind("45678901")
    .bind(&ingreso_13h)
    .bind("C-002")
    .bind("demo-guardia-1")
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    sqlx::query("UPDATE gafetes SET estado = 'en_uso' WHERE numero = 'C-002'")
        .execute(pool)
        .await?;

    // ==========================================
    // CASO 3: Ingreso ACTIVO con TIEMPO EXCEDIDO (15 horas)
    // ==========================================
    let ingreso_15h = (now - Duration::hours(15)).to_rfc3339();
    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-3")
    .bind("demo-cont-6")
    .bind("67890123")
    .bind(&ingreso_15h)
    .bind("C-003")
    .bind("demo-guardia-2")
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    sqlx::query("UPDATE gafetes SET estado = 'en_uso' WHERE numero = 'C-003'")
        .execute(pool)
        .await?;

    // ==========================================
    // CASO 4: Ingreso COMPLETADO (historial)
    // ==========================================
    let ingreso_ayer_8am =
        (now - Duration::days(1)).date_naive().and_hms_opt(8, 0, 0).unwrap().and_utc().to_rfc3339();
    let salida_ayer_5pm = (now - Duration::days(1))
        .date_naive()
        .and_hms_opt(17, 0, 0)
        .unwrap()
        .and_utc()
        .to_rfc3339();

    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, fecha_hora_ingreso, fecha_hora_salida, gafete_numero, usuario_ingreso_id, usuario_salida_id, tiempo_permanencia_minutos, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-4")
    .bind("demo-cont-7")
    .bind("78901234")
    .bind(&ingreso_ayer_8am)
    .bind(&salida_ayer_5pm)
    .bind("C-004")
    .bind("demo-guardia-1")
    .bind("demo-guardia-2")
    .bind(540) // 9 horas = 540 minutos
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    Ok(())
}

// ==========================================
// LISTA NEGRA DE PRUEBA
// ==========================================

async fn seed_demo_lista_negra(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    // Persona bloqueada
    sqlx::query(
        r#"INSERT OR IGNORE INTO lista_negra 
           (id, cedula, nivel_severidad, motivo_bloqueo, descripcion, is_active, created_at, updated_at, created_by)
           VALUES (?, ?, ?, ?, ?, 1, ?, ?, ?)"#,
    )
    .bind("demo-bloqueo-1")
    .bind("99999999")
    .bind("alto")
    .bind("Robo de herramientas")
    .bind("Se detectó sustrayendo herramientas del almacén el 15/12/2024")
    .bind(&now)
    .bind(&now)
    .bind("demo-supervisor-1")
    .execute(pool)
    .await?;

    // Contratista bloqueado (demo-cont-5 está suspendido, pero también bloqueado)
    sqlx::query(
        r#"INSERT OR IGNORE INTO lista_negra 
           (id, cedula, nivel_severidad, motivo_bloqueo, descripcion, is_active, created_at, updated_at, created_by)
           VALUES (?, ?, ?, ?, ?, 1, ?, ?, ?)"#,
    )
    .bind("demo-bloqueo-2")
    .bind("56789012") // cedula de demo-cont-5
    .bind("medio")
    .bind("Incumplimiento de normas de seguridad")
    .bind("No utilizó EPP requerido en área de alto riesgo")
    .bind(&now)
    .bind(&now)
    .bind("demo-supervisor-1")
    .execute(pool)
    .await?;

    Ok(())
}

// ==========================================
// ALERTAS DE GAFETE (CASOS DE USO)
// ==========================================

async fn seed_demo_alertas_gafete(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    // ==========================================
    // CASO 1: Contratista con 1 gafete pendiente (WARNING, puede entrar)
    // Este contratista (demo-cont-7) salió ayer pero no devolvió su gafete
    // ==========================================
    sqlx::query(
        r#"INSERT OR IGNORE INTO alertas_gafetes 
           (id, cedula, gafete_numero, ingreso_contratista_id, resuelto, created_at, updated_at)
           VALUES (?, ?, ?, ?, 0, ?, ?)"#,
    )
    .bind("demo-alerta-1")
    .bind("78901234") // cedula de demo-cont-7
    .bind("C-004")
    .bind("demo-ingreso-4")
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    // ==========================================
    // CASO 2: Contratista con 2 gafetes pendientes (BLOQUEADO, no puede entrar)
    // Este contratista tiene 2 alertas sin resolver
    // ==========================================
    sqlx::query(
        r#"INSERT OR IGNORE INTO alertas_gafetes 
           (id, cedula, gafete_numero, ingreso_contratista_id, resuelto, created_at, updated_at)
           VALUES (?, ?, ?, NULL, 0, ?, ?)"#,
    )
    .bind("demo-alerta-2")
    .bind("89012345") // cedula de demo-cont-8 (PRAIND por vencer 7 días)
    .bind("C-010")
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"INSERT OR IGNORE INTO alertas_gafetes 
           (id, cedula, gafete_numero, ingreso_contratista_id, resuelto, created_at, updated_at)
           VALUES (?, ?, ?, NULL, 0, ?, ?)"#,
    )
    .bind("demo-alerta-3")
    .bind("89012345") // mismo contratista, segunda alerta
    .bind("C-011")
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}
