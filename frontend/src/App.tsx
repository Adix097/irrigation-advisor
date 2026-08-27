import { useEffect, useState } from "react";
import type { Crop, SoilType } from "./types";
import { getCrops, getSoilTypes } from "./api";

function App() {
  const [crops, setCrops] = useState<Crop[]>([]);
  const [soilTypes, setSoilTypes] = useState<SoilType[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    Promise.all([getCrops(), getSoilTypes()])
      .then(([cropsData, soilData]) => {
        setCrops(cropsData);
        setSoilTypes(soilData);
      })
      .catch((err) => setError(err.message));
  }, []);

  if (error) {
    return (
      <div className="min-h-screen bg-cream flex items-center justify-center">
        <p className="text-red-700">Failed to load: {error}</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-cream text-ink">
      <header className="border-b border-line px-8 py-5">
        <h1 className="text-lg font-semibold tracking-tight">
          Irrigation Advisor
        </h1>
      </header>

      <main className="max-w-2xl mx-auto px-8 py-10">
        <section className="mb-10">
          <h2 className="text-sm font-medium text-muted uppercase tracking-wide mb-3">
            Crops
          </h2>
          <div className="border border-line rounded-md divide-y divide-line bg-white">
            {crops.map((crop) => (
              <div
                key={crop.id}
                className="px-4 py-3 flex justify-between items-center"
              >
                <span>{crop.name}</span>
                <span className="text-sm text-muted">
                  Kc {crop.kc_initial} / {crop.kc_mid} / {crop.kc_late}
                </span>
              </div>
            ))}
          </div>
        </section>

        <section>
          <h2 className="text-sm font-medium text-muted uppercase tracking-wide mb-3">
            Soil Types
          </h2>
          <div className="border border-line rounded-md divide-y divide-line bg-white">
            {soilTypes.map((soil) => (
              <div key={soil.id} className="px-4 py-3">
                {soil.name}
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;