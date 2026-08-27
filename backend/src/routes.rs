use axum::{ Json, extract::State, http::StatusCode, response::IntoResponse };
use sqlx::PgPool;

use crate::models::{ Crop, SoilType };
use crate::models::{ FarmProfile, NewFarmProfile };
use crate::infra::weather;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub lat: f64,
    pub lon: f64,
}

// GET /api/crops -> returns every crop in the database as JSON
pub async fn get_crops(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx
        ::query_as::<_, Crop>(
            "SELECT id, name, kc_initial::float8 as kc_initial, kc_mid::float8 as kc_mid, kc_late::float8 as kc_late, total_growth_days FROM crops ORDER BY name"
        )
        .fetch_all(&pool).await;

    match result {
        Ok(crops) => (StatusCode::OK, Json(crops)).into_response(),
        Err(err) => {
            tracing::error!("Failed to fetch crops: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch crops").into_response()
        }
    }
}

// GET /api/soil-types -> returns all soil types
pub async fn get_soil_types(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx
        ::query_as::<_, SoilType>(
            "SELECT id, name, field_capacity::float8 as field_capacity, infiltration_rate::float8 as infiltration_rate FROM soil_types ORDER BY name"
        )
        .fetch_all(&pool).await;

    match result {
        Ok(soil_types) => (StatusCode::OK, Json(soil_types)).into_response(),
        Err(err) => {
            tracing::error!("Failed to fetch soil types: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch soil types").into_response()
        }
    }
}

// GET /api/weather?lat=..&lon=.. -> return latitude/longitude
pub async fn get_weather(Query(params): Query<WeatherQuery>) -> impl IntoResponse {
    match weather::fetch_daily_forecast(params.lat, params.lon).await {
        Ok(daily) => (StatusCode::OK, Json(daily)).into_response(),
        Err(err) => {
            tracing::error!("Failed to fetch weather: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch weather").into_response()
        }
    }
}

// POST /api/farm-profiles -> saves a new farm profile and returns it
pub async fn create_farm_profile(
    State(pool): State<PgPool>,
    Json(payload): Json<NewFarmProfile>
) -> impl IntoResponse {
    let result = sqlx
        ::query_as::<_, FarmProfile>(
            "INSERT INTO farm_profiles
            (profile_name, location_lat, location_lon, crop_id, soil_type_id,
             field_area_hectares, pump_power_kw, pump_flow_rate_lpm, planting_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING
            id, profile_name,
            location_lat::float8 as location_lat,
            location_lon::float8 as location_lon,
            crop_id, soil_type_id,
            field_area_hectares::float8 as field_area_hectares,
            pump_power_kw::float8 as pump_power_kw,
            pump_flow_rate_lpm::float8 as pump_flow_rate_lpm,
            planting_date, created_at"
        )
        .bind(payload.profile_name)
        .bind(payload.location_lat)
        .bind(payload.location_lon)
        .bind(payload.crop_id)
        .bind(payload.soil_type_id)
        .bind(payload.field_area_hectares)
        .bind(payload.pump_power_kw)
        .bind(payload.pump_flow_rate_lpm)
        .bind(payload.planting_date)
        .fetch_one(&pool).await;

    match result {
        Ok(profile) => (StatusCode::CREATED, Json(profile)).into_response(),
        Err(err) => {
            tracing::error!("Failed to create farm profile: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create farm profile").into_response()
        }
    }
}

// GET /api/farm-profiles -> lists all saved profiles
pub async fn list_farm_profiles(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx
        ::query_as::<_, FarmProfile>(
            "SELECT
            id, profile_name,
            location_lat::float8 as location_lat,
            location_lon::float8 as location_lon,
            crop_id, soil_type_id,
            field_area_hectares::float8 as field_area_hectares,
            pump_power_kw::float8 as pump_power_kw,
            pump_flow_rate_lpm::float8 as pump_flow_rate_lpm,
            planting_date, created_at
         FROM farm_profiles ORDER BY created_at DESC"
        )
        .fetch_all(&pool).await;

    match result {
        Ok(profiles) => (StatusCode::OK, Json(profiles)).into_response(),
        Err(err) => {
            tracing::error!("Failed to list farm profiles: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list farm profiles").into_response()
        }
    }
}
