use std::f64::consts::PI;

// calculates extraterrestrial radiation (Ra) in MJ/m²/day for a given
pub fn extraterrestrial_radiation(latitude_deg: f64, day_of_year: u32) -> f64 {
    let phi = latitude_deg.to_radians(); // latitude in radians
    let j = day_of_year as f64;

    const GSC: f64 = 0.082; // Solar constant: MJ/m²/min
    let dr = 1.0 + 0.033 * (((2.0 * PI) / 365.0) * j).cos(); // relative distance Earth-Sun
    let delta = 0.409 * (((2.0 * PI) / 365.0) * j - 1.39).sin(); // solar declination
    let sunset_angle_arg = -phi.tan() * delta.tan(); // sunset hour angle
    let omega_s = sunset_angle_arg.clamp(-1.0, 1.0).acos(); // idk wtf is this but i need this in range of [-1, 1]

    let ra =
        ((24.0 * 60.0) / PI) *
        GSC *
        dr *
        (omega_s * phi.sin() * delta.sin() + phi.cos() * delta.cos() * omega_s.sin());

    ra
}

pub fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    use chrono::{ Datelike, NaiveDate };
    let date = NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date");
    date.ordinal()
}
