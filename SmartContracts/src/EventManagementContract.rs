use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Timestamp, log};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EventManagementContract {
    owner: AccountId,
    events: LookupMap<u64, Event>,
    // Track attendance
    attendees: LookupMap<u64, UnorderedSet<AccountId>>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Event {
    title: String,
    description: String,
    start_time: Timestamp,
    end_time: Timestamp,
    capacity: u64,
}

#[near_bindgen]
impl EventManagementContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");
        Self {
            owner: env::signer_account_id(),
            events: LookupMap::new(b"e"),
            attendees: LookupMap::new(b"a"),
        }
    }

    // Methods for creating events, registering attendees, managing capacity, etc.
}