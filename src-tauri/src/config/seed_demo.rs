// src/config/seed_demo.rs
//
// ==========================================
// SEED DE DEMOSTRACIÓN PARA SURREALDB
// ==========================================
// Este módulo genera datos mockeados para demostrar
// todas las funcionalidades del sistema.
//
// USO: Solo se ejecuta bajo demanda (Modo Demo)
// NO se ejecuta en startup normal.

use chrono::{Duration, Utc};
use log::{debug, info};

use crate::domain::role::{ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID};
use crate::services::auth::hash_password;
use crate::services::surrealdb_service::{get_db, SurrealDbError};

/// Ejecuta todos los seeds de demostración
pub async fn run_demo_seed() -> Result<(), Box<dyn std::error::Error>> {
    debug!("🌱 Iniciando seeds de demo en SurrealDB");

    debug!("🌱 Seeding users...");
    seed_demo_users().await?;

    debug!("🌱 Seeding empresas...");
    seed_demo_empresas().await?;

    debug!("🌱 Seeding gafetes...");
    seed_demo_gafetes().await?;

    debug!("🌱 Seeding contratistas...");
    seed_demo_contratistas().await?;

    debug!("🌱 Seeding proveedores...");
    seed_demo_proveedores().await?;

    debug!("🌱 Seeding visitantes...");
    seed_demo_visitantes().await?;

    debug!("🌱 Seeding vehiculos...");
    seed_demo_vehiculos().await?;

    debug!("🌱 Seeding lista negra...");
    seed_demo_lista_negra().await?;

    info!("✅ Todos los seeds de demo completados exitosamente.");
    Ok(())
}

// ==========================================
// USUARIOS DE PRUEBA
// ==========================================

async fn seed_demo_users() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let password_hash =
        hash_password("demo123").map_err(|e| SurrealDbError::Query(e.to_string()))?;

    let users = [
        (
            "demo-supervisor-1",
            "marie.curie@demo.com",
            "Marie",
            "Curie",
            ROLE_SUPERVISOR_ID,
            "10000001",
        ),
        (
            "demo-admin-1",
            "albert.einstein@demo.com",
            "Albert",
            "Einstein",
            ROLE_ADMIN_ID,
            "10000002",
        ),
        (
            "demo-guardia-2",
            "richard.feynman@demo.com",
            "Richard",
            "Feynman",
            ROLE_GUARDIA_ID,
            "10000003",
        ),
    ];

    for (id, email, nombre, apellido, role_id, cedula) in users {
        db.query(
            r#"
                UPSERT user CONTENT {
                    id: type::thing('user', $id),
                    email: $email,
                    password_hash: $password_hash,
                    nombre: $nombre,
                    apellido: $apellido,
                    role: type::thing('role', $role_id),
                    is_active: true,
                    cedula: $cedula,
                    must_change_password: false,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
        )
        .bind(("id", id))
        .bind(("email", email))
        .bind(("password_hash", password_hash.clone()))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("role_id", role_id))
        .bind(("cedula", cedula))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// EMPRESAS DE PRUEBA
// ==========================================

async fn seed_demo_empresas() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let empresas = [
        ("demo-emp-1", "Bell Labs"),
        ("demo-emp-2", "Xerox PARC"),
        ("demo-emp-3", "IBM Research"),
        ("demo-emp-4", "Oracle Systems"),
        ("demo-emp-5", "Red Hat"),
        ("demo-emp-6", "CERN Computing"),
    ];

    for (id, nombre) in empresas {
        db.query(
            r#"
                UPSERT empresa SET
                    id = $id,
                    nombre = $nombre,
                    is_active = true,
                    created_at = $now,
                    updated_at = $now
                WHERE id = $id
                "#,
        )
        .bind(("id", id))
        .bind(("nombre", nombre))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// GAFETES DE PRUEBA
// ==========================================

async fn seed_demo_gafetes() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let now = Utc::now().to_rfc3339();

    // Gafetes de contratista (01 a 20)
    for i in 1..=20 {
        let numero = format!("{:02}", i);
        db.query(
            r#"
                UPSERT gafete SET
                    numero = $numero,
                    tipo = "contratista",
                    estado = "activo",
                    en_uso = false,
                    created_at = $now,
                    updated_at = $now
                WHERE numero = $numero AND tipo = "contratista"
                "#,
        )
        .bind(("numero", numero.clone()))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    // Gafetes de proveedor (01 a 10)
    for i in 1..=10 {
        let numero = format!("{:02}", i);
        db.query(
            r#"
                UPSERT gafete SET
                    numero = $numero,
                    tipo = "proveedor",
                    estado = "activo",
                    en_uso = false,
                    created_at = $now,
                    updated_at = $now
                WHERE numero = $numero AND tipo = "proveedor"
                "#,
        )
        .bind(("numero", numero.clone()))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    // Gafetes de visita (01 a 10)
    for i in 1..=10 {
        let numero = format!("{:02}", i);
        db.query(
            r#"
                UPSERT gafete SET
                    numero = $numero,
                    tipo = "visita",
                    estado = "activo",
                    en_uso = false,
                    created_at = $now,
                    updated_at = $now
                WHERE numero = $numero AND tipo = "visita"
                "#,
        )
        .bind(("numero", numero.clone()))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// CONTRATISTAS DE PRUEBA
// ==========================================

async fn seed_demo_contratistas() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let hoy = Utc::now().date_naive();

    let contratistas = [
        ("demo-cont-1", "17040100", "Isaac", "Newton", "demo-emp-1", 180, "activo"),
        ("demo-cont-2", "18560700", "Nikola", "Tesla", "demo-emp-1", 15, "activo"),
        ("demo-cont-3", "19420100", "Stephen", "Hawking", "demo-emp-2", -5, "activo"),
        ("demo-cont-4", "18850100", "Niels", "Bohr", "demo-emp-2", 90, "activo"),
        ("demo-cont-5", "19010100", "Werner", "Heisenberg", "demo-emp-3", 60, "suspendido"),
        ("demo-cont-6", "19000100", "Enrico", "Fermi", "demo-emp-3", 120, "activo"),
        ("demo-cont-7", "18670100", "Max", "Planck", "demo-emp-1", 45, "activo"),
        ("demo-cont-8", "18870100", "Erwin", "Schrödinger", "demo-emp-2", 7, "activo"),
    ];

    for (id, cedula, nombre, apellido, empresa_id, praind_dias, estado) in contratistas {
        let praind_fecha = (hoy + Duration::days(praind_dias)).format("%Y-%m-%d").to_string();

        db.query(
            r#"
                UPSERT contratista CONTENT {
                    id: type::thing('contratista', $id),
                    cedula: $cedula,
                    nombre: $nombre,
                    apellido: $apellido,
                    empresa: type::thing('empresa', $empresa_id),
                    fecha_vencimiento_praind: $praind_fecha,
                    estado: $estado,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
        )
        .bind(("id", id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("empresa_id", empresa_id))
        .bind(("praind_fecha", praind_fecha.clone()))
        .bind(("estado", estado))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// PROVEEDORES DE PRUEBA
// ==========================================

async fn seed_demo_proveedores() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    let proveedores = [
        ("demo-prov-1", "17770100", "Carl", "Gauss", "demo-emp-4"),
        ("demo-prov-2", "17070100", "Leonhard", "Euler", "demo-emp-4"),
        ("demo-prov-3", "18260100", "Bernhard", "Riemann", "demo-emp-5"),
        ("demo-prov-4", "16010100", "Pierre", "Fermat", "demo-emp-4"),
        ("demo-prov-5", "17890100", "Augustin", "Cauchy", "demo-emp-5"),
        ("demo-prov-6", "17490100", "Joseph", "Lagrange", "demo-emp-4"),
        ("demo-prov-7", "18450100", "Georg", "Cantor", "demo-emp-5"),
        ("demo-prov-8", "18620100", "David", "Hilbert", "demo-emp-4"),
    ];

    for (id, cedula, nombre, apellido, empresa_id) in proveedores {
        db.query(
            r#"
                UPSERT proveedor CONTENT {
                    id: type::thing('proveedor', $id),
                    cedula: $cedula,
                    nombre: $nombre,
                    apellido: $apellido,
                    empresa: type::thing('empresa', $empresa_id),
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
        )
        .bind(("id", id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("empresa_id", empresa_id))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// VISITANTES DE PRUEBA
// ==========================================

async fn seed_demo_visitantes() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let visitantes = [
        ("demo-visit-1", "18150100", "Ada", "Lovelace"),
        ("demo-visit-2", "19060100", "Grace", "Hopper"),
        ("demo-visit-3", "19690100", "Linus", "Torvalds"),
        ("demo-visit-4", "19120100", "Alan", "Turing"),
        ("demo-visit-5", "19420100", "Dennis", "Ritchie"),
        ("demo-visit-6", "19400100", "Ken", "Thompson"),
        ("demo-visit-7", "19500100", "Bjarne", "Stroustrup"),
        ("demo-visit-8", "19550100", "James", "Gosling"),
    ];

    for (id, cedula, nombre, apellido) in visitantes {
        db.query(
            r#"
                UPSERT visitante SET
                    id = $id,
                    cedula = $cedula,
                    nombre = $nombre,
                    apellido = $apellido,
                    created_at = $now,
                    updated_at = $now
                WHERE id = $id
                "#,
        )
        .bind(("id", id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    Ok(())
}

// ==========================================
// VEHÍCULOS DE PRUEBA
// ==========================================

async fn seed_demo_vehiculos() -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    let vehiculos: Vec<(&str, Option<&str>, Option<&str>, &str, &str, &str, &str, &str)> = vec![
        (
            "demo-veh-1",
            Some("demo-cont-1"),
            None,
            "automovil",
            "MKB-123",
            "Tesla",
            "Model S",
            "Rojo",
        ),
        (
            "demo-veh-2",
            Some("demo-cont-4"),
            None,
            "motocicleta",
            "XYZ-101",
            "Yamaha",
            "MT-07",
            "Negro",
        ),
        (
            "demo-veh-3",
            Some("demo-cont-6"),
            None,
            "automovil",
            "ABC-456",
            "Toyota",
            "Hilux",
            "Blanco",
        ),
        (
            "demo-veh-4",
            Some("demo-cont-2"),
            None,
            "automovil",
            "FUT-777",
            "Ford",
            "Mustang",
            "Gris",
        ),
        ("demo-veh-9", None, Some("demo-prov-1"), "automovil", "BMW-001", "BMW", "M3", "Negro"),
        (
            "demo-veh-10",
            None,
            Some("demo-prov-2"),
            "motocicleta",
            "DUC-999",
            "Ducati",
            "Panigale V4",
            "Rojo",
        ),
    ];

    for (id, contratista_id, proveedor_id, tipo, placa, marca, modelo, color) in vehiculos {
        db.query(
                r#"
                UPSERT vehiculo CONTENT {
                    id: type::thing('vehiculo', $id),
                    contratista: IF $contratista_id != NONE THEN type::thing('contratista', $contratista_id) ELSE NONE END,
                    proveedor: IF $proveedor_id != NONE THEN type::thing('proveedor', $proveedor_id) ELSE NONE END,
                    tipo_vehiculo: $tipo,
                    placa: $placa,
                    marca: $marca,
                    modelo: $modelo,
                    color: $color,
                    is_active: true,
                    created_at: time::now(),
                    updated_at: time::now()
                }
                "#,
            )
            .bind(("id", id))
            .bind(("contratista_id", contratista_id))
            .bind(("proveedor_id", proveedor_id))
            .bind(("tipo", tipo))
            .bind(("placa", placa))
            .bind(("marca", marca))
            .bind(("modelo", modelo))
            .bind(("color", color))
            .await?
            .check()?;
    }

    Ok(())
}

// ==========================================
// LISTA NEGRA DE PRUEBA
// ==========================================

async fn seed_demo_lista_negra() -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let now = Utc::now().to_rfc3339();

    let bloqueados = [
        (
            "demo-bloqueo-1",
            "99999999",
            "Kevin",
            "Mitnick",
            "ALTO",
            "Robo de herramientas",
            "demo-supervisor-1",
        ),
        (
            "demo-bloqueo-2",
            "19010100",
            "Werner",
            "Heisenberg",
            "MEDIO",
            "Incumplimiento de normas de seguridad",
            "demo-supervisor-1",
        ),
    ];

    for (id, cedula, nombre, apellido, nivel, motivo, bloqueado_por) in bloqueados {
        db.query(
            r#"
                UPSERT lista_negra SET
                    id = $id,
                    cedula = $cedula,
                    nombre = $nombre,
                    apellido = $apellido,
                    nivel_severidad = $nivel,
                    motivo_bloqueo = $motivo,
                    bloqueado_por = $bloqueado_por,
                    is_active = true,
                    created_at = $now,
                    updated_at = $now
                WHERE id = $id
                "#,
        )
        .bind(("id", id))
        .bind(("cedula", cedula))
        .bind(("nombre", nombre))
        .bind(("apellido", apellido))
        .bind(("nivel", nivel))
        .bind(("motivo", motivo))
        .bind(("bloqueado_por", bloqueado_por))
        .bind(("now", now.clone()))
        .await?
        .check()?;
    }

    Ok(())
}
