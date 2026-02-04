// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn rust_to_react(message: &str) {
    println!("Hello from Rust to React! Message: {}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![rust_to_react])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
