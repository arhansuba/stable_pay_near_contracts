use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, assert_one_yocto, PromiseOrValue};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::utils::assert_self;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StakingContract {
    owner: AccountId,
    total_staked: Balance,
    stakes: LookupMap<AccountId, StakeInfo>,
    reward_rate_per_epoch: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeInfo {
    amount: Balance,
    last_staked_epoch: u64,
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new(owner_id: AccountId, reward_rate: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { 
            owner: owner_id,
            total_staked: 0,
            stakes: LookupMap::new(b"s"),
            reward_rate_per_epoch: reward_rate.into(),
        }
    }

    // Allows users to stake tokens into the contract
    #[payable]
    pub fn stake(&mut self) {
        let account_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Deposit must be greater than 0");
        
        let mut stake_info = self.stakes.get(&account_id).unwrap_or(StakeInfo {
            amount: 0,
            last_staked_epoch: env::epoch_height(),
        });

        stake_info.amount += deposit;
        self.total_staked += deposit;
        self.stakes.insert(&account_id, &stake_info);
    }

    // Calculates rewards for the caller based on the staked amount and duration
    pub fn calculate_rewards(&self, account_id: AccountId) -> U128 {
        let stake_info = self.stakes.get(&account_id).expect("No stake found for account");
        let epochs_staked = env::epoch_height() - stake_info.last_staked_epoch;
        
        let rewards = (stake_info.amount * self.reward_rate_per_epoch * epochs_staked) / 1_000_000_000;
        U128(rewards)
    }

    // Withdraw stake and rewards
    pub fn withdraw(&mut self) -> PromiseOrValue<U128> {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let stake_info = self.stakes.get(&account_id).expect("No stake found for account");
        
        let rewards = self.calculate_rewards(account_id).0;
        let total_withdrawal = stake_info.amount + rewards;

        self.total_staked -= stake_info.amount;
        self.stakes.remove(&account_id);
        
        PromiseOrValue::Value(U128(total_withdrawal))
    }

    // Additional methods as necessary...
}