import { useState } from "react";
import type { Crop, SoilType, NewFarmProfile, FarmProfile } from "../types";
import { createFarmProfile } from "../api";

interface Props {
    crops: Crop[];
    soilTypes: SoilType[];
    onCreated: (profile: FarmProfile) => void;
}

// placeholders
const initialFormState: NewFarmProfile = {
    profile_name: "",
    location_lat: 28.6139,
    location_lon: 77.209,
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

    // single generic change handler
    function handleChange(
        field: keyof NewFarmProfile,
        value: string | number
    ) {
        setForm((prev) => ({ ...prev, [field]: value }));
    }

    async function handleSubmit(e: React.FormEvent): Promise<void> {
        e.preventDefault();
        setError(null);

        if (form.crop_id === 0 || form.soil_type_id === 0) {
            setError("Please select a crop and soil type.");
            return;
        }

        setSubmitting(true);
        try {
            const created = await createFarmProfile(form);
            onCreated(created);
            setForm(initialFormState); // reset for the next entry
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
                    placeholder="e.g. North Field - Wheat"
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

            <div className="grid grid-cols-2 gap-4">
                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Latitude
                    </label>
                    <input
                        type="number"
                        step="0.0001"
                        required
                        value={form.location_lat}
                        onChange={(e) =>
                            handleChange("location_lat", Number(e.target.value))
                        }
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    />
                </div>
                <div>
                    <label className="block text-sm font-medium text-muted mb-1">
                        Longitude
                    </label>
                    <input
                        type="number"
                        step="0.0001"
                        required
                        value={form.location_lon}
                        onChange={(e) =>
                            handleChange("location_lon", Number(e.target.value))
                        }
                        className="w-full border border-line rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-accent"
                    />
                </div>
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