# Local Setup

## Prerequisites

- Rust (stable toolchain) + Cargo
- Node.js + npm
- `sqlx-cli`: `cargo install sqlx-cli --no-default-features --features rustls,postgres`
- A Supabase account (or any PostgreSQL instance)
- An OpenWeatherMap account

## 1. Clone and configure the backend
`cd backend`
Create `backend/.env` (see `backend/.env.example` for the template)

Run database migrations: `sqlx migrate run`

This creates the `crops`, `soil_types`, and `farm_profiles` tables and
seeds reference crop/soil data (see `backend/migrations/`).

Start the backend: `cargo run`

## 2. Configure and run the frontend

In a separate terminal:
```
cd frontend
npm install
```

Create `frontend/.env` (see `frontend/.env.example`)

Start the dev server: `npm run dev`
