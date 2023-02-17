#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use clipboard_win::{formats, get_clipboard};
use sqlite::Connection;
use tauri::{Manager, State};

use crate::database::Database;

mod database;

/// Returns the HTML associated with the dictionary entry
#[tauri::command]
fn get_html(entry: &str, database: State<Database>) -> Option<String> {
    return database.get_html(entry).unwrap_or(Some(String::new()));
}

/// A fail-safe command to read the system clipboard
#[tauri::command]
fn read_clipboard() -> String {
    get_clipboard(formats::Unicode).unwrap_or(String::new())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let resource_path = app.path_resolver()
                .resolve_resource("dictionary/dictionary.db")
                .expect("failed to resolve resource");
            let connection = Connection::open_with_full_mutex(&resource_path)?;
            app.manage(Database::new(connection));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_html, read_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
