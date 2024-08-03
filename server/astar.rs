#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../metadata/astar.scale")]
pub mod astar {}

async fn astar_collector() -> Result<()> {
    let astar_api =
        OnlineClient::<PolkadotConfig>::from_url("wss://astar.api.onfinality.io/public:443").await?;

    // Validate the generated code against the node we are connected to.
    if let Err(e) = astar::validate_codegen(&astar_api) {
        eprintln!("Generated code is not up to date with node we're connected to: {e}");
        std::process::exit(1);
    }

    let astar_genesis_hash = astar_api.rpc().genesis_hash().await?;
    println!("astar genesis hash: {astar_genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
