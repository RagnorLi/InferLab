mod db;

use db::{Database, TimelineEvent};

#[tauri::command]
fn get_timeline_events() -> Result<Vec<TimelineEvent>, String> {
    let db = Database::new();
    db.load_events()
}

#[tauri::command]
fn save_timeline_events(events: Vec<TimelineEvent>) -> Result<(), String> {
    let db = Database::new();
    db.save_events(&events)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_timeline_events,
            save_timeline_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
