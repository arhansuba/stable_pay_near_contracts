// src/rewards.rs

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault};
use near_sdk::collections::LookupMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Rewards {
    rewards: LookupMap<AccountId, Balance>,
}

impl Default for Rewards {
    fn default() -> Self {
        Self {
            rewards: LookupMap::new(b"r"),
        }
    }
}

#[near_bindgen]
impl Rewards {
    // Add rewards to a user's account
    pub fn add_rewards(&mut self, account_id: AccountId, amount: Balance) {
        let current_balance = self.rewards.get(&account_id).unwrap_or(0);
        self.rewards.insert(&account_id, &(current_balance + amount));
        near_sdk::env::log_str(&format!("Added {} rewards to account {}", amount, account_id));
    }

    // Check the reward balance of a user
    pub fn get_reward_balance(&self, account_id: AccountId) -> Balance {
        self.rewards.get(&account_id).unwrap_or(0)
    }

    // Withdraw rewards (converts rewards into a balance that can be spent)
    pub fn withdraw_rewards(&mut self, account_id: AccountId) -> Balance {
        let balance = self.rewards.get(&account_id).unwrap_or(0);
        assert!(balance > 0, "No rewards available for withdrawal.");

        // Assuming a function exists to transfer balance from rewards to user's main account
        // transfer_rewards_to_main_account(account_id, balance);

        self.rewards.insert(&account_id, &0); // Reset rewards balance
        near_sdk::env::log_str(&format!("Withdrawn {} rewards for account {}", balance, account_id));
        balance
    }
}