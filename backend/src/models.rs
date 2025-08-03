use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RainRecord {
    pub id: u64,
    pub amount_mm: f64,
}

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub records: Vec<RainRecord>,
}

