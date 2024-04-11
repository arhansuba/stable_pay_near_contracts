#!/bin/bash

# Deploy script for deploying smart contracts to the NEAR testnet

# Exit immediately if a command exits with a non-zero status
set -e

# NEAR Testnet Account ID where the contracts will be deployed
NEAR_ACCOUNT="devacctestnet.testnet"

# Compile Rust smart contracts
cargo build --target wasm32-unknown-unknown --release

# Deploy Core Payment Contract
near deploy --wasmFile target/wasm32-unknown-unknown/release/core_payment.wasm --accountId $NEAR_ACCOUNT

# Deploy Payment Requests Contract
near deploy --wasmFile target/wasm32-unknown-unknown/release/payment_requests.wasm --accountId $NEAR_ACCOUNT

# Deploy Rewards Contract
near deploy --wasmFile target/wasm32-unknown-unknown/release/rewards.wasm --accountId $NEAR_ACCOUNT

# Deploy Dispute Resolution Contract
near deploy --wasmFile target/wasm32-unknown-unknown/release/dispute_resolution.wasm --accountId $NEAR_ACCOUNT

echo "Deployment to NEAR testnet completed successfully."
