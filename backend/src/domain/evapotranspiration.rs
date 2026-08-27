use crate::domain::solar;
use crate::infra::weather::DailyWeather;
use chrono::NaiveDate;

pub fn calculate_et0(day: &DailyWeather, latitude_deg: f64) -> f64 {
    // Parse "2026-08-30" into a real date
    let date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").expect(
        "Weather API returned an invalid date"
    );

    use chrono::Datelike;
    let doy = solar::day_of_year(date.year(), date.month(), date.day());
    let ra = solar::extraterrestrial_radiation(latitude_deg, doy);

    // Hargreaves-Samani equation: ET0 = 0.0023 * Ra * (T_mean + 17.8) * sqrt(T_max - T_min)
    let temp_range = day.temp_max_c - day.temp_min_c;
    let sqrt_temp_range = temp_range.max(0.0).sqrt();

    let et0 = 0.0023 * ra * (day.avg_temp_c + 17.8) * sqrt_temp_range;
    et0.max(0.0)
}
