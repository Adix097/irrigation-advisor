// Mirrors backend/src/models.rs -> Crop
export interface Crop {
    id: number;
    name: string;
    kc_initial: number;
    kc_mid: number;
    kc_late: number;
    total_growth_days: number;
}

// Mirrors backend/src/models.rs -> SoilType
export interface SoilType {
    id: number;
    name: string;
    field_capacity: number;
    infiltration_rate: number;
}

// Mirrors backend/src/models.rs -> FarmProfile
export interface FarmProfile {
    id: number;
    profile_name: string;
    location_lat: number;
    location_lon: number;
    crop_id: number;
    soil_type_id: number;
    field_area_hectares: number;
    pump_power_kw: number;
    pump_flow_rate_lpm: number;
    planting_date: string;
    created_at: string;
}

// Mirrors backend/src/models.rs -> NewFarmProfile
export type NewFarmProfile = Omit<FarmProfile, "id" | "created_at">;

// Mirrors backend/src/models.rs -> IrrigationDay
export interface IrrigationDay {
    date: string;
    et0_mm: number;
    kc: number;
    etc_mm: number;
    rainfall_mm: number;
    net_irrigation_mm: number;
    pump_runtime_minutes: number;
    energy_kwh: number;
    baseline_energy_kwh: number;
}

// Mirrors backend/src/models.rs -> IrrigationPlanResponse
export interface IrrigationPlanResponse {
    farm_profile_id: number;
    crop_name: string;
    days: IrrigationDay[];
    total_energy_kwh: number;
    total_baseline_energy_kwh: number;
    energy_saved_kwh: number;
    energy_saved_percent: number;
}

// Mirrors backend/src/infra/weather.rs -> GeocodeResult
export interface GeocodeResult {
    name: string;
    lat: number;
    lon: number;
    country: string;
    state: string | null;
}