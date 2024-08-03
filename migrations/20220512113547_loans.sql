CREATE TABLE IF NOT EXISTS loans (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  collateral_asset_name TEXT,
  collateral_asset_amount DECIMAL,
  loan_asset_name TEXT,
  loan_asset_amount DECIMAL,
  loan_provider_name TEXT,
  loan_transaction_id TEXT
);
