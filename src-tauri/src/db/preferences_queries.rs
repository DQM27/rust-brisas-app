use sqlx::{Result, SqlitePool};

#[derive(serde::Serialize, serde::Deserialize, Debug, sqlx::FromRow)]
pub struct UserPreferenceRow {
    pub category: String,
    pub key: String,
    pub value: String,
}

pub async fn get_user_preferences(
    pool: &SqlitePool,
    user_id: &str,
    category: &str,
) -> Result<Vec<UserPreferenceRow>> {
    let prefs = sqlx::query_as::<_, UserPreferenceRow>(
        r#"
        SELECT category, key, value
        FROM user_preferences
        WHERE user_id = $1 AND category = $2
        "#,
    )
    .bind(user_id)
    .bind(category)
    .fetch_all(pool)
    .await?;

    Ok(prefs)
}

pub async fn set_user_preference(
    pool: &SqlitePool,
    user_id: &str,
    category: &str,
    key: &str,
    value: &str,
) -> Result<()> {
    // UPSERT (Insert or Update)
    sqlx::query(
        r#"
        INSERT INTO user_preferences (user_id, category, key, value)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT(user_id, category, key) DO UPDATE SET
            value = excluded.value,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(user_id)
    .bind(category)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}
