#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../metadata/moonbeam.scale")]
pub mod moonbeam {}

async fn moonbeam_collector() -> Result<()> {
    let moonbeam_api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.api.moonbeam.network:443").await?;

    // Validate the generated code against the node we are connected to.
    if let Err(e) = moonbeam::validate_codegen(&moonbeam_api) {
        eprintln!("Generated code is not up to date with node we're connected to: {e}");
        std::process::exit(1);
    }

    let moonbeam_genesis_hash = moonbeam_api.rpc().genesis_hash().await?;
    println!("moonbeam genesis hash: {moonbeam_genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
