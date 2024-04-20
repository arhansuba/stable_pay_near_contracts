// src/errors.rs
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Balance, AccountId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("The account {0} is not registered.")]
    NotRegistered(AccountId),

    #[error("Insufficient balance: required {required}, available {available}.")]
    InsufficientBalance { required: U128, available: Balance },

    #[error("Invalid parameters: {0}.")]
    InvalidParameters(String),
    
    #[error("Internal error occurred.")]
    InternalError,
}

impl From<ContractError> for String {
    fn from(error: ContractError) -> Self {
        error.to_string()
    }
}