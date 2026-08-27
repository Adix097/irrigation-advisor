// the fraction of total rainfall that actually becomes available to the crop
const EFFECTIVE_RAINFALL_FACTOR: f64 = 0.8;

// net irrigation requirement in mm/day: crop water need minus the portion of rainfall that's actually usable
pub fn net_irrigation_mm(etc_mm: f64, rainfall_mm: f64) -> f64 {
    let effective_rain = rainfall_mm * EFFECTIVE_RAINFALL_FACTOR;
    (etc_mm - effective_rain).max(0.0)
}

// 1mm of water over 1 hectare = 10,000 liters.
pub fn mm_to_liters(depth_mm: f64, field_area_hectares: f64) -> f64 {
    depth_mm * field_area_hectares * 10_000.0
}

pub fn liters_to_pump_minutes(liters: f64, pump_flow_rate_lpm: f64) -> f64 {
    if pump_flow_rate_lpm <= 0.0 {
        return 0.0; // defensive guard against divide-by-zero on bad input
    }
    liters / pump_flow_rate_lpm
}

pub fn minutes_to_kwh(runtime_minutes: f64, pump_power_kw: f64) -> f64 {
    (runtime_minutes / 60.0) * pump_power_kw
}
