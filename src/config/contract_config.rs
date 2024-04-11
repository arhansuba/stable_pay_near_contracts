// src/config/contract_config.rs

/// Configuration related to smart contracts.
pub struct ContractConfig {
    /// The account ID of the main smart contract.
    pub main_contract_account_id: &'static str,

    /// Other related contract account IDs or specific method names could be added here.
}

/// Provides a default configuration for easy access.
/// Adjust the contract account IDs according to your deployment.
impl Default for ContractConfig {
    fn default() -> Self {
        Self {
            main_contract_account_id: "main_contract.testnet",
        }
    }
}