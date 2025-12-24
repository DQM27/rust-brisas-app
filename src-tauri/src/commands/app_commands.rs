// src-tauri/src/commands/app_commands.rs

use tauri::command;

#[command]
pub fn exit_app() {
    std::process::exit(0);
}
