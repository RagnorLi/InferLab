//! InferLab - 爬山学习进度追踪应用
//! 
//! Tauri backend with PostgreSQL integration

mod db;

use chrono::NaiveDate;
use serde::Serialize;

// ============================================
// Response Types
// ============================================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppData {
    pub habits: Vec<db::Habit>,
    pub milestones: Vec<db::Milestone>,
    pub history: Vec<db::DailyRecordWithHabit>,
    pub current_altitude: i64,
}

// ============================================
// Tauri Commands
// ============================================

/// Initialize database connection
#[tauri::command]
async fn init_database() -> ApiResponse<String> {
    match db::init_pool().await {
        Ok(_) => ApiResponse::ok("Database connected".to_string()),
        Err(e) => ApiResponse::err(&format!("Database connection failed: {}", e)),
    }
}

/// Get welcome message
#[tauri::command]
fn get_welcome_message() -> String {
    "会当凌绝顶，一览众山小！".to_string()
}

/// Load all app data (habits, milestones, history)
#[tauri::command]
async fn load_app_data() -> ApiResponse<AppData> {
    let habits = match db::get_habits().await {
        Ok(h) => h,
        Err(e) => return ApiResponse::err(&format!("Failed to load habits: {}", e)),
    };

    let milestones = match db::get_milestones().await {
        Ok(m) => m,
        Err(e) => return ApiResponse::err(&format!("Failed to load milestones: {}", e)),
    };

    let history = match db::get_all_history().await {
        Ok(h) => h,
        Err(e) => return ApiResponse::err(&format!("Failed to load history: {}", e)),
    };

    let current_altitude = match db::get_current_altitude().await {
        Ok(a) => a,
        Err(e) => return ApiResponse::err(&format!("Failed to get altitude: {}", e)),
    };

    ApiResponse::ok(AppData {
        habits,
        milestones,
        history,
        current_altitude,
    })
}

/// Get records for a specific date
#[tauri::command]
async fn get_records_for_date(date: String) -> ApiResponse<Vec<db::DailyRecordWithHabit>> {
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => return ApiResponse::err(&format!("Invalid date format: {}", e)),
    };

    match db::get_records_for_date(parsed_date).await {
        Ok(records) => ApiResponse::ok(records),
        Err(e) => ApiResponse::err(&format!("Failed to get records: {}", e)),
    }
}

/// Toggle habit completion
#[tauri::command]
async fn toggle_habit(habit_key: String, date: String) -> ApiResponse<bool> {
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => return ApiResponse::err(&format!("Invalid date format: {}", e)),
    };

    match db::toggle_habit(&habit_key, parsed_date).await {
        Ok(completed) => ApiResponse::ok(completed),
        Err(e) => ApiResponse::err(&format!("Failed to toggle habit: {}", e)),
    }
}

/// Save habit log
#[tauri::command]
async fn save_habit_log(habit_key: String, date: String, log_text: String) -> ApiResponse<()> {
    let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => return ApiResponse::err(&format!("Invalid date format: {}", e)),
    };

    match db::save_habit_log(&habit_key, parsed_date, &log_text).await {
        Ok(_) => ApiResponse::ok(()),
        Err(e) => ApiResponse::err(&format!("Failed to save log: {}", e)),
    }
}

/// Get current altitude
#[tauri::command]
async fn get_current_altitude() -> ApiResponse<i64> {
    match db::get_current_altitude().await {
        Ok(altitude) => ApiResponse::ok(altitude),
        Err(e) => ApiResponse::err(&format!("Failed to get altitude: {}", e)),
    }
}

// ============================================
// App Entry Point
// ============================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_welcome_message,
            init_database,
            load_app_data,
            get_records_for_date,
            toggle_habit,
            save_habit_log,
            get_current_altitude,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
