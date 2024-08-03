#![allow(unused)]

use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

fn get_ethereum_addresses() -> HashMap<String, String> {
    let mut addresses: HashMap<String, String> = HashMap::new();
    addresses.insert(
        "Store".into(),
        "0xc87894D37EF0f2A4285e544215eFF06414e6E9cb".into(),
    );
    addresses.insert("Work".into(), "0xdad6a71A31e6fF08eb2E495d9efF13f5c7E75b29".into());
    addresses.insert(
        "Trade1".into(),
        "0xa77Ca618518bc2884a0b898A1A2e24F34E941315".into(),
    );
    addresses.insert(
        "Trade2".into(),
        "0xdB71247d2f0fc9530C200bAC025646827E0B7a5a".into(),
    );
    addresses.insert(
        "Salary".into(),
        "0x590C99bB7fe590480963fdf222f33620d84Dd236".into(),
    );
    addresses.insert(
        "Secondary".into(),
        "0x3627d93560893B0Bc34785f666100b7d0691A4a6".into(),
    );
    addresses.insert(
        "Default".into(),
        "0x528Fd9e7D88F954951A15b1A30B4073cb7B3BF1D".into(),
    );
    addresses.insert(
        "LedgerX3".into(),
        "0xf080A19a4a3161e18C6e377d4c7d3F58198656d5".into(),
    );
    addresses.insert(
        "Moonbeam".into(),
        "0xe96029F0251fcAe87ce85300263408bCfF8c9bcB".into(),
    );
    addresses.insert(
        "Moonriver".into(),
        "0xcfcc153a0332eD72a5eC4C914F5e109B8c7987a6".into(),
    );
    addresses.insert(
        "LedgerX1".into(),
        "0x0ce5f3241F2fC075d0b083bBcd17E7C6067cf101".into(),
    );
    addresses.insert(
        "LedgerX2".into(),
        "0xF65492A4896D3cC6DE3aE274d90D29a7EF5AE520".into(),
    );
    addresses.insert(
        "LedgerS1".into(),
        "0x8FC7e30adC6f94Ba662005E12447DD3F554998c2".into(),
    );
    addresses
}
