use sqlx::sqlite::SqlitePoolOptions;
use serde_json::json;

#[tokio::test]
async fn test_crear_contratista() {
    let pool = SqlitePoolOptions::new()
        .connect(":memory:")
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .unwrap();

    // Llamas el comando DENTRO del test
    let result = crate::commands::crear_contratista(
        tauri::State::from(&pool),
        json!({
            "nombre": "Pepito",
            "telefono": "123",
            "empresa": "ACME"
        })
        .try_into()
        .unwrap()
    )
    .await;

    assert!(result.is_ok());
}
