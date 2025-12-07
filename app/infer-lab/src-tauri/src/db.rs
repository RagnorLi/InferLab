//! Database module for InferLab
//! 
//! Handles PostgreSQL connections and provides data access layer.

use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use once_cell::sync::OnceCell;
use std::env;

/// Global database pool
static DB_POOL: OnceCell<PgPool> = OnceCell::new();

/// Initialize database connection pool (safe to call multiple times)
pub async fn init_pool() -> Result<(), sqlx::Error> {
    // Already initialized, skip
    if DB_POOL.get().is_some() {
        return Ok(());
    }
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://inferlab:inferlab_dev_2024@localhost:5432/inferlab".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Use set() but ignore if already set (race condition safe)
    let _ = DB_POOL.set(pool);
    Ok(())
}

/// Get database pool reference
pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database pool not initialized")
}

// ============================================
// Data Models
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Habit {
    pub id: i32,
    pub key: String,
    pub label: String,
    pub gain_meters: i32,
    pub description: Option<String>,
    pub action_label: Option<String>,
    pub action_link: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyRecord {
    pub id: i32,
    pub habit_id: i32,
    pub record_date: NaiveDate,
    pub completed: bool,
    pub log_text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Milestone {
    pub id: i32,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub altitude: i32,
    pub sort_order: i32,
}

/// Record with habit info for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRecordWithHabit {
    pub habit_key: String,
    pub record_date: NaiveDate,
    pub completed: bool,
    pub log_text: Option<String>,
}

/// Current altitude stats (reserved for future use)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AltitudeStats {
    pub total_altitude: i64,
    pub total_completed: i64,
}

// ============================================
// Database Queries
// ============================================

/// Get all active habits
pub async fn get_habits() -> Result<Vec<Habit>, sqlx::Error> {
    sqlx::query_as::<_, Habit>(
        r#"
        SELECT id, key, label, gain_meters, description, action_label, action_link, sort_order, is_active
        FROM habits
        WHERE is_active = TRUE
        ORDER BY sort_order
        "#
    )
    .fetch_all(get_pool())
    .await
}

/// Get all milestones
pub async fn get_milestones() -> Result<Vec<Milestone>, sqlx::Error> {
    sqlx::query_as::<_, Milestone>(
        r#"
        SELECT id, key, title, description, altitude, sort_order
        FROM milestones
        ORDER BY sort_order
        "#
    )
    .fetch_all(get_pool())
    .await
}

/// Get current total altitude
pub async fn get_current_altitude() -> Result<i64, sqlx::Error> {
    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(h.gain_meters), 0)::BIGINT
        FROM daily_records dr
        JOIN habits h ON dr.habit_id = h.id
        WHERE dr.completed = TRUE
        "#
    )
    .fetch_one(get_pool())
    .await?;
    
    Ok(result.0)
}

/// Get records for a specific date
pub async fn get_records_for_date(date: NaiveDate) -> Result<Vec<DailyRecordWithHabit>, sqlx::Error> {
    sqlx::query_as::<_, (String, NaiveDate, bool, Option<String>)>(
        r#"
        SELECT h.key, dr.record_date, dr.completed, dr.log_text
        FROM daily_records dr
        JOIN habits h ON dr.habit_id = h.id
        WHERE dr.record_date = $1
        "#
    )
    .bind(date)
    .fetch_all(get_pool())
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|(habit_key, record_date, completed, log_text)| DailyRecordWithHabit {
                habit_key,
                record_date,
                completed,
                log_text,
            })
            .collect()
    })
}

/// Get all history records
pub async fn get_all_history() -> Result<Vec<DailyRecordWithHabit>, sqlx::Error> {
    sqlx::query_as::<_, (String, NaiveDate, bool, Option<String>)>(
        r#"
        SELECT h.key, dr.record_date, dr.completed, dr.log_text
        FROM daily_records dr
        JOIN habits h ON dr.habit_id = h.id
        ORDER BY dr.record_date DESC, h.sort_order
        "#
    )
    .fetch_all(get_pool())
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|(habit_key, record_date, completed, log_text)| DailyRecordWithHabit {
                habit_key,
                record_date,
                completed,
                log_text,
            })
            .collect()
    })
}

/// Toggle habit completion for a date
pub async fn toggle_habit(habit_key: &str, date: NaiveDate) -> Result<bool, sqlx::Error> {
    // First, get the habit id
    let habit: Habit = sqlx::query_as::<_, Habit>(
        "SELECT id, key, label, gain_meters, description, action_label, action_link, sort_order, is_active FROM habits WHERE key = $1"
    )
    .bind(habit_key)
    .fetch_one(get_pool())
    .await?;
    
    // Check if record exists
    let existing: Option<DailyRecord> = sqlx::query_as::<_, DailyRecord>(
        r#"
        SELECT id, habit_id, record_date, completed, log_text, created_at, updated_at
        FROM daily_records
        WHERE habit_id = $1 AND record_date = $2
        "#
    )
    .bind(habit.id)
    .bind(date)
    .fetch_optional(get_pool())
    .await?;
    
    let new_completed: bool;
    
    if let Some(record) = existing {
        // Toggle existing record
        new_completed = !record.completed;
        sqlx::query(
            "UPDATE daily_records SET completed = $1 WHERE id = $2"
        )
        .bind(new_completed)
        .bind(record.id)
        .execute(get_pool())
        .await?;
    } else {
        // Create new record with completed = true
        new_completed = true;
        sqlx::query(
            r#"
            INSERT INTO daily_records (habit_id, record_date, completed)
            VALUES ($1, $2, TRUE)
            "#
        )
        .bind(habit.id)
        .bind(date)
        .execute(get_pool())
        .await?;
    }
    
    Ok(new_completed)
}

/// Save habit log
pub async fn save_habit_log(habit_key: &str, date: NaiveDate, log_text: &str) -> Result<(), sqlx::Error> {
    // Get habit id
    let habit: Habit = sqlx::query_as::<_, Habit>(
        "SELECT id, key, label, gain_meters, description, action_label, action_link, sort_order, is_active FROM habits WHERE key = $1"
    )
    .bind(habit_key)
    .fetch_one(get_pool())
    .await?;
    
    // Upsert record
    sqlx::query(
        r#"
        INSERT INTO daily_records (habit_id, record_date, completed, log_text)
        VALUES ($1, $2, TRUE, $3)
        ON CONFLICT (habit_id, record_date)
        DO UPDATE SET log_text = $3, completed = TRUE
        "#
    )
    .bind(habit.id)
    .bind(date)
    .bind(log_text)
    .execute(get_pool())
    .await?;
    
    Ok(())
}

