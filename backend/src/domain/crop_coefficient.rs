// Calculates the crop coefficient (Kc) for a given day based on
// how many days have passed since planting relative to the crop's total growth cycle
pub fn calculate_kc(
    days_since_planting: i64,
    total_growth_days: i32,
    kc_initial: f64,
    kc_mid: f64,
    kc_late: f64
) -> f64 {
    let total = total_growth_days as f64;
    let day = days_since_planting as f64;

    if day <= 0.0 {
        return kc_initial;
    }
    if day >= total {
        return kc_late;
    }

    let initial_end = total * 0.2;
    let dev_end = total * 0.5;
    let mid_end = total * 0.8;
    // remaining 20% is late-season, up to `total`

    if day <= initial_end {
        // initial stage.
        kc_initial
    } else if day <= dev_end {
        // linear interpolation from kc_initial to kc_mid across development stage
        let progress = (day - initial_end) / (dev_end - initial_end);
        kc_initial + progress * (kc_mid - kc_initial)
    } else if day <= mid_end {
        // mid-season stage - peak water demand
        kc_mid
    } else {
        // linear interpolation from kc_mid to kc_late across late-season decline
        let progress = (day - mid_end) / (total - mid_end);
        kc_mid + progress * (kc_late - kc_mid)
    }
}
