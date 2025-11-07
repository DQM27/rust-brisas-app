// ==========================================
// src/lib.rs
// ==========================================
pub mod models;
pub mod services;
pub mod commands;
pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(not(mobile))]
    {
        #[tokio::main]
        async fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();
            let pool = db::init_db().await?;
            tauri::Builder::default()
                .manage(pool)
                .invoke_handler(tauri::generate_handler![
                    commands::user_commands::create_user,
                    commands::user_commands::get_user_by_id,  
                    commands::user_commands::get_all_users,
                    commands::user_commands::update_user,
                    commands::user_commands::delete_user,
                    commands::user_commands::login 
                ])
                .run(tauri::generate_context!())?;
            Ok(())
        }
        main_inner().expect("error");
    }
}
