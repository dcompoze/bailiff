#![allow(unused)]

use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use btcrpc::{bitcoin, Auth, Client as BTCClient, Error as BTCError, Result as BTCResult, RpcApi};
use sqlx::postgres::PgPool;

async fn bitcoin_collector() -> Result<()> {
    let btc_client = BTCClient::new(
        "http://server.lan:8332",
        Auth::UserPass("core".to_string(), "core".to_string()),
    )?;

    let btc_best_block = btc_client.get_best_block_hash()?;
    println!("{btc_best_block}");

    let db_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let addresses = get_addresses();

    Ok(())
}

fn get_addresses() -> HashMap<String, String> {
    let mut addresses: HashMap<String, String> = HashMap::new();
    addresses.insert(
        "LedgerX1".into(),
        "bc1q6lcw9xmzwqeranmz80vwu9934r5m7p2y0yvcwe".into(),
    );
    addresses.insert(
        "LedgerX2".into(),
        "bc1q0z0wdhzhwj7ejaglkzkpq464ltht9k9taznxwp".into(),
    );
    addresses.insert("LedgerSegwit".into(), "3GTGySPUMEZ5VPU98TMcffdxAbT7VEkoMP".into());
    addresses.insert(
        "LedgerWork".into(),
        "bc1qyt9eentevqzhk2jvqugadswc7pydudnu623023".into(),
    );
    addresses.insert("Trade1".into(), "1PzYkddDMKQvgo1195uZ3kgr4YopDSt3aV".into());
    addresses.insert("Trade2".into(), "1Fogu9HMviADhn6jD8Yo6JnSZbNbeCQneT".into());
    addresses.insert("Trade3".into(), "18x2M3whDDmxhCsG5N5oKhMNNK1XAhNDrg".into());
    addresses.insert("Electrum1".into(), "1sgbgzyocbtvrk5e6q8kwncywdws6uzaj".into());
    addresses.insert("Electrum2".into(), "1podj6h2narhremve1uhgdnfmzrnvt7dou".into());
    addresses.insert("Electrum3".into(), "1pjtjullmdhfrwsozg3rukirac7gqxnbzz".into());
    addresses.insert("Electrum4".into(), "1mqgawesvxnkytqyuqenxe6mdwcl9lmumn".into());
    addresses.insert("Electrum5".into(), "1lhmg9xsgvknt1jfqcer51h4msyejauiqj".into());
    addresses
}
