CREATE TABLE IF NOT EXISTS swaps (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  base_asset_name TEXT,
  base_asset_amount DECIMAL,
  base_pound_value DECIMAL,
  base_dollar_value DECIMAL,
  quote_asset_name TEXT,
  quote_asset_amount DECIMAL,
  exchange_name TEXT,
  exchange_transaction_id TEXT
);
