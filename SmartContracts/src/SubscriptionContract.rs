use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Timestamp};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SubscriptionContract {
    owner: AccountId,
    subscriptions: LookupMap<AccountId, Subscription>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Subscription {
    start_date: Timestamp,
    end_date: Timestamp,
    tier: SubscriptionTier,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum SubscriptionTier {
    Basic,
    Premium,
    // Add more tiers as needed
}

#[near_bindgen]
impl SubscriptionContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");
        Self {
            owner: env::signer_account_id(),
            subscriptions: LookupMap::new(b"s"),
        }
    }

    // Methods for managing subscriptions, updating tiers, checking subscription status, etc.
}