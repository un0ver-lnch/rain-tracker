use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Rain measurement stored in the database
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct RainRecord {
    pub id: i64,
    pub amount_mm: f64,
}

/// Payload used to create or update a rain record
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RainRecordInput {
    pub amount_mm: f64,
}

impl RainRecordInput {
    pub fn with_id(self, id: i64) -> RainRecord {
        RainRecord { id, amount_mm: self.amount_mm }
    }
}

/// A piece of clothing used in a wear entry
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

/// Snapshot of weather when the clothing entry was recorded
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

/// Complete wear entry returned by the API
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WearEntry {
    pub id: i64,
    pub clothing: Vec<ClothingItem>,
    pub weather: WeatherSnapshot,
    pub comfort: ComfortLevel,
    pub notes: Option<String>,
}

/// Payload used to create or update a wear entry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WearEntryInput {
    pub clothing: Vec<ClothingItem>,
    pub weather: WeatherSnapshot,
    pub comfort: ComfortLevel,
    pub notes: Option<String>,
}

impl WearEntryInput {
    pub fn with_id(self, id: i64) -> WearEntry {
        WearEntry {
            id,
            clothing: self.clothing,
            weather: self.weather,
            comfort: self.comfort,
            notes: self.notes,
        }
    }
}

/// Shared application state containing the database connection pool
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::AnyPool,
}

