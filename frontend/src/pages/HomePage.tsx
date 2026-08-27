import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import type { Crop, SoilType, FarmProfile } from "../types";
import { getCrops, getSoilTypes, getFarmProfiles } from "../api";
import { FarmProfileForm } from "../components/FarmProfileForm";

export function HomePage() {
    const [crops, setCrops] = useState<Crop[]>([]);
    const [soilTypes, setSoilTypes] = useState<SoilType[]>([]);
    const [profiles, setProfiles] = useState<FarmProfile[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [showForm, setShowForm] = useState(false);

    useEffect(() => {
        Promise.all([getCrops(), getSoilTypes(), getFarmProfiles()])
            .then(([cropsData, soilData, profileData]) => {
                setCrops(cropsData);
                setSoilTypes(soilData);
                setProfiles(profileData);
            })
            .catch((err) => setError(err.message));
    }, []);

    function handleProfileCreated(profile: FarmProfile) {
        setProfiles((prev) => [profile, ...prev]);
        setShowForm(false); // collapse the form back down after a successful save
    }

    if (error) {
        return <p className="text-red-700 p-8">Failed to load: {error}</p>;
    }

    return (
        <div className="max-w-2xl mx-auto px-8 py-10 space-y-10">
            <section>
                {!showForm ? (
                    <button
                        onClick={() => setShowForm(true)}
                        className="w-full border border-line rounded-md bg-white p-4 text-left font-medium text-accent hover:bg-cream transition"
                    >
                        + Create Farm
                    </button>
                ) : (
                    <div className="border border-line rounded-md bg-white p-5">
                        <div className="flex justify-between items-center mb-4">
                            <h2 className="text-sm font-medium text-muted uppercase tracking-wide">
                                Add a farm
                            </h2>
                            <button
                                onClick={() => setShowForm(false)}
                                className="text-sm text-muted hover:text-ink transition"
                            >
                                Cancel
                            </button>
                        </div>
                        <FarmProfileForm
                            crops={crops}
                            soilTypes={soilTypes}
                            onCreated={handleProfileCreated}
                        />
                    </div>
                )}
            </section>

            <section>
                <h2 className="text-sm font-medium text-muted uppercase tracking-wide mb-3">
                    Your farms ({profiles.length})
                </h2>
                {profiles.length === 0 ? (
                    <p className="text-muted text-sm">
                        No farms saved yet
                    </p>
                ) : (
                    <div className="border border-line rounded-md divide-y divide-line bg-white">
                        {profiles.map((p) => (
                            <Link
                                key={p.id}
                                to={`/farms/${p.id}`}
                                className="block px-4 py-3 hover:bg-cream transition"
                            >
                                <p className="font-medium">{p.profile_name}</p>
                                <p className="text-sm text-muted">
                                    Planted {p.planting_date}, {p.field_area_hectares} ha
                                </p>
                            </Link>
                        ))}
                    </div>
                )}
            </section>
        </div>
    );
}