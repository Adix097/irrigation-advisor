-- crops table: stores FAO-56 crop coefficient (Kc) values per growth stage
CREATE TABLE crops (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    kc_initial NUMERIC(3, 2) NOT NULL,
    kc_mid NUMERIC(3, 2) NOT NULL,
    kc_late NUMERIC(3, 2) NOT NULL,
    total_growth_days INTEGER NOT NULL
);

-- soil types: affects how efficiently irrigation water is retained vs lost
CREATE TABLE soil_types (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    field_capacity NUMERIC(5, 2) NOT NULL,   -- mm of water per meter of soil depth
    infiltration_rate NUMERIC(5, 2) NOT NULL -- mm per hour
);

-- farm profiles: user-submitted data, one row per saved farm
CREATE TABLE farm_profiles (
    id SERIAL PRIMARY KEY,
    profile_name TEXT NOT NULL,
    location_lat NUMERIC(9, 6) NOT NULL,
    location_lon NUMERIC(9, 6) NOT NULL,
    crop_id INTEGER NOT NULL REFERENCES crops(id),
    soil_type_id INTEGER NOT NULL REFERENCES soil_types(id),
    field_area_hectares NUMERIC(6, 2) NOT NULL,
    pump_power_kw NUMERIC(5, 2) NOT NULL,
    pump_flow_rate_lpm NUMERIC(7, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- seed data: common crops with realistic FAO-56 reference Kc values
INSERT INTO crops (name, kc_initial, kc_mid, kc_late, total_growth_days) VALUES
    ('Wheat', 0.40, 1.15, 0.40, 150),
    ('Rice', 1.05, 1.20, 0.90, 120),
    ('Cotton', 0.35, 1.20, 0.70, 180),
    ('Sugarcane', 0.40, 1.25, 0.75, 300),
    ('Maize', 0.30, 1.20, 0.60, 120);

-- seed data: common soil types with representative field capacity / infiltration values
INSERT INTO soil_types (name, field_capacity, infiltration_rate) VALUES
    ('Sandy', 90.00, 20.00),
    ('Loamy', 150.00, 10.00),
    ('Clay', 200.00, 3.00);