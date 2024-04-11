use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RewardsAndIncentivesContract {
    owner: AccountId,
    user_rewards: LookupMap<AccountId, Balance>, // Tracks rewards balance for each user
}

#[near_bindgen]
impl RewardsAndIncentivesContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { 
            owner,
            user_rewards: LookupMap::new(b"u"),
        }
    }

    // Method to award rewards to a user
    pub fn award_rewards(&mut self, user: AccountId, amount: Balance) {
        self.assert_owner();
        let current_balance = self.user_rewards.get(&user).unwrap_or(0);
        self.user_rewards.insert(&user, &(current_balance + amount));
        env::log_str(&format!("Awarded {} rewards to {}", amount, user));
    }

    // Method for users to redeem their rewards
    pub fn redeem_rewards(&mut self, user: AccountId, amount: Balance) -> PromiseOrValue<bool> {
        let current_balance = self.user_rewards.get(&user).unwrap_or(0);
        assert!(current_balance >= amount, "Insufficient rewards balance");

        // Redemption logic here. Example: convert rewards to tokens or access to features
        // For simplicity, let's just decrease the balance
        self.user_rewards.insert(&user, &(current_balance - amount));
        env::log_str(&format!("User {} redeemed {} rewards", user, amount));

        // In a real use case, you might want to call another contract or perform additional operations
        PromiseOrValue::Value(true)
    }

    // Utility function to check the rewards balance of a user
    pub fn check_rewards_balance(&self, user: AccountId) -> Balance