CREATE TABLE IF NOT EXISTS rewards (
  id SERIAL PRIMARY KEY NOT NULL,
  date_time_zone TIMESTAMPTZ NOT NULL,
  stake_asset_name TEXT,
  stake_asset_amount DECIMAL,
  reward_asset_name TEXT,
  reward_asset_amount DECIMAL,
  reward_pound_value DECIMAL,
  project_name TEXT,
  transaction_id TEXT,
);
