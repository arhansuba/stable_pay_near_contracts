use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
use near_sdk::json_types::{U128, ValidAccountId};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardsAndIncentivesContract {
    owner: AccountId,
    user_rewards: LookupMap<AccountId, Balance>, // Rewards balance in stablecoins
    stablecoin_contract: AccountId, // NEP-141 stablecoin contract for rewards
}

impl Default for RewardsAndIncentivesContract {
    fn default() -> Self {
        panic!("RewardsAndIncentivesContract should be initialized before usage")
    }
}

#[near_bindgen]
impl RewardsAndIncentivesContract {
    #[init]
    pub fn new(owner: AccountId, stablecoin_contract: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            user_rewards: LookupMap::new(b"r".to_vec()),
            stablecoin_contract: stablecoin_contract.into(),
        }
    }

    // Distribute rewards in stablecoin to a user
    pub fn distribute_rewards(&mut self, user_id: ValidAccountId, amount: U128) -> Promise {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can distribute rewards");

        Promise::new(self.stablecoin_contract.clone()).function_call(
            "ft_transfer".to_string().into_bytes(),
            json!({
                "receiver_id": user_id,
                "amount": amount,
                "memo": "Reward distribution"
            }).to_string().into_bytes(),
            1, // Attached deposit may vary based on stablecoin contract
            env::prepaid_gas() / 2, // Allocate gas for function call
        )
    }

    // Additional functionality for managing rewards...

    // Get rewards balance of a user
    pub fn get_user_rewards(&self, user_id: ValidAccountId) -> U128 {
        self.user_rewards.get(user_id.as_ref()).unwrap_or(0).into()
    }

    // Withdraw rewards by the user
    pub fn withdraw_rewards(&mut self) -> Promise {
        let user_id = env::predecessor_account_id();
        let amount = self.user_rewards.get(&user_id).unwrap_or(0);
        assert!(amount > 0, "No rewards to withdraw");

        self.user_rewards.insert(&user_id, &0);
        Promise::new(user_id.clone()).transfer(amount)
    }

    // Add rewards to a user's balance
    pub fn add_rewards(&mut self, user_id: ValidAccountId, amount: U128) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can add rewards");

        let balance = self.user_rewards.get(user_id.as_ref()).unwrap_or(0);
        self.user_rewards.insert(user_id.as_ref(), &(balance + amount.0));
    }
}
