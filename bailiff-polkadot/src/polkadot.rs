#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../metadata/polkadot.scale")]
pub mod polkadot {}

async fn polkadot_collector() -> Result<()> {
    let polkadot_api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.api.moonbeam.network:443").await?;

    // Validate the generated code against the node we are connected to.
    if let Err(e) = polkadot::validate_codegen(&polkadot_api) {
        eprintln!("Generated code is not up to date with node we're connected to: {e}");
        std::process::exit(1);
    }

    let polkadot_genesis_hash = polkadot_api.rpc().genesis_hash().await?;
    println!("Polkadot genesis hash: {polkadot_genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
