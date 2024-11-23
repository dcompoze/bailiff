#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../metadata/karura.scale")]
pub mod karura {}

async fn karura_collector() -> Result<()> {
    let karura_api = OnlineClient::<PolkadotConfig>::from_url("ws://server.lan:3744").await?;

    // Validate the generated code against the node we are connected to.
    if let Err(e) = karura::validate_codegen(&karura_api) {
        eprintln!("Generated code is not up to date with node we're connected to: {e}");
        std::process::exit(1);
    }

    let karura_genesis_hash = karura_api.rpc().genesis_hash().await?;
    println!("Karura genesis hash: {karura_genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
