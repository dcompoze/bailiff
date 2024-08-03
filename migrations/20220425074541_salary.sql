CREATE TABLE IF NOT EXISTS salary (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  asset_name TEXT,
  asset_amount DECIMAL,
  pound_tax DECIMAL,
  pound_value DECIMAL,
  dollar_value DECIMAL,
  company_name TEXT,
  transaction_id TEXT
);
