import { Outlet, Link } from "react-router-dom";

function App() {
  return (
    <div className="min-h-screen bg-cream text-ink">
      <header className="border-b border-line px-8 py-5">
        <Link to="/" className="text-lg font-semibold tracking-tight">
          Irrigation Advisor
        </Link>
      </header>
      <Outlet />
    </div>
  );
}

export default App;