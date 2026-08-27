import { useParams, Link } from "react-router-dom";
import { IrrigationDashboard } from "../components/IrrigationDashboard";

export function FarmDetailPage() {
    const { id } = useParams<{ id: string }>();
    const farmProfileId = Number(id);

    if (!id || Number.isNaN(farmProfileId)) {
        return (
            <div className="max-w-2xl mx-auto px-8 py-10">
                <p className="text-red-700">Invalid farm id</p>
                <Link to="/" className="text-accent text-sm mt-2 inline-block">
                    ← Back home
                </Link>
            </div>
        );
    }

    return (
        <div className="max-w-2xl mx-auto px-8 py-10">
            <Link to="/" className="text-sm text-muted hover:text-ink transition mb-6 inline-block">
                ← Back to farms
            </Link>
            <IrrigationDashboard farmProfileId={farmProfileId} />
        </div>
    );
}