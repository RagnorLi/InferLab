// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_welcome_message() -> String {
    "会当凌绝顶，一览众山小 ！".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_welcome_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
