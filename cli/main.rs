#![allow(unused)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, FromPrimitive};
use clap::{arg, ArgAction, Command};
use colored::Colorize;
use sqlx::pool::Pool;
use sqlx::postgres::{PgPool, PgPoolOptions, Postgres};
use subxt::{OnlineClient, PolkadotConfig};

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

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    // Get the required data from the database with an SQL query (do as much work as possible with SQL).

    // Load the data into a polars data frame.

    // Use the data frame to calculate costs, profits, etc. for the given year.

    // From the resulting data frame calculate the taxes and print a report.

    println!("Tax year: [2021 April 6, 2022 April 5]");

    purchases(&pool).await?;
    disposals(&pool).await?;
    swaps(&pool).await?;
    airdrops(&pool).await?;
    rewards(&pool).await?;
    loans(&pool).await?;

    // println!("{:<30}{:>20}", "Year:", cmd.get_one::<String>("year").unwrap());
    // println!("");
    // println!("{:<30}{:>20}", "From:", "2020 April 6");
    // println!("{:<30}{:>20}", "To:", "2021 April 5");
    // println!("");
    // println!("{:<30}{:>20}", "Carry over losses:", "£0.0".red());
    // println!("{:<30}{:>20}", "Carry over acquisition cost:", "£50,000.00".yellow());
    // println!("");
    // println!("{:<30}{:>20}", "Loans:", "£8,840.00".blue());
    // println!("{:<30}{:>20}", "Disposals:", "£22,000.00".green());
    // println!("{:<30}{:>20}", "Acquisition cost:", "£4,000.00".red());
    // println!("{:<30}{:>20}", "Purchases:", "£0.00".yellow());
    // println!("{:<30}{:>20}", "Airdrops:", "£1,000.00".green());
    // println!("{:<30}{:>20}", "Losses:", "£3,000.00".red());
    // println!("{:<30}{:>20}", "Rewards:", "£0.00".green());
    // println!("{:<30}{:>20}", "Salary:", "£0.00".bright_black());
    // println!("");
    // println!("{:<30}{:>20}", "Income:", "£5,503.00".green());
    // println!("{:<30}{:>20}", "Personal allowance:", "£12,570.00");
    // println!("{:<30}{:>20}", "Twenty bracket:", "£0.00");
    // println!("{:<30}{:>20}", "Fourty bracket:", "£0.00");
    // println!("{:<30}{:>20}", "Tax:", "£0.00".red());
    // println!("");
    // println!("{:<30}{:>20}", "Capital gains:", "£18,093.00".green());
    // println!("{:<30}{:>20}", "Personal allowance:", "£12,300.00");
    // println!("{:<30}{:>20}", "Ten bracket:", "£6,093.00");
    // println!("{:<30}{:>20}", "Twenty bracket:", "£0.00");
    // println!("{:<30}{:>20}", "Tax:", "£1.203.00".red());

    Ok(())
}
