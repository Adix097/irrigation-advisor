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

// Mirrors the `soil_types` table
#[derive(Debug, Serialize, FromRow)]
pub struct SoilType {
    pub id: i32,
    pub name: String,
    pub field_capacity: f64,
    pub infiltration_rate: f64,
}

use chrono::NaiveDate;

/// Mirrors the `farm_profiles` table
#[derive(Debug, Serialize, FromRow)]
pub struct FarmProfile {
    pub id: i32,
    pub profile_name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub crop_id: i32,
    pub soil_type_id: i32,
    pub field_area_hectares: f64,
    pub pump_power_kw: f64,
    pub pump_flow_rate_lpm: f64,
    pub planting_date: NaiveDate,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewFarmProfile {
    pub profile_name: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub crop_id: i32,
    pub soil_type_id: i32,
    pub field_area_hectares: f64,
    pub pump_power_kw: f64,
    pub pump_flow_rate_lpm: f64,
    pub planting_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct IrrigationDay {
    pub date: String,
    pub et0_mm: f64,
    pub kc: f64,
    pub etc_mm: f64, // crop water requirement before rain adjustment
    pub rainfall_mm: f64,
    pub net_irrigation_mm: f64, // after subtracting effective rainfall
    pub pump_runtime_minutes: f64,
    pub energy_kwh: f64,
    pub baseline_energy_kwh: f64, // what a fixed/naive schedule would have used
}

// full response for GET /api/irrigation-plan/:id
#[derive(Debug, Serialize)]
pub struct IrrigationPlanResponse {
    pub farm_profile_id: i32,
    pub crop_name: String,
    pub days: Vec<IrrigationDay>,
    pub total_energy_kwh: f64,
    pub total_baseline_energy_kwh: f64,
    pub energy_saved_kwh: f64,
    pub energy_saved_percent: f64,
}
