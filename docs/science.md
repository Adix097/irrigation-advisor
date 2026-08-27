# The Science Behind Irrigation Advisor

This document explains the agronomic and hydrological models used to calculate irrigation recommendations.

## Overview of the calculation pipeline
Weather forecast (OpenWeatherMap)
→ Reference evapotranspiration (ET₀) via Hargreaves-Samani
→ Crop coefficient (Kc) based on growth stage
→ Crop water requirement (ETc = ET₀ × Kc)
→ Net irrigation need (ETc minus effective rainfall)
→ Water volume (liters) for the field
→ Pump runtime (minutes)
→ Energy consumed (kWh)


Every stage above is a small, independently-tested function in
`backend/src/domain/`, chained together in the `/api/irrigation-plan/:id`
route handler.

## Reference evapotranspiration (ET₀)

ET₀ answers: "how many millimeters of water would evaporate from a reference
grass surface today, given the weather?" It's a weather-driven baseline, not
specific to any crop yet.

**We use the Hargreaves-Samani equation**, not full FAO-56 Penman-Monteith:
$$
ET_0 = 0.0023 \times R_a \times (T_{mean} + 17.8)
\times \sqrt{T_{max} - T_{min}}
$$

Full Penman-Monteith needs solar radiation and wind data from ground
weather stations, which our free-tier weather API doesn't provide.

**Known limitation:** Hargreaves-Samani is documented to run 15-20% higher than
full Penman-Monteith in hot, arid, windy conditions.

`Ra` (extraterrestrial radiation) is calculated purely from latitude and
day-of-year using standard astronomical formulas (FAO-56 Eq. 21) — no external
data needed, implemented in `backend/src/domain/solar.rs`.

## Crop coefficient (Kc) and growth staging

ET₀ is generic; actual crop water use depends on the crop's growth stage. We
use the FAO-56 four-stage model:

| Stage | % of total growth cycle | Kc behavior |
|---|---|---|
| Initial | 0-20% | flat at `kc_initial` |
| Development | 20-50% | linear ramp: `kc_initial` → `kc_mid` |
| Mid-season | 50-80% | flat at `kc_mid` |
| Late-season | 80-100% | linear ramp: `kc_mid` → `kc_late` |

Growth stage is calculated automatically from the user's planting date,
rather than asking the user to self-select a stage — implemented in
`backend/src/domain/crop_coefficient.rs`.

**Known limitation:** the 20/30/30/20% split is FAO-56's generalized default.
Real stage lengths vary by crop, region, and cultivar; FAO-56's appendix
tables give crop-specific proportions we don't yet incorporate.

Crop water requirement: `ETc = ET0 × Kc` (mm/day).

## Effective rainfall

Not all rainfall reduces irrigation need — some is lost to runoff. We apply a
flat **80% effective rainfall factor**:

$$
\text{net\_irrigation\_mm}
=
\max\left(
0,\;
ET_c - (\text{rainfall\_mm} \times 0.8)
\right)
$$

This is a simplified, commonly-cited default in agricultural water-balance
literature. A more advanced model would vary this factor by soil
infiltration rate (which we do store, in `soil_types.infiltration_rate`, but
don't yet use) — noted here as a clear next step, not a gap we're unaware of.

## From water need to energy

$$
\text{liters}
=
\text{net\_irrigation\_mm}
\times
\text{field\_area\_hectares}
\times
10{,}000
$$

$$
\text{pump\_runtime\_minutes}
=
\frac{\text{liters}}{\text{pump\_flow\_rate\_lpm}}
$$

$$
\text{energy\_kwh}
=
\left(
\frac{\text{pump\_runtime\_minutes}}{60}
\right)
\times
\text{pump\_power\_kw}
$$

The `10,000` constant: 1mm of water depth over 1 hectare = 10,000 liters
(1 hectare = 10,000 m²; 1mm over 1m² = 1 liter).

## The "energy saved" baseline

Our comparison baseline is: the same crop water requirement (`ETc`), applied
**without** any rainfall adjustment — i.e., a farmer who correctly follows
crop water tables but doesn't adjust for whether it actually rained that day.

This isolates exactly what our system adds: **weather-responsiveness**. We
deliberately did not compare against an arbitrary flat irrigation amount
(e.g. "5mm every day"), since that conflates two different things (knowing
correct crop water need vs. responding to weather) and can produce misleading
or even negative "savings" depending on how the arbitrary number compares to
real crop need.