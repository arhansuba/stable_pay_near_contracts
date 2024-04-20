// src/config/network_config.rs

/// Configuration related to the NEAR blockchain network.
pub struct NetworkConfig {
    /// The network ID (e.g., "testnet", "mainnet").
    pub network_id: &'static str,

    /// The RPC endpoint URL for interacting with the NEAR blockchain.
    pub rpc_endpoint: &'static str,

    /// The Explorer URL for viewing transactions and accounts.
    pub explorer_url: &'static str,
}

/// Provides a default configuration for the NEAR testnet.
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_id: "testnet",
            rpc_endpoint: "https://rpc.testnet.near.org",
            explorer_url: "https://explorer.testnet.near.org",
        }
    }
}