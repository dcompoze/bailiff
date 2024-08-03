CREATE TABLE IF NOT EXISTS airdrops (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  asset_name TEXT,
  asset_amount DECIMAL,
  vested BOOLEAN,
  pound_value DECIMAL,
  project_name TEXT,
  transaction_id TEXT,
);
