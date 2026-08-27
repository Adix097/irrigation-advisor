use axum::{ Json, extract::State, http::StatusCode, response::IntoResponse };
use sqlx::PgPool;

use crate::models::{ Crop, SoilType, IrrigationDay, IrrigationPlanResponse };
use crate::models::{ FarmProfile, NewFarmProfile };
use crate::infra::weather;
use axum::extract::Query;
use serde::Deserialize;

use crate::domain::{ crop_coefficient, evapotranspiration, irrigation };
use axum::extract::Path;
use chrono::{ NaiveDate, Utc };

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

// GET /api/irrigation-plan/:id -> chaining weather -> ET0 -> Kc -> irrigation math
pub async fn get_irrigation_plan(
    State(pool): State<PgPool>,
    Path(farm_profile_id): Path<i32>
) -> impl IntoResponse {
    // Step 1: look up the farm profile
    let profile_result = sqlx
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
         FROM farm_profiles WHERE id = $1"
        )
        .bind(farm_profile_id)
        .fetch_optional(&pool).await;

    let profile = match profile_result {
        Ok(Some(p)) => p,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Farm profile not found").into_response();
        }
        Err(err) => {
            tracing::error!("DB error fetching farm profile: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Step 2: look up the crop's Kc values
    let crop_result = sqlx
        ::query_as::<_, Crop>(
            "SELECT id, name,
            kc_initial::float8 as kc_initial,
            kc_mid::float8 as kc_mid,
            kc_late::float8 as kc_late,
            total_growth_days
         FROM crops WHERE id = $1"
        )
        .bind(profile.crop_id)
        .fetch_optional(&pool).await;

    let crop = match crop_result {
        Ok(Some(c)) => c,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "Crop not found").into_response();
        }
        Err(err) => {
            tracing::error!("DB error fetching crop: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Step 3: fetch the weather forecast for this location
    let forecast = match
        weather::fetch_daily_forecast(profile.location_lat, profile.location_lon).await
    {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Failed to fetch weather: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch weather").into_response();
        }
    };

    // Step 4: run the pipeline for each forecast day
    let mut days = Vec::new();
    let mut total_energy_kwh = 0.0;
    let mut total_baseline_energy_kwh = 0.0;

    for day in &forecast {
        let forecast_date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap_or_else(|_|
            Utc::now().date_naive()
        );
        let days_since_planting = (forecast_date - profile.planting_date).num_days();

        let kc = crop_coefficient::calculate_kc(
            days_since_planting,
            crop.total_growth_days,
            crop.kc_initial,
            crop.kc_mid,
            crop.kc_late
        );

        let et0 = evapotranspiration::calculate_et0(day, profile.location_lat);
        let etc = et0 * kc;

        let net_irrigation = irrigation::net_irrigation_mm(etc, day.total_rainfall_mm);
        let liters = irrigation::mm_to_liters(net_irrigation, profile.field_area_hectares);
        let runtime_minutes = irrigation::liters_to_pump_minutes(
            liters,
            profile.pump_flow_rate_lpm
        );
        let energy_kwh = irrigation::minutes_to_kwh(runtime_minutes, profile.pump_power_kw);

        let baseline_liters = irrigation::mm_to_liters(etc, profile.field_area_hectares);
        let baseline_runtime = irrigation::liters_to_pump_minutes(
            baseline_liters,
            profile.pump_flow_rate_lpm
        );
        let baseline_energy_kwh = irrigation::minutes_to_kwh(
            baseline_runtime,
            profile.pump_power_kw
        );

        total_energy_kwh += energy_kwh;
        total_baseline_energy_kwh += baseline_energy_kwh;

        days.push(IrrigationDay {
            date: day.date.clone(),
            et0_mm: et0,
            kc,
            etc_mm: etc,
            rainfall_mm: day.total_rainfall_mm,
            net_irrigation_mm: net_irrigation,
            pump_runtime_minutes: runtime_minutes,
            energy_kwh,
            baseline_energy_kwh,
        });
    }

    let energy_saved_kwh = total_baseline_energy_kwh - total_energy_kwh;
    let energy_saved_percent = if total_baseline_energy_kwh > 0.0 {
        (energy_saved_kwh / total_baseline_energy_kwh) * 100.0
    } else {
        0.0
    };

    let response = IrrigationPlanResponse {
        farm_profile_id: profile.id,
        crop_name: crop.name,
        days,
        total_energy_kwh,
        total_baseline_energy_kwh,
        energy_saved_kwh,
        energy_saved_percent,
    };

    (StatusCode::OK, Json(response)).into_response()
}
