# Architecture

## Tech stack

| Layer | Technology |
|---|---|
| Frontend | React + TypeScript (Vite) | 
| Styling | Tailwind CSS v4 |
| Routing | React Router | 
| Charts | Recharts | 
| Backend | Rust (Axum + Tokio) | 
| Database | PostgreSQL (Supabase-hosted) |
| DB access | sqlx | 
| External API | OpenWeatherMap (Free Tier) |

## Folder structure
```
irrigation-advisor/
├── backend/
│   ├── src/
│   │   ├── main.rs              
│   │   ├── db.rs                
│   │   ├── models.rs            
│   │   ├── routes.rs            
│   │   ├── domain/              
│   │   │   ├── solar.rs
│   │   │   ├── evapotranspiration.rs
│   │   │   ├── crop_coefficient.rs
│   │   │   └── irrigation.rs
│   │   └── infra/
│   │       └── weather.rs
│   └── migrations/              
├── frontend/
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── api.ts
│   │   ├── types.ts
│   │   ├── pages/
│   │   │   ├── HomePage.tsx
│   │   │   └── FarmDetailPage.tsx
│   │   └── components/
│   │       ├── FarmProfileForm.tsx
│   │       └── IrrigationDashboard.tsx
└── docs/                        
```

## Design decisions

**domain/infra separation in the backend:** `domain/` contains pure
functions (same input always produces same output) --
this is our tested "science engine." `infra/` talks to the outside world
(HTTP calls to OpenWeatherMap).

**a shared `types.ts` mirroring Rust structs:** every API response shape
is defined once in Rust and mirrored in TypeScript. This
isn't automatically synced, but keeps the frontend type-safe 
against the shapes we expect the backend to return.

**NUMERIC columns are cast to `::float8` in queries:** Postgres's
`NUMERIC` type is exact/arbitrary-precision and doesn't automatically map to
Rust's `f64` (floating point) via sqlx -- an intentional safety check by
sqlx to avoid silent precision loss. We cast explicitly since our values
(coordinates, coefficients, areas) don't need arbitrary-precision exactness.

## Known limitations / honest trade-offs

- **Frontend/backend type sync is manual.** A schema change in Rust doesn't
  automatically update `types.ts` -- both must be updated together.
- **No authentication.** Farm profiles are not scoped to a user account --
  anyone with API access can see/create any profile.
- **CORS is fully open** (`Any` origin/methods/headers) for local development
  convenience.
- **Weather forecast is capped at ~5-6 days** by OpenWeatherMap's free tier,
  which is why our "irrigation plan" covers that window rather than a full
  crop season.