use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RainRecord {
    pub id: u64,
    pub amount_mm: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClothingItem {
    pub id: u32,
    pub name: String,
    pub category: ClothingCategory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClothingCategory {
    Top,
    Bottom,
    Outerwear,
    Shoes,
    Accessory,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeatherSnapshot {
    pub timestamp: DateTime<Utc>,
    pub temperature_c: f32,
    pub humidity_pct: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ComfortLevel {
    Freezing,
    Cold,
    Comfortable,
    Warm,
    Hot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WearEntry {
    pub clothing: Vec<ClothingItem>,
    pub weather: WeatherSnapshot,
    pub comfort: ComfortLevel,
    pub notes: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub records: Vec<RainRecord>,
    pub wear_entries: Vec<WearEntry>,
}

impl Default for AppState {
    fn default() -> Self {
        let sample_entry = WearEntry {
            clothing: vec![
                ClothingItem {
                    id: 1,
                    name: "Camisa roja".to_string(),
                    category: ClothingCategory::Top,
                },
                ClothingItem {
                    id: 2,
                    name: "Pantalón jean".to_string(),
                    category: ClothingCategory::Bottom,
                },
            ],
            weather: WeatherSnapshot {
                timestamp: Utc::now(),
                temperature_c: 22.0,
                humidity_pct: 60,
            },
            comfort: ComfortLevel::Comfortable,
            notes: Some("Se sintió muy bien".to_string()),
        };

        Self {
            records: vec![],
            wear_entries: vec![sample_entry],
        }
    }
}

