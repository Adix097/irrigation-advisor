import type {
    Crop,
    SoilType,
    FarmProfile,
    NewFarmProfile,
    IrrigationPlanResponse,
    GeocodeResult,
} from "./types";

const BASE_URL = import.meta.env.VITE_API_BASE_URL;

async function apiFetch<T>(path: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${BASE_URL}${path}`, {
        headers: { "Content-Type": "application/json" },
        ...options,
    });

    if (!response.ok) {
        throw new Error(`API error ${response.status}: ${await response.text()}`);
    }

    return response.json() as Promise<T>;
}

export function getCrops(): Promise<Crop[]> {
    return apiFetch<Crop[]>("/api/crops");
}

export function getSoilTypes(): Promise<SoilType[]> {
    return apiFetch<SoilType[]>("/api/soil-types");
}

export function getFarmProfiles(): Promise<FarmProfile[]> {
    return apiFetch<FarmProfile[]>("/api/farm-profiles");
}

export function createFarmProfile(profile: NewFarmProfile): Promise<FarmProfile> {
    return apiFetch<FarmProfile>("/api/farm-profiles", {
        method: "POST",
        body: JSON.stringify(profile),
    });
}

export function getIrrigationPlan(farmProfileId: number): Promise<IrrigationPlanResponse> {
    return apiFetch<IrrigationPlanResponse>(
        `/api/irrigation-plan/${farmProfileId}`
    );
}

export function geocodeLocation(query: string): Promise<GeocodeResult[]> {
    return apiFetch<GeocodeResult[]>(`/api/geocode?q=${encodeURIComponent(query)}`);
}