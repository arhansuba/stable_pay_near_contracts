// src/core_payment.rs

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, AccountId, Balance};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CorePayment {
    accounts: LookupMap<AccountId, Balance>,
}

impl Default for CorePayment {
    fn default() -> Self {
        Self {
            accounts: LookupMap::new(b"a"),
        }
    }
}

#[near_bindgen]
impl CorePayment {
    // Register a new account with an initial balance of 0
    pub fn register_account(&mut self) {
        let account_id = env::signer_account_id();
        // Initialize only if the account doesn't exist
        if self.accounts.get(&account_id).is_none() {
            self.accounts.insert(&account_id, &0);
            env::log_str(&format!("Account {} registered", account_id));
        }
    }

    // Allow a user to send payment to another user
    pub fn send_payment(&mut self, receiver_id: AccountId, amount: U128) {
        let sender_id = env::signer_account_id();
        let sender_balance = self.accounts.get(&sender_id).expect("Sender not registered");
        assert!(amount.0 <= sender_balance, "Insufficient balance");

        let receiver_balance = self.accounts.get(&receiver_id).unwrap_or(0);
        self.accounts.insert(&sender_id, &(sender_balance - amount.0));
        self.accounts.insert(&receiver_id, &(receiver_balance + amount.0));

        env::log_str(&format!("{} sent {} to {}", sender_id, amount.0, receiver_id));
    }

    // Query the balance of an account
    pub fn get_balance(&self, account_id: AccountId) -> U128 {
        U128(self.accounts.get(&account_id).unwrap_or(0))
    }
}