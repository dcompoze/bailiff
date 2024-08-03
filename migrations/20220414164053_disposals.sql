CREATE TABLE IF NOT EXISTS disposals (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  base_asset_name TEXT,
  base_asset_amount DECIMAL,
  quote_asset_name TEXT,
  quote_asset_amount DECIMAL,
  quote_pound_value DECIMAL,
  exchange_name TEXT,
  exchange_transaction_id TEXT
);
