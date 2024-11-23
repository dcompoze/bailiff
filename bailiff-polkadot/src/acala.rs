#![allow(unused)]

use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata/acala.scale")]
pub mod acala {}

async fn acala_collector() -> Result<()> {
    let acala_api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    let genesis_hash = acala_api.genesis_hash();

    println!("acala genesis hash: {genesis_hash:?}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
