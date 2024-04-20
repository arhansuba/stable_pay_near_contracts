use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Balance, Timestamp, log};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct LoyaltyRewardsContract {
    owner: AccountId,
    // Maps user account to their rewards info
    user_rewards: LookupMap<AccountId, RewardsInfo>,
    // Tracks all rewards, useful for audit or administrative purposes
    rewards_log: Vector<RewardAction>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardsInfo {
    balance: Balance,
    tier: RewardTier,
    expiration: Timestamp, // When the reward points expire
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum RewardTier {
    Silver,
    Gold,
    Platinum,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardAction {
    user: AccountId,
    action: String,
    amount: Balance,
    timestamp: Timestamp,
}

#[near_bindgen]
impl LoyaltyRewardsContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { 
            owner,
            user_rewards: LookupMap::new(b"u"),
            rewards_log: Vector::new(b"l"),
        }
    }

    // Adds reward points to a user's account
    pub fn add_rewards(&mut self, user: AccountId, amount: U128) {
        let mut rewards_info = self.user_rewards.get(&user).unwrap_or_else(|| RewardsInfo {
            balance: 0,
            tier: RewardTier::Silver,
            expiration: env::block_timestamp() + 31_536_000_000_000_000, // One year from now
        });

        rewards_info.balance += amount.0;
        rewards_info.tier = self.update_tier(rewards_info.balance);

        self.user_rewards.insert(&user, &rewards_info);
        self.log_reward_action(user.clone(), "Add".to_string(), amount.0);
        
        log!("Added {} points to user {}. New tier: {:?}", amount.0, user, rewards_info.tier);
    }

    // Redeems rewards for a user
    pub fn redeem_rewards(&mut self, user: AccountId, amount: U128) {
        let mut rewards_info = self.user_rewards.get(&user).expect("User has no rewards");
        assert!(rewards_info.balance >= amount.0, "Insufficient reward balance");
        assert!(rewards_info.expiration > env::block_timestamp(), "Reward points expired");

        rewards_info.balance -= amount.0;
        self.user_rewards.insert(&user, &rewards_info);
        self.log_reward_action(user.clone(), "Redeem".to_string(), amount.0);

        log!("User {} redeemed {} points", user, amount.0);
    }

    // Private helper method to update user's tier based on balance
    fn update_tier(&self, balance: Balance) -> RewardTier {
        match balance {
            0..=999 => RewardTier::Silver,
            1000..=4999 => RewardTier::Gold,
            _ => RewardTier::Platinum,
        }
    }

    // Logs reward actions for audit and tracking
    fn log_reward_action(&mut self, user: AccountId, action: String, amount: Balance) {
        let reward_action = RewardAction {
            user,
            action,
            amount,
            timestamp: env::block_timestamp(),
        };
        self.rewards_log.push(&reward_action);
    }

    // Additional methods for administrative purposes, user queries, etc...
}