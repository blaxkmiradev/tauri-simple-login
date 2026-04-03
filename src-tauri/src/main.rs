#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::State;
use std::collections::HashMap;
use std::sync::Mutex;

// Shared state for users
struct Users(Mutex<HashMap<String, String>>);

#[tauri::command]
fn login(username: String, password: String, users: State<Users>) -> bool {
    let users = users.0.lock().unwrap();
    match users.get(&username) {
        Some(stored_pass) if stored_pass == &password => true,
        _ => false,
    }
}

fn main() {
    let mut users_map = HashMap::new();
    users_map.insert("admin".into(), "password123".into()); // sample user

    tauri::Builder::default()
        .manage(Users(Mutex::new(users_map)))
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
