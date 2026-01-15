use brisas_app_lib::models::contratista::{Contratista, EstadoContratista};
use std::str::FromStr;
use surrealdb::{
    engine::local::{Db, SurrealKv},
    RecordId, Surreal,
};

pub async fn setup_test_db() -> Surreal<Db> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir().join("brisas_test").join(uuid);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let db = Surreal::new::<SurrealKv>(temp_dir).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let schema = include_str!("../../src/db/surrealdb_schema.surql");
    db.query(schema).await.expect("Failed to apply schema");

    db
}

pub fn crear_contratista_activo(cedula: &str) -> Contratista {
    let id_str = format!("contratista:{}", uuid::Uuid::new_v4());
    Contratista {
        id: RecordId::from_str(&id_str).expect("Valid ID"),
        cedula: cedula.to_string(),
        nombre: format!("Test {}", cedula),
        segundo_nombre: None,
        apellido: "User".to_string(),
        segundo_apellido: None,
        empresa: RecordId::from_str("empresa:test").expect("Valid ID"),
        fecha_vencimiento_praind: surrealdb::Datetime::default(),
        estado: EstadoContratista::Activo,
        created_at: surrealdb::Datetime::default(),
        updated_at: surrealdb::Datetime::default(),
        deleted_at: None,
    }
}

pub fn crear_contratista_inactivo(cedula: &str) -> Contratista {
    let mut c = crear_contratista_activo(cedula);
    c.estado = EstadoContratista::Inactivo;
    c
}
