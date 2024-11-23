#![allow(unused)]
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Error, Result};
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

const ETHEREUM_ENDPOINT: &str = "https://sepolia.infura.io/v3/123";

async fn tx_collector() -> Result<()> {
    Ok(())
}
