#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{anyhow, Result, Error};
use bigdecimal::{BigDecimal, FromPrimitive};
use clap::{arg, ArgAction, Command};
use colored::Colorize;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPool, PgPoolOptions, Postgres};
use subxt::{OnlineClient, PolkadotConfig};
use serde::{Deserialize, Serialize};
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{broadcast, Mutex};
use tokio::time::{sleep, Duration};
use tracing::{info, Level};

// ######################## Setup ########################

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let (shutdown_t, _) = broadcast::channel::<()>(1);

    tokio::select! {
        biased;
        result = tokio::spawn(interrupt_handler()) => {
            tracing::warn!("Interrupt handler exited");
            match result {
                Err(e) => {
                    tracing::error!("Failed to launch interrupt handler task: {}", e);
                }
                Ok(Err(e)) => {
                    tracing::error!("Interrupt handler failed: {}", e);
                }
                _ => {}
            }
            broadcast_shutdown(shutdown_t.clone()).await?;
        },
        result = tokio::spawn(background_task(shutdown_t.subscribe())) => {
            tracing::warn!("Background task exited");
            match result {
                Err(e) => {
                    tracing::error!("Failed to launch background task: {}", e);
                }
                Ok(Err(e)) => {
                    tracing::error!("Background task failed: {}", e);
                }
                _ => {}
            }
            broadcast_shutdown(shutdown_t.clone()).await?;
        },
        result = tokio::spawn(tx_collector()) => {
            tracing::warn!("HTTP server exited");
            if let Err(e) = result {
                tracing::error!("Failed to launch HTTP server task: {}", e);
            }
            broadcast_shutdown(shutdown_t.clone()).await?;
        },
    }
    Ok(())
}

async fn interrupt_handler() -> Result<()> {
    let sigint = signal::ctrl_c();
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = sigint => {
            tracing::info!("Received SIGINT signal, exiting");
        }
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM signal, exiting");
        }
    }
    Ok(())
}

async fn broadcast_shutdown(shutdown_t: broadcast::Sender<()>) -> Result<()> {
    if shutdown_t.receiver_count() > 0 {
        tracing::info!("Sending shutdown signal to all tasks");
        let _ = shutdown_t.send(())?;
        tracing::info!("Waiting 3 sec for graceful shutdown");
        sleep(Duration::from_secs(3)).await;
    }
    Ok(())
}

// ######################## Background task ########################

async fn background_task(mut shutdown_r: broadcast::Receiver<()>) -> Result<()> {
    loop {
        tokio::select! {
            biased;
            _ = shutdown_r.recv() => {
                tracing::info!("<task> received shutdown signal, shutting down");
                return Ok(())
            },
            _ = sleep(Duration::from_secs(3600)) => {
                tracing::info!("<task> completed after 3600 seconds");
            },
        }
    }
}

// ######################## Transaction collector ########################

async fn tx_collector() -> Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    // Get the required data from the database with an SQL query (do as much work as possible with SQL).

    // Load the data into a polars data frame.

    // Use the data frame to calculate costs, profits, etc. for the given year.

    // From the resulting data frame calculate the taxes and print a report.

    purchases(&pool).await?;
    disposals(&pool).await?;
    swaps(&pool).await?;
    airdrops(&pool).await?;
    rewards(&pool).await?;
    loans(&pool).await?;

    Ok(())
}

const DATA_MISSING_ERROR: &str = "data missing";

// Return the pooled totals of purchased assets.
async fn purchases(db: &Pool<Postgres>) -> Result<()> {
    // Get the list of all purchased token names.
    let records = sqlx::query!("SELECT quote_asset_name FROM purchases").fetch_all(db).await?;

    // Create a set of unique asset names.
    let unique_assets: HashSet<_> =
        records
            .into_iter()
            .fold(Ok(HashSet::new()), |set: Result<_>, record| {
                let mut set = set?;
                set.insert(record.quote_asset_name
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?);
                Ok(set)
            })?;

    // For each unique asset, get the list of purchases and their value.
    for asset in unique_assets {
        let records = 
        sqlx::query!(r#"SELECT base_asset_name, base_asset_amount, base_pound_value, quote_asset_amount FROM purchases WHERE quote_asset_name = $1"#, asset)
        .fetch_all(db).await?;

        // Total amount of assets purchased.
        let total_amount = records.iter().fold(Ok(BigDecimal::from_str("0")?), | total: Result<_>, record| {
            let mut total = total?;
            let amount = record.quote_asset_amount
                .as_ref()
                .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            total += amount;
            Ok(total)
        })?;

        // Total GBP value of the asset.
        let total_value = records.iter().fold(Ok(BigDecimal::from_str("0")?), | total: Result<_>, record | {
            let mut total = total?;
            let base_asset_name = record.base_asset_name
                .as_ref()
                .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            if  base_asset_name == "GBP" {
                total += record.base_asset_amount
                    .as_ref()
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            } else if base_asset_name == "EUR" {
                total += record.base_pound_value
                    .as_ref()
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            }
            Ok(total)
        })?;

        // Average GBP cost per unit of asset.
        let average_cost = &total_value / &total_amount;

        println!("Purchase pool: [{:.4} {}, £{:.4}, £{:.4}]", total_amount, asset, average_cost, total_value);
    }

    // DOT purchased with BTC in tax year 0.
    println!("Purchase pool: [7550.87 DOT, £0.9868, £7451.40]");
    Ok(())
}

async fn disposals(db: &Pool<Postgres>) -> Result<()> {
    let records = 
    sqlx::query!(r#"SELECT base_asset_name, base_asset_amount, quote_asset_name, quote_asset_amount, quote_pound_value FROM disposals WHERE date_time_zone >= '2021-04-06' AND date_time_zone <= '2022-04-05'"#)
    .fetch_all(db).await?;

    println!("Disposal count: {}", records.len());

    // Create a set of unique asset names.
    let unique_assets: HashSet<_> =
        records
            .iter()
            .fold(Ok(HashSet::new()), |set: Result<_>, record| {
                let mut set = set?;
                set.insert(record.base_asset_name
                    .as_ref()
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?);
                Ok(set)
            })?;

    // For each unique asset, get the list of disposals and their value.
    for asset in unique_assets {
        let records = 
        sqlx::query!(r#"SELECT base_asset_name, base_asset_amount, quote_asset_name, quote_asset_amount, quote_pound_value FROM disposals WHERE date_time_zone >= '2021-04-06' AND date_time_zone <= '2022-04-05' AND base_asset_name = $1"#, asset)
        .fetch_all(db).await?;

        // Total amount of assets sold.
        let total_amount = records.iter().fold(Ok(BigDecimal::from_str("0")?), | total: Result<_>, record| {
            let mut total = total?;
            let amount = record.base_asset_amount
                .as_ref()
                .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            total += amount;
            Ok(total)
        })?;

        // Total GBP amount received for the given asset.
        let total_value = records.iter().fold(Ok(BigDecimal::from_str("0")?), | total: Result<_>, record | {
            let mut total = total?;
            let quote_asset_name = record.quote_asset_name
                .as_ref()
                .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            if  quote_asset_name == "GBP" {
                total += record.quote_asset_amount
                    .as_ref()
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            } else if quote_asset_name == "EUR" {
                total += record.quote_pound_value
                    .as_ref()
                    .ok_or(anyhow!(DATA_MISSING_ERROR))?;
            }
            Ok(total)
        })?;        
        
        println!("Disposal: [{:.4} {}, £{:.4}]", total_amount, asset, total_value);
    }

    Ok(())
}

async fn swaps(db: &Pool<Postgres>) -> Result<()> {
    // Buying DOT with BTC is a disposal event.
    // Paying off a debt position with kUSD is a disposal event (is that 0 profit).
    let records = 
    sqlx::query!(r#"SELECT base_asset_name, base_asset_amount, base_pound_value, quote_asset_name, quote_asset_amount FROM swaps WHERE date_time_zone >= '2021-04-06' AND date_time_zone <= '2022-04-05'"#)
    .fetch_all(db).await?;

    println!("Swap count: {}", records.len());

    println!("Swap disposal: [? BTC, £?]");

    Ok(())
}

async fn airdrops(_db: &Pool<Postgres>) -> Result<()> {
    // Kusama Karura crowdloan (consider the vesting).
    // Kusama Moonriver crowdloan (consider the vesting).
    // Kusama Shiden crowdloan (consider the vesting).

    println!("Airdrop: [? KAR, £?]");
    println!("Airdrop: [? MOVR, £?]");
    println!("Airdrop: [? SDN, £?]");
    Ok(())
}

async fn rewards(_db: &Pool<Postgres>) -> Result<()> {
    // LKSM staking rewards on Karura.
    // KAR staking rewards on Karura.
    // KSM staking rewards on Karura.
    // kUSD staking rewards on Karura.
    println!("Reward: [? KAR, £?]");
    println!("Reward: [? kUSD, £?]");
    Ok(())
}

async fn loans(_db: &Pool<Postgres>) -> Result<()> {
    // kUSD loan on Karura.
    println!("Loan: [? kUSD, £?]");
    Ok(())
}
