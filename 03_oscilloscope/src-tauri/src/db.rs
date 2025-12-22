use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineEvent {
    pub id: String,
    pub time: String,
    pub title: String,
    pub description: String,
    pub color: String,
    pub icon: String,
}

pub struct Database {
    db_path: PathBuf,
}

impl Database {
    pub fn new() -> Self {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("lib.db");
        
        Database { db_path }
    }

    fn get_default_events() -> Vec<TimelineEvent> {
        vec![
            TimelineEvent {
                id: "1".to_string(),
                time: "8:00 am".to_string(),
                title: "早餐".to_string(),
                description: "因为你需要能量".to_string(),
                color: "grey".to_string(),
                icon: "Fastfood".to_string(),
            },
            TimelineEvent {
                id: "2".to_string(),
                time: "09:30 am".to_string(),
                title: "Neetcode150".to_string(),
                description: "上午刷算法题".to_string(),
                color: "primary".to_string(),
                icon: "LaptopMac".to_string(),
            },
            TimelineEvent {
                id: "3".to_string(),
                time: "2:00 pm".to_string(),
                title: "rLLM".to_string(),
                description: "Run any LLM on any device 推理引擎项目开发".to_string(),
                color: "primary".to_string(),
                icon: "LaptopMac".to_string(),
            },
            TimelineEvent {
                id: "4".to_string(),
                time: "11:00 pm".to_string(),
                title: "休息".to_string(),
                description: "因为你需要休息".to_string(),
                color: "primary".to_string(),
                icon: "Hotel".to_string(),
            },
            TimelineEvent {
                id: "5".to_string(),
                time: "8:00 pm".to_string(),
                title: "重复".to_string(),
                description: "因为这是你热爱的生活！".to_string(),
                color: "secondary".to_string(),
                icon: "Repeat".to_string(),
            },
        ]
    }

    pub fn load_events(&self) -> Result<Vec<TimelineEvent>, String> {
        if !self.db_path.exists() {
            let events = Self::get_default_events();
            self.save_events(&events)?;
            return Ok(events);
        }

        let content = fs::read_to_string(&self.db_path)
            .map_err(|e| format!("读取数据库失败: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("解析数据失败: {}", e))
    }

    pub fn save_events(&self, events: &Vec<TimelineEvent>) -> Result<(), String> {
        let json = serde_json::to_string_pretty(events)
            .map_err(|e| format!("序列化失败: {}", e))?;
        
        fs::write(&self.db_path, json)
            .map_err(|e| format!("写入数据库失败: {}", e))?;
        
        Ok(())
    }
}

