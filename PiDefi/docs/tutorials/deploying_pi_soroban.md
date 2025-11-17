# Tutorial: Deploying PiSoroban.rs

## Overview
This tutorial deploys PiSoroban.rs, PiDefi's core stablecoin contract, to Stellar's Soroban testnet. Includes hyper-tech features like AI stabilization and quantum security.

## Prerequisites
- Rust and Soroban CLI installed.
- Stellar account with XLM.
- Git clone of PiDefi repo.

## Step 1: Setup Environment
1. Install Soroban CLI: `cargo install soroban-cli`.
2. Generate keys: `./scripts/quantum_keygen.py --num 1`.
3. Fund testnet account: Use Stellar Laboratory.

## Step 2: Build Contracts
```bash
cd contracts
soroban contract build
```
- Output: WASM files in `target/wasm32-unknown-unknown/release/`.

## Step 3: Deploy to Testnet
```bash
./scripts/deploy.sh
```
- Sets NETWORK=testnet.
- Deploys StableCoin, PriceOracle, etc.

## Step 4: Configure Hyper-Tech Features
- **AI Stabilization**: Update config/ai_config.yaml for model paths.
- **Quantum Security**: Use generated keys in deploy.sh.
- **Holographic Viz**: Enable in config/stellar.toml.

## Step 5: Test Deployment
```bash
soroban contract invoke --id <stable_coin_id> --method total_supply
```
- Expected: 100 billion Pi Coin.

## Step 6: Integrate Bots
- Run StabilizationBot.go to monitor peg.
- Use SDK for interactions.

## Troubleshooting
- **Build Fails**: Check Rust version.
- **Deploy Fails**: Verify XLM balance.
- **AI Not Working**: Retrain models.

## Advanced Tips
- Monitor with analytics engine.
- Use CI/CD for automated deploys.

For more, see [architecture.md](architecture.md).
