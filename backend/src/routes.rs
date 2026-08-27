use axum::{ Json, extract::State, http::StatusCode, response::IntoResponse };
use sqlx::PgPool;

use crate::models::{ Crop, SoilType };
use crate::weather;
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

/// GET /api/weather?lat=..&lon=..
pub async fn get_weather(Query(params): Query<WeatherQuery>) -> impl IntoResponse {
    match weather::fetch_daily_forecast(params.lat, params.lon).await {
        Ok(daily) => (StatusCode::OK, Json(daily)).into_response(),
        Err(err) => {
            tracing::error!("Failed to fetch weather: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch weather").into_response()
        }
    }
}
