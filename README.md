# BOOBI-PROJECT

This project contains a minimal example of a Solana arbitrage bot written in Rust.

The bot connects to the devnet cluster using `anchor-client` and queries the
[Jupiter Aggregator](https://quote-api.jup.ag/) for SOL/USDC quotes. It detects if
a potential profit above 0.3% exists between two DEX routes and prints the
opportunity. Transactions are simulated only.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

The bot expects to find a local keypair at `~/.config/solana/id.json`.

## Extending

Modify `SOL_MINT` and `USDC_MINT` in `src/main.rs` to target different pairs, or
loop `ArbitrageBot::check` as needed.
