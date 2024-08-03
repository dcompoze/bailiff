#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../metadata/kusama.scale")]
pub mod kusama {}

async fn kusama_collector() -> Result<()> {
    let kusama_api = OnlineClient::<PolkadotConfig>::from_url("ws://server.lan:6644").await?;

    // Validate the generated code against the node we are connected to.
    if let Err(e) = kusama::validate_codegen(&kusama_api) {
        eprintln!("Generated code is not up to date with node we're connected to: {e}");
        std::process::exit(1);
    }

    let kusama_genesis_hash = kusama_api.rpc().genesis_hash().await?;
    println!("Kusama genesis hash: {kusama_genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}