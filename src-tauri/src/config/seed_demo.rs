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

use crate::domain::role::{ROLE_ADMIN_ID, ROLE_GUARDIA_ID, ROLE_SUPERVISOR_ID};
use crate::services::auth::hash_password;

/// Ejecuta todos los seeds de demostración
pub async fn run_demo_seed(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // 🛡️ SAFETY CHECK: Verificar que estamos en la DB de demo
    let db_path: String =
        sqlx::query_scalar("SELECT file FROM pragma_database_list WHERE name='main'")
            .fetch_one(pool)
            .await?;

    if !db_path.contains("brisas_demo.db") {
        let error_msg = format!(
            "⛔ CRITICAL: Intento de correr seeds demo en base de datos de producción: {}",
            db_path
        );
        log::error!("{}", error_msg);
        return Err(error_msg.into());
    }

    log::debug!("🌱 Iniciando seeds de demo en: {}", db_path);

    log::debug!("🌱 Seeding users...");
    seed_demo_users(pool).await?;

    log::debug!("🌱 Seeding empresas...");
    seed_demo_empresas(pool).await?;

    log::debug!("🌱 Seeding gafetes...");
    seed_demo_gafetes(pool).await?;

    log::debug!("🌱 Seeding contratistas...");
    seed_demo_contratistas(pool).await?;

    log::debug!("🌱 Seeding proveedores...");
    seed_demo_proveedores(pool).await?;

    log::debug!("🌱 Seeding visitantes...");
    seed_demo_visitantes(pool).await?;

    // IMPORTANTE: Vehículos antes que ingresos para evitar FK constraint
    log::debug!("🌱 Seeding vehiculos...");
    seed_demo_vehiculos(pool).await?;

    log::debug!("🌱 Seeding ingresos...");
    seed_demo_ingresos_contratistas(pool).await?;

    log::debug!("🌱 Seeding alertas...");
    seed_demo_alertas_gafete(pool).await?;

    log::debug!("🌱 Seeding lista negra...");
    seed_demo_lista_negra(pool).await?;

    log::info!("✅ Todos los seeds de demo completados exitosamente.");
    Ok(())
}

// ==========================================
// USUARIOS DE PRUEBA
// ==========================================

async fn seed_demo_users(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password("demo123")?;

    let users = [
        // Supervisora: Marie Curie (primera mujer en ganar un Nobel)
        (
            "demo-supervisor-1",
            "marie.curie@demo.com",
            "Marie",
            "Curie",
            ROLE_SUPERVISOR_ID,
            "MC123456",
        ),
        // Admin: Albert Einstein (padre de la relatividad, merece ser admin)
        (
            "demo-admin-1",
            "albert.einstein@demo.com",
            "Albert",
            "Einstein",
            ROLE_ADMIN_ID,
            "AE789012",
        ),
        // Guardia 2: Richard Feynman
        (
            "demo-guardia-2",
            "richard.feynman@demo.com",
            "Richard",
            "Feynman",
            ROLE_GUARDIA_ID,
            "RF345678",
        ),
    ];

    for (id, email, nombre, apellido, role_id, cedula) in users {
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
        .bind(cedula)
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

    // Empresas: Compañías legendarias del mundo tech
    let empresas = [
        ("demo-emp-1", "Bell Labs"),      // Donde se inventó Unix
        ("demo-emp-2", "Xerox PARC"),     // GUI, mouse, ethernet
        ("demo-emp-3", "IBM Research"),   // Mainframes legendarios
        ("demo-emp-4", "Oracle Systems"), // Bases de datos
        ("demo-emp-5", "Red Hat"),        // Linux empresarial
        ("demo-emp-6", "CERN Computing"), // Donde nació la web
    ];

    for (id, nombre) in empresas {
        sqlx::query(
            r#"INSERT OR IGNORE INTO empresas 
               (id, nombre, is_active, created_at, updated_at)
               VALUES (?, ?, 1, ?, ?)"#,
        )
        .bind(id)
        .bind(nombre)
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
    // Nota: PK es (numero, tipo), no hay columna 'id'
    // Estado válido: 'activo', 'danado', 'extraviado'
    for i in 1..=20 {
        let numero = format!("C-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (numero, tipo, estado, created_at, updated_at)
               VALUES (?, 'contratista', 'activo', ?, ?)"#,
        )
        .bind(&numero)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Gafetes de proveedor (P-001 a P-010)
    for i in 1..=10 {
        let numero = format!("P-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (numero, tipo, estado, created_at, updated_at)
               VALUES (?, 'proveedor', 'activo', ?, ?)"#,
        )
        .bind(&numero)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Gafetes de visita (V-001 a V-010)
    for i in 1..=10 {
        let numero = format!("V-{:03}", i);
        sqlx::query(
            r#"INSERT OR IGNORE INTO gafetes 
               (numero, tipo, estado, created_at, updated_at)
               VALUES (?, 'visita', 'activo', ?, ?)"#,
        )
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

    // Contratistas: Físicos famosos con diferentes estados de PRAIND
    let contratistas = [
        // (id, cedula, nombre, apellido, empresa_id, praind_vence_en_dias, estado)
        ("demo-cont-1", "IN170401", "Isaac", "Newton", "demo-emp-1", 180, "activo"), // PRAIND OK - Padre de la física clásica
        ("demo-cont-2", "NT185607", "Nikola", "Tesla", "demo-emp-1", 15, "activo"), // PRAIND por vencer - Genio de la electricidad
        ("demo-cont-3", "SH194201", "Stephen", "Hawking", "demo-emp-2", -5, "activo"), // PRAIND VENCIDO - Cosmólogo
        ("demo-cont-4", "NB188501", "Niels", "Bohr", "demo-emp-2", 90, "activo"), // PRAIND OK - Física cuántica
        ("demo-cont-5", "WH190101", "Werner", "Heisenberg", "demo-emp-3", 60, "suspendido"), // Suspendido - Principio de incertidumbre
        ("demo-cont-6", "EP190001", "Enrico", "Fermi", "demo-emp-3", 120, "activo"), // PRAIND OK - Física nuclear
        ("demo-cont-7", "MD186701", "Max", "Planck", "demo-emp-1", 45, "activo"), // PRAIND OK - Padre de la cuántica
        ("demo-cont-8", "ES188701", "Erwin", "Schrödinger", "demo-emp-2", 7, "activo"), // PRAIND por vencer - El gato
    ];

    for (id, cedula, nombre, apellido, empresa_id, praind_dias, estado) in contratistas {
        let praind_fecha = (hoy + Duration::days(praind_dias)).format("%Y-%m-%d").to_string();

        sqlx::query(
            r#"INSERT OR IGNORE INTO contratistas 
               (id, cedula, nombre, apellido, empresa_id, fecha_vencimiento_praind, estado, created_at, updated_at)
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

    // Proveedores: Matemáticos legendarios
    let proveedores = [
        ("demo-prov-1", "CG177701", "Carl", "Gauss", "demo-emp-4"), // Príncipe de las matemáticas
        ("demo-prov-2", "LE170701", "Leonhard", "Euler", "demo-emp-4"), // El más prolífico de la historia
        ("demo-prov-3", "BR182601", "Bernhard", "Riemann", "demo-emp-5"), // Hipótesis de Riemann
        ("demo-prov-4", "PF160101", "Pierre", "Fermat", "demo-emp-4"),  // Último teorema de Fermat
        ("demo-prov-5", "AG178901", "Augustin", "Cauchy", "demo-emp-5"), // Análisis complejo
        ("demo-prov-6", "JL174901", "Joseph", "Lagrange", "demo-emp-4"), // Mecánica analítica
        ("demo-prov-7", "GC184501", "Georg", "Cantor", "demo-emp-5"),   // Teoría de conjuntos
        ("demo-prov-8", "DP186201", "David", "Hilbert", "demo-emp-4"),  // 23 problemas de Hilbert
    ];

    for (id, cedula, nombre, apellido, empresa_id) in proveedores {
        sqlx::query(
            r#"INSERT OR IGNORE INTO proveedores 
               (id, cedula, nombre, apellido, empresa_id, estado, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, 'activo', ?, ?)"#,
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

    // Visitantes: Leyendas de la programación y ciencias de la computación
    let visitantes = [
        ("demo-visit-1", "AL181501", "Ada", "Lovelace", Some("demo-emp-6")), // Primera programadora de la historia
        ("demo-visit-2", "GH190601", "Grace", "Hopper", None), // COBOL, el primer "bug"
        ("demo-visit-3", "LT196901", "Linus", "Torvalds", None), // Creador de Linux y Git
        ("demo-visit-4", "AT191201", "Alan", "Turing", Some("demo-emp-6")), // Padre de la computación
        ("demo-visit-5", "DJ194201", "Dennis", "Ritchie", None),            // Creador de C y Unix
        ("demo-visit-6", "KB194001", "Ken", "Thompson", None), // Co-creador de Unix y Go
        ("demo-visit-7", "BS195001", "Bjarne", "Stroustrup", Some("demo-emp-6")), // Creador de C++
        ("demo-visit-8", "JG195501", "James", "Gosling", None), // Padre de Java
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
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre, tipo_autorizacion, modo_ingreso, vehiculo_id, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, 'praind', 'vehiculo', ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-1")
    .bind("demo-cont-1")
    .bind("12345678")
    .bind("Isaac")
    .bind("Newton")
    .bind("Bell Labs")
    .bind("demo-veh-1")
    .bind(&ingreso_2h_ago)
    .bind("C-001")
    .bind("demo-guardia-2")
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    // ==========================================
    // CASO 2: Ingreso ACTIVO con ALERTA TEMPRANA (13h 35min)
    // ==========================================
    let ingreso_13h = (now - Duration::hours(13) - Duration::minutes(35)).to_rfc3339();
    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre, tipo_autorizacion, modo_ingreso, vehiculo_id, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, 'praind', 'vehiculo', ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-2")
    .bind("demo-cont-4")
    .bind("45678901")
    .bind("Niels")
    .bind("Bohr")
    .bind("Xerox PARC")
    .bind("demo-veh-2")
    .bind(&ingreso_13h)
    .bind("C-002")
    .bind("demo-guardia-2")
    .bind(&now_str)
    .bind(&now_str)
    .execute(pool)
    .await?;

    // ==========================================
    // CASO 3: Ingreso ACTIVO con TIEMPO EXCEDIDO (15 horas)
    // ==========================================
    let ingreso_15h = (now - Duration::hours(15)).to_rfc3339();
    sqlx::query(
        r#"INSERT OR IGNORE INTO ingresos_contratistas 
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre, tipo_autorizacion, modo_ingreso, fecha_hora_ingreso, gafete_numero, usuario_ingreso_id, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, 'praind', 'caminando', ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-3")
    .bind("demo-cont-6")
    .bind("67890123")
    .bind("Enrico")
    .bind("Fermi")
    .bind("IBM Research")
    .bind(&ingreso_15h)
    .bind("C-003")
    .bind("demo-guardia-2")
    .bind(&now_str)
    .bind(&now_str)
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
           (id, contratista_id, cedula, nombre, apellido, empresa_nombre, tipo_autorizacion, modo_ingreso, fecha_hora_ingreso, fecha_hora_salida, gafete_numero, usuario_ingreso_id, usuario_salida_id, tiempo_permanencia_minutos, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, 'praind', 'caminando', ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind("demo-ingreso-4")
    .bind("demo-cont-7")
    .bind("78901234")
    .bind("Max")
    .bind("Planck")
    .bind("Bell Labs")
    .bind(&ingreso_ayer_8am)
    .bind(&salida_ayer_5pm)
    .bind("C-004")
    .bind("demo-guardia-2")
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
    let exists_1: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM lista_negra WHERE id = ?)")
            .bind("demo-bloqueo-1")
            .fetch_one(pool)
            .await?;

    if !exists_1 {
        sqlx::query(
            r#"INSERT INTO lista_negra 
            (id, cedula, nombre, apellido, nivel_severidad, motivo_bloqueo, bloqueado_por, observaciones, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        )
        .bind("demo-bloqueo-1")
        .bind("99999999")
        .bind("Kevin")
        .bind("Mitnick")
        .bind("ALTO")
        .bind("Robo de herramientas")
        .bind("demo-supervisor-1")
        .bind("Se detectó sustrayendo herramientas del almacén el 15/12/2024")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Contratista bloqueado
    let exists_2: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM lista_negra WHERE id = ?)")
            .bind("demo-bloqueo-2")
            .fetch_one(pool)
            .await?;

    if !exists_2 {
        sqlx::query(
            r#"INSERT INTO lista_negra 
            (id, cedula, nombre, apellido, nivel_severidad, motivo_bloqueo, bloqueado_por, observaciones, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        )
        .bind("demo-bloqueo-2")
        .bind("56789012")
        .bind("Werner")
        .bind("Heisenberg")
        .bind("MEDIO")
        .bind("Incumplimiento de normas de seguridad")
        .bind("demo-supervisor-1")
        .bind("No utilizó EPP requerido en área de alto riesgo")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

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
           (id, cedula, nombre_completo, gafete_numero, ingreso_contratista_id, resuelto, fecha_reporte, reportado_por, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, ?)"#,
    )
    .bind("demo-alerta-1")
    .bind("78901234") // cedula de demo-cont-7
    .bind("Max Planck")
    .bind("C-004")
    .bind("demo-ingreso-4")
    .bind(&now)
    .bind("demo-admin-1")
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
           (id, cedula, nombre_completo, gafete_numero, ingreso_contratista_id, resuelto, fecha_reporte, reportado_por, created_at, updated_at)
           VALUES (?, ?, ?, ?, NULL, 0, ?, ?, ?, ?)"#,
    )
    .bind("demo-alerta-2")
    .bind("89012345") // cedula de demo-cont-8 (PRAIND por vencer 7 días)
    .bind("Erwin Schrödinger")
    .bind("C-010")
    .bind(&now)
    .bind("demo-admin-1")
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"INSERT OR IGNORE INTO alertas_gafetes 
           (id, cedula, nombre_completo, gafete_numero, ingreso_contratista_id, resuelto, fecha_reporte, reportado_por, created_at, updated_at)
           VALUES (?, ?, ?, ?, NULL, 0, ?, ?, ?, ?)"#,
    )
    .bind("demo-alerta-3")
    .bind("89012345") // mismo contratista, segunda alerta
    .bind("Erwin Schrödinger")
    .bind("C-011")
    .bind(&now)
    .bind("demo-admin-1")
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}

// ==========================================
// VEHÍCULOS DE PRUEBA
// ==========================================

async fn seed_demo_vehiculos(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().to_rfc3339();

    // Estructura: (id, contratista_id, proveedor_id, visitante_id, tipo, placa, marca, modelo, color)
    let vehiculos = [
        // --- Contratistas (Isaac Newton, Niels Bohr, etc.) ---
        (
            "demo-veh-1",
            Some("demo-cont-1"),
            None,
            None,
            "automovil",
            "MKB-123",
            "Tesla",
            "Model S",
            "Rojo",
        ),
        (
            "demo-veh-1b",
            Some("demo-cont-1"),
            None,
            None,
            "motocicleta",
            "MKB-123M",
            "BMW",
            "S1000RR",
            "Blanco/Azul",
        ),
        (
            "demo-veh-2",
            Some("demo-cont-4"),
            None,
            None,
            "motocicleta",
            "XYZ-101",
            "Yamaha",
            "MT-07",
            "Negro",
        ),
        (
            "demo-veh-2b",
            Some("demo-cont-4"),
            None,
            None,
            "automovil",
            "XYZ-101A",
            "Toyota",
            "Corolla",
            "Gris",
        ),
        (
            "demo-veh-3",
            Some("demo-cont-6"),
            None,
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
            None,
            "automovil",
            "FUT-777",
            "Ford",
            "Mustang",
            "Gris",
        ),
        (
            "demo-veh-5",
            Some("demo-cont-3"),
            None,
            None,
            "motocicleta",
            "HAR-888",
            "Harley Davidson",
            "Iron 883",
            "Mate",
        ),
        (
            "demo-veh-6",
            Some("demo-cont-7"),
            None,
            None,
            "automovil",
            "HON-999",
            "Honda",
            "Civic",
            "Azul",
        ),
        (
            "demo-veh-7",
            Some("demo-cont-8"),
            None,
            None,
            "motocicleta",
            "VES-202",
            "Vespa",
            "Primavera",
            "Amarillo",
        ),
        (
            "demo-veh-8",
            Some("demo-cont-5"),
            None,
            None,
            "automovil",
            "CHE-555",
            "Chevrolet",
            "Silverado",
            "Verde",
        ),
        // --- Proveedores ---
        (
            "demo-veh-9",
            None,
            Some("demo-prov-1"),
            None,
            "automovil",
            "BMW-001",
            "BMW",
            "M3",
            "Negro",
        ),
        (
            "demo-veh-10",
            None,
            Some("demo-prov-2"),
            None,
            "motocicleta",
            "DUC-999",
            "Ducati",
            "Panigale V4",
            "Rojo",
        ),
        (
            "demo-veh-10b",
            None,
            Some("demo-prov-2"),
            None,
            "automovil",
            "DUC-999A",
            "Audi",
            "RS6",
            "Negro Mate",
        ),
        (
            "demo-veh-11",
            None,
            Some("demo-prov-3"),
            None,
            "automovil",
            "POR-911",
            "Porsche",
            "911 GT3",
            "Plateado",
        ),
        (
            "demo-veh-12",
            None,
            Some("demo-prov-4"),
            None,
            "motocicleta",
            "KAW-636",
            "Kawasaki",
            "Ninja ZX-6R",
            "Verde Kawa",
        ),
        (
            "demo-veh-13",
            None,
            Some("demo-prov-5"),
            None,
            "automovil",
            "HND-202",
            "Honda",
            "CR-V",
            "Blanco",
        ),
        (
            "demo-veh-18",
            None,
            Some("demo-prov-6"),
            None,
            "motocicleta",
            "TRI-333",
            "Triumph",
            "Tiger 900",
            "Naranja",
        ),
        (
            "demo-veh-19",
            None,
            Some("demo-prov-7"),
            None,
            "motocicleta",
            "KTM-129",
            "KTM",
            "SuperDuke 1290",
            "Naranja/Negro",
        ),
        (
            "demo-veh-23",
            None,
            Some("demo-prov-1"),
            None,
            "motocicleta",
            "BMW-S1K",
            "BMW",
            "S1000XR",
            "Rojo",
        ),
        // --- Visitantes ---
        (
            "demo-veh-14",
            None,
            None,
            Some("demo-visit-1"),
            "automovil",
            "AUD-101",
            "Audi",
            "A4",
            "Gris",
        ),
        (
            "demo-veh-14b",
            None,
            None,
            Some("demo-visit-1"),
            "motocicleta",
            "AUD-101M",
            "Ducati",
            "Monster",
            "Amarillo",
        ),
        (
            "demo-veh-15",
            None,
            None,
            Some("demo-visit-2"),
            "motocicleta",
            "SUZ-1300",
            "Suzuki",
            "Hayabusa",
            "Azul",
        ),
        (
            "demo-veh-16",
            None,
            None,
            Some("demo-visit-3"),
            "automovil",
            "MBZ-500",
            "Mercedes-Benz",
            "E-Class",
            "Negro",
        ),
        (
            "demo-veh-17",
            None,
            None,
            Some("demo-visit-4"),
            "motocicleta",
            "NIZ-400",
            "Nissan",
            "Frontier",
            "Naranja",
        ),
        (
            "demo-veh-20",
            None,
            None,
            Some("demo-visit-5"),
            "motocicleta",
            "BMW-GS",
            "BMW",
            "R1250 GS",
            "Blanco/Azul",
        ),
        (
            "demo-veh-21",
            None,
            None,
            Some("demo-visit-6"),
            "automovil",
            "SUB-WRX",
            "Subaru",
            "WRX STI",
            "Azul Rally",
        ),
        (
            "demo-veh-22",
            None,
            None,
            Some("demo-visit-7"),
            "motocicleta",
            "HND-CBR",
            "Honda",
            "CBR-1000RR-R",
            "HRC Colors",
        ),
        (
            "demo-veh-24",
            None,
            None,
            Some("demo-visit-2"),
            "automovil",
            "SUZ-VIT",
            "Suzuki",
            "Vitara",
            "Rojo",
        ),
        (
            "demo-veh-25",
            None,
            None,
            Some("demo-visit-3"),
            "motocicleta",
            "YAM-R1",
            "Yamaha",
            "YZF-R1",
            "Petronas",
        ),
        (
            "demo-veh-26",
            None,
            None,
            Some("demo-visit-4"),
            "automovil",
            "NIZ-Z",
            "Nissan",
            "400Z",
            "Amarillo",
        ),
        (
            "demo-veh-27",
            None,
            None,
            Some("demo-visit-5"),
            "automovil",
            "BMW-X5",
            "BMW",
            "X5 M",
            "Blanco",
        ),
        (
            "demo-veh-28",
            None,
            None,
            Some("demo-visit-6"),
            "motocicleta",
            "KTM-ADV",
            "KTM",
            "1290 Super Adventure",
            "Blanco",
        ),
    ];

    for (id, cont_id, prov_id, visit_id, tipo, placa, marca, modelo, color) in vehiculos {
        sqlx::query(
            r#"INSERT OR IGNORE INTO vehiculos 
               (id, contratista_id, proveedor_id, visitante_id, tipo_vehiculo, placa, marca, modelo, color, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
        )
        .bind(id)
        .bind(cont_id)
        .bind(prov_id)
        .bind(visit_id)
        .bind(tipo)
        .bind(placa)
        .bind(marca)
        .bind(modelo)
        .bind(color)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    Ok(())
}
