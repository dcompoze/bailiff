# Bailiff

A cross-chain transaction collector for tax purposes.

- `bailiff-ethereum` collects common ethereum transactions (transfers, DeFi protocols).
- `bailiff-bitcoin` collects common bitcoin transaction (transfers).
- `bailiff-polkadot` collects common polkadot, kusama and substrate parachain transactions (transfers, DeFi protocols, XCM transactions)
- `bailiff-solana` collects common solana transactions (transfers, DeFi protocols).

A list of blockchain accounts is provided in a configuration file.

PostgreSQL is used to store captured transactions.

Alloy crate is used to interact with the the Ethreum blockchain and generate smart contract bindings.

Subxt is used to interact with Polkadot, Kusama and Substrate-based parachains.

Anchor client is used to interact with the Solana blockchain and on-chain programs.
