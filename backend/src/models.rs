use serde::Serialize;
use sqlx::FromRow;

// Mirrors the `crops` table, FromRow lets sqlx map a Postgres row directly into this struct
#[derive(Debug, Serialize, FromRow)]
pub struct Crop {
    pub id: i32,
    pub name: String,
    pub kc_initial: f64,
    pub kc_mid: f64,
    pub kc_late: f64,
    pub total_growth_days: i32,
}

// Mirrors the `soil_types` table.
#[derive(Debug, Serialize, FromRow)]
pub struct SoilType {
    pub id: i32,
    pub name: String,
    pub field_capacity: f64,
    pub infiltration_rate: f64,
}
