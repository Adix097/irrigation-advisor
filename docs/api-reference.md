# API Reference

Base URL (local development): `http://127.0.0.1:3000`

All endpoints return JSON. Errors return a plain-text message with a
4xx/5xx status code.

---

## `GET /health`

Health check. Returns `200 OK` with body `OK` if the server is running.

---

## `GET /api/crops`

Returns every crop with its FAO-56 Kc values.

**Example response:**
```json
[
  {
    "id": 1,
    "name": "Wheat",
    "kc_initial": 0.40,
    "kc_mid": 1.15,
    "kc_late": 0.40,
    "total_growth_days": 150
  }
]
```

---

## `GET /api/soil-types`

Returns every soil type.

**Example response:**
```json
[
  { "id": 2, "name": "Loamy", "field_capacity": 150.00, "infiltration_rate": 10.00 }
]
```

---

## `GET /api/geocode?q={place_name}`

Proxies OpenWeatherMap's geocoding API to resolve a place name into
coordinates. Returns up to 5 candidate matches.

**Example:** `GET /api/geocode?q=Kochi`
```json
[
  { "name": "Kochi", "lat": 9.9679032, "lon": 76.2444378, "country": "IN", "state": "Kerala" }
]
```

---

## `GET /api/weather?lat={lat}&lon={lon}`

Returns a 5-6 day daily-aggregated weather forecast for the given
coordinates. Used internally by the irrigation-plan endpoint; also
callable directly for debugging.

**Example response (one day):**
```json
{
  "date": "2026-08-30",
  "avg_temp_c": 36.19,
  "temp_min_c": 30.0,
  "temp_max_c": 41.5,
  "avg_humidity_pct": 35.9,
  "avg_wind_speed_ms": 4.4,
  "total_rainfall_mm": 0.0
}
```

---

## `POST /api/farm-profiles`

Creates and saves a new farm profile.

**Request body:**
```json
{
  "profile_name": "North Field - Wheat",
  "location_lat": 28.6139,
  "location_lon": 77.2090,
  "crop_id": 1,
  "soil_type_id": 2,
  "field_area_hectares": 2.5,
  "pump_power_kw": 5.0,
  "pump_flow_rate_lpm": 150.0,
  "planting_date": "2026-07-13"
}
```

**Response:** `201 Created`, returns the full saved profile including the
generated `id` and `created_at` timestamp.

---

## `GET /api/farm-profiles`

Lists all saved farm profiles, newest first.

---

## `GET /api/irrigation-plan/{id}`

The core endpoint. Generates a full 5-6 day irrigation and energy plan for
a saved farm profile, chaining weather → ET₀ → Kc → net irrigation → pump
energy. See `docs/science.md` for the full explanation of each calculation
step.

**Response shape:**
```json
{
  "farm_profile_id": 5,
  "crop_name": "Rice",
  "days": [
    {
      "date": "2026-08-27",
      "et0_mm": 8.20,
      "kc": 1.14,
      "etc_mm": 9.32,
      "rainfall_mm": 3.73,
      "net_irrigation_mm": 6.34,
      "pump_runtime_minutes": 1056.8,
      "energy_kwh": 88.07,
      "baseline_energy_kwh": 129.51
    }
  ],
  "total_energy_kwh": 514.51,
  "total_baseline_energy_kwh": 630.06,
  "energy_saved_kwh": 115.56,
  "energy_saved_percent": 18.34
}
```

Returns `404 Not Found` if the farm profile id doesn't exist.