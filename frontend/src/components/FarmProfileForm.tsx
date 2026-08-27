import { useState } from "react";
import type {
    Crop,
    SoilType,
    NewFarmProfile,
    FarmProfile,
    GeocodeResult,
} from "../types";
import { createFarmProfile, geocodeLocation } from "../api";

interface Props {
    crops: Crop[];
    soilTypes: SoilType[];
    onCreated: (profile: FarmProfile) => void;
}

const initialFormState: NewFarmProfile = {
    profile_name: "",
    location_lat: 0,
    location_lon: 0,
    crop_id: 0,
    soil_type_id: 0,
    field_area_hectares: 1,
    pump_power_kw: 5,
    pump_flow_rate_lpm: 150,
    planting_date: new Date().toISOString().split("T")[0],
};

export function FarmProfileForm({ crops, soilTypes, onCreated }: Props) {
    const [form, setForm] = useState<NewFarmProfile>(initialFormState);
    const [submitting, setSubmitting] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const [locationQuery, setLocationQuery] = useState("");
    const [locationResults, setLocationResults] = useState<GeocodeResult[]>([]);
    const [selectedLocation, setSelectedLocation] = useState<GeocodeResult | null>(null);
    const [searching, setSearching] = useState(false);

    function handleChange(field: keyof NewFarmProfile, value: string | number) {
        setForm((prev) => ({ ...prev, [field]: value }));
    }

    async function handleLocationSearch() {
        if (!locationQuery.trim()) return;
        setSearching(true);
        setLocationResults([]);
        try {
            const results = await geocodeLocation(locationQuery);
            setLocationResults(results);
        } catch (err) {
            setError(err instanceof Error ? err.message : "Location search failed");
        } finally {
            setSearching(false);
        }
    }

    function handleSelectLocation(result: GeocodeResult) {
        setSelectedLocation(result);
        setLocationResults([]);
        setForm((prev) => ({
            ...prev,
            location_lat: result.lat,
            location_lon: result.lon,
        }));
    }

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        setError(null);

        if (form.crop_id === 0 || form.soil_type_id === 0) {
            setError("Please select a crop and soil type.");
            return;
        }
        if (!selectedLocation) {
            setError("Please search for and select a location.");
            return;
        }

        setSubmitting(true);
        try {
            const created = await createFarmProfile(form);
            onCreated(created);
            setForm(initialFormState);
            setLocationQuery("");
            setSelectedLocation(null);
        } catch (err) {
            setError(err instanceof Error ? err.message : "Failed to save profile");
        } finally {
            setSubmitting(false);
        }
    }

    return (
        <form onSubmit={handleSubmit} className="space-y-4">
            <div>
                <label className="block text-sm font-medium text-muted mb-1">
                    Farm name
                </label>
                <input
                    type="text"
                    required
                    value={form.profile_name}
                    onChange={(e) => handleChange("profile_name", e.target.value)}
                    className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                />
            </div>

            <div className="grid grid-cols-2 gap-4">
                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Crop
                    </label>
                    <select
                        required
                        value={form.crop_id}
                        onChange={(e) => handleChange("crop_id", Number(e.target.value))}
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    >
                        <option value={0} disabled>
                            Select crop
                        </option>
                        {crops.map((c) => (
                            <option key={c.id} value={c.id}>
                                {c.name}
                            </option>
                        ))}
                    </select>
                </div>

                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Soil type
                    </label>
                    <select
                        required
                        value={form.soil_type_id}
                        onChange={(e) =>
                            handleChange("soil_type_id", Number(e.target.value))
                        }
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    >
                        <option value={0} disabled>
                            Select soil
                        </option>
                        {soilTypes.map((s) => (
                            <option key={s.id} value={s.id}>
                                {s.name}
                            </option>
                        ))}
                    </select>
                </div>
            </div>

            <div>
                <label className="block text-sm font-medium text-muted mb-1">
                    Location
                </label>
                <div className="flex gap-2">
                    <input
                        type="text"
                        value={locationQuery}
                        onChange={(e) => setLocationQuery(e.target.value)}
                        onKeyDown={(e) => {
                            if (e.key === "Enter") {
                                e.preventDefault();
                                handleLocationSearch();
                            }
                        }}
                        className="flex-1 border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    />
                    <button
                        type="button"
                        onClick={handleLocationSearch}
                        disabled={searching}
                        className="border border-line rounded-md px-4 py-2 text-sm font-medium hover:bg-line disabled:opacity-50 transition"
                    >
                        {searching ? "..." : "Find"}
                    </button>
                </div>

                {locationResults.length > 0 && (
                    <div className="mt-2 border border-line rounded-md bg-white divide-y divide-line">
                        {locationResults.map((result, idx) => (
                            <button
                                type="button"
                                key={idx}
                                onClick={() => handleSelectLocation(result)}
                                className="w-full text-left px-3 py-2 hover:bg-cream text-sm"
                            >
                                {result.name}
                                {result.state ? `, ${result.state}` : ""}, {result.country}
                                <span className="text-muted">
                                    {" "}
                                    ({result.lat.toFixed(2)}, {result.lon.toFixed(2)})
                                </span>
                            </button>
                        ))}
                    </div>
                )}

                {selectedLocation && (
                    <p className="mt-2 text-sm text-accent">
                        ✓ {selectedLocation.name}
                        {selectedLocation.state ? `, ${selectedLocation.state}` : ""}
                    </p>
                )}
            </div>

            <div>
                <label className="block text-sm font-medium text-muted mb-1">
                    Planting date
                </label>
                <input
                    type="date"
                    required
                    value={form.planting_date}
                    onChange={(e) => handleChange("planting_date", e.target.value)}
                    className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                />
            </div>

            <div>
                <label className="block text-sm font-medium text-muted mb-1">
                    Field area (hectares)
                </label>
                <input
                    type="number"
                    step="0.1"
                    min="0.1"
                    required
                    value={form.field_area_hectares}
                    onChange={(e) =>
                        handleChange("field_area_hectares", Number(e.target.value))
                    }
                    className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                />
            </div>

            <div className="grid grid-cols-2 gap-4">
                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Pump power (kW)
                    </label>
                    <input
                        type="number"
                        step="0.1"
                        min="0.1"
                        required
                        value={form.pump_power_kw}
                        onChange={(e) =>
                            handleChange("pump_power_kw", Number(e.target.value))
                        }
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    />
                </div>
                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Pump flow rate (L/min)
                    </label>
                    <input
                        type="number"
                        step="1"
                        min="1"
                        required
                        value={form.pump_flow_rate_lpm}
                        onChange={(e) =>
                            handleChange("pump_flow_rate_lpm", Number(e.target.value))
                        }
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    />
                </div>
            </div>

            {error && <p className="text-red-700 text-sm">{error}</p>}

            <button
                type="submit"
                disabled={submitting}
                className="w-full bg-accent text-white rounded-md px-4 py-2.5 font-medium hover:opacity-90 disabled:opacity-50 transition"
            >
                {submitting ? "Saving..." : "Save farm profile"}
            </button>
        </form>
    );
}