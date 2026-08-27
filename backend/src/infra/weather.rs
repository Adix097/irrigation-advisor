use serde::Deserialize;
use std::collections::BTreeMap;

// response from OpenWeatherMap's 5-day/3-hour forecast endpoint.
#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<ForecastEntry>,
}

// One 3-hour forecast slot. There are ~40 of these per API call (8/day × 5 days).
#[derive(Debug, Deserialize)]
pub struct ForecastEntry {
    pub main: MainData,
    pub wind: WindData, // 'rain' is only present in the JSON when it's actually raining in that window
    pub rain: Option<RainData>,
    pub dt_txt: String,
}

#[derive(Debug, Deserialize)]
pub struct MainData {
    pub temp: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: f64,
}

#[derive(Debug, Deserialize)]
pub struct WindData {
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct RainData {
    #[serde(rename = "3h")]
    pub three_hour: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyWeather {
    pub date: String, // "YYYY-MM-DD"
    pub avg_temp_c: f64,
    pub temp_min_c: f64,
    pub temp_max_c: f64,
    pub avg_humidity_pct: f64,
    pub avg_wind_speed_ms: f64,
    pub total_rainfall_mm: f64,
}

// Fetches the 5-day/3-hour forecast for given coordinates and aggregates it into daily summaries
pub async fn fetch_daily_forecast(lat: f64, lon: f64) -> Result<Vec<DailyWeather>, reqwest::Error> {
    let api_key = std::env
        ::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set in .env");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&units=metric&appid={}",
        lat,
        lon,
        api_key
    );

    let response = reqwest::get(&url).await?.json::<ForecastResponse>().await?;
    Ok(aggregate_by_day(response.list))
}

// Groups 3-hour forecast entries by calendar day and reduces each group to one DailyWeather summary
fn aggregate_by_day(entries: Vec<ForecastEntry>) -> Vec<DailyWeather> {
    // BTreeMap (not HashMap) because it keeps keys SORTED by date automatically
    let mut grouped: BTreeMap<String, Vec<ForecastEntry>> = BTreeMap::new();

    for entry in entries {
        let date = entry.dt_txt.split(' ').next().unwrap_or("").to_string();
        grouped.entry(date).or_insert_with(Vec::new).push(entry);
    }

    grouped
        .into_iter()
        .map(|(date, day_entries)| {
            let count = day_entries.len() as f64;

            let avg_temp_c =
                day_entries
                    .iter()
                    .map(|e| e.main.temp)
                    .sum::<f64>() / count;

            let temp_min_c = day_entries
                .iter()
                .fold(f64::INFINITY, |acc, e| acc.min(e.main.temp_min));

            let temp_max_c = day_entries
                .iter()
                .fold(f64::NEG_INFINITY, |acc, e| acc.max(e.main.temp_max));

            let avg_humidity_pct =
                day_entries
                    .iter()
                    .map(|e| e.main.humidity)
                    .sum::<f64>() / count;

            let avg_wind_speed_ms =
                day_entries
                    .iter()
                    .map(|e| e.wind.speed)
                    .sum::<f64>() / count;

            // Sum rainfall — we use `.map_or(0.0, ...)` to treat "no rain field present" as 0mm
            let total_rainfall_mm = day_entries
                .iter()
                .map(|e| e.rain.as_ref().map_or(0.0, |r| r.three_hour))
                .sum();

            DailyWeather {
                date,
                avg_temp_c,
                temp_min_c,
                temp_max_c,
                avg_humidity_pct,
                avg_wind_speed_ms,
                total_rainfall_mm,
            }
        })
        .collect()
}
