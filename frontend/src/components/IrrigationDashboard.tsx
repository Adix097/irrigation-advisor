import { useEffect, useState } from "react";
import {
    BarChart,
    Bar,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from "recharts";
import type { IrrigationPlanResponse } from "../types";
import { getIrrigationPlan } from "../api";

interface Props {
    farmProfileId: number;
}

export function IrrigationDashboard({ farmProfileId }: Props) {
    const [plan, setPlan] = useState<IrrigationPlanResponse | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        setLoading(true);
        setError(null);
        getIrrigationPlan(farmProfileId)
            .then(setPlan)
            .catch((err) =>
                setError(err instanceof Error ? err.message : "Failed to load plan")
            )
            .finally(() => setLoading(false));
    }, [farmProfileId]);

    if (loading) {
        return <p className="text-muted text-sm">Loading irrigation plan...</p>;
    }

    if (error) {
        return <p className="text-red-700 text-sm">{error}</p>;
    }

    if (!plan) return null;

    // Recharts needs a flat array of plain objects
    // reshape our IrrigationDay[] into the {name, smart, baseline} 
    const chartData = plan.days.map((d) => ({
        date: d.date.slice(5), // "2026-08-30" -> "08-30"
        Smart: Math.round(d.energy_kwh * 10) / 10,
        Baseline: Math.round(d.baseline_energy_kwh * 10) / 10,
    }));

    return (
        <div className="space-y-6">
            <div>
                <h3 className="text-lg font-semibold">{plan.crop_name} — 7-Day Plan</h3>
                <p className="text-sm text-muted">Farm profile #{plan.farm_profile_id}</p>
            </div>

            <div className="grid grid-cols-3 gap-4">
                <div className="border border-line rounded-md bg-white p-4">
                    <p className="text-xs text-muted uppercase tracking-wide mb-1">
                        Energy Saved
                    </p>
                    <p className="text-2xl font-semibold text-accent">
                        {plan.energy_saved_percent.toFixed(1)}%
                    </p>
                </div>
                <div className="border border-line rounded-md bg-white p-4">
                    <p className="text-xs text-muted uppercase tracking-wide mb-1">
                        kWh Saved
                    </p>
                    <p className="text-2xl font-semibold text-accent">
                        {plan.energy_saved_kwh.toFixed(1)}
                    </p>
                </div>
                <div className="border border-line rounded-md bg-white p-4">
                    <p className="text-xs text-muted uppercase tracking-wide mb-1">
                        Total Energy Used
                    </p>
                    <p className="text-2xl font-semibold">
                        {plan.total_energy_kwh.toFixed(1)} kWh
                    </p>
                </div>
            </div>

            {/* Chart: smart vs baseline energy per day */}
            <div className="border border-line rounded-md bg-white p-4">
                <p className="text-xs text-muted uppercase tracking-wide mb-3">
                    Daily Energy — Smart vs Baseline
                </p>
                <ResponsiveContainer width="100%" height={240}>
                    <BarChart data={chartData}>
                        <CartesianGrid strokeDasharray="3 3" stroke="#E4E2DA" />
                        <XAxis dataKey="date" tick={{ fontSize: 12, fill: "#8A8677" }} />
                        <YAxis tick={{ fontSize: 12, fill: "#8A8677" }} />
                        <Tooltip />
                        <Legend />
                        <Bar dataKey="Baseline" fill="#C9C6BA" radius={[3, 3, 0, 0]} />
                        <Bar dataKey="Smart" fill="#3D6B52" radius={[3, 3, 0, 0]} />
                    </BarChart>
                </ResponsiveContainer>
            </div>

            {/* Day-by-day detail table */}
            <div className="border border-line rounded-md bg-white overflow-hidden">
                <table className="w-full text-sm">
                    <thead>
                        <tr className="border-b border-line text-left text-muted">
                            <th className="px-3 py-2 font-medium">Date</th>
                            <th className="px-3 py-2 font-medium">ET₀ (mm)</th>
                            <th className="px-3 py-2 font-medium">Kc</th>
                            <th className="px-3 py-2 font-medium">Rain (mm)</th>
                            <th className="px-3 py-2 font-medium">Net Irrigation (mm)</th>
                            <th className="px-3 py-2 font-medium">Energy (kWh)</th>
                        </tr>
                    </thead>
                    <tbody>
                        {plan.days.map((day) => (
                            <tr key={day.date} className="border-b border-line last:border-0">
                                <td className="px-3 py-2">{day.date}</td>
                                <td className="px-3 py-2">{day.et0_mm.toFixed(2)}</td>
                                <td className="px-3 py-2">{day.kc.toFixed(2)}</td>
                                <td className="px-3 py-2">{day.rainfall_mm.toFixed(1)}</td>
                                <td className="px-3 py-2">{day.net_irrigation_mm.toFixed(2)}</td>
                                <td className="px-3 py-2">{day.energy_kwh.toFixed(2)}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}