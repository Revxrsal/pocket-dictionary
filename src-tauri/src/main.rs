#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::Mutex;
use clipboard_win::{formats, get_clipboard};
use sqlite::Connection;
use tauri::{Manager, State};

use crate::database::Database;

mod database;
mod sanitization;

/// Returns the HTML associated with the dictionary entry
#[tauri::command]
fn lookup_entry(entry: String, database_mutex: State<Mutex<Database>>) -> Option<String> {
    let mut database = database_mutex.lock().unwrap();
    return sanitization::lookup(entry, &mut database);
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

            app.manage(Mutex::new(Database::new(connection)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![lookup_entry, read_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
