use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue, log};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DataPrivacyContract {
    owner: AccountId,
    // Maps user account to their encrypted data
    user_data: LookupMap<AccountId, String>,
    // Tracks authorized viewers of a user's data
    access_control: LookupMap<AccountId, UnorderedSet<AccountId>>,
    // Audit logs for data access
    audit_logs: LookupMap<AccountId, Vector<String>>,
}

#[near_bindgen]
impl DataPrivacyContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: env::signer_account_id(),
            user_data: LookupMap::new(b"u"),
            access_control: LookupMap::new(b"a"),
            audit_logs: LookupMap::new(b"l"),
        }
    }

    // Allows users to store their encrypted data
    pub fn store_encrypted_data(&mut self, user: AccountId, encrypted_data: String) {
        assert_eq!(env::signer_account_id(), user, "Unauthorized");
        self.user_data.insert(&user, &encrypted_data);
        log!("Encrypted data stored for user: {}", user);
    }

    // Users can authorize additional accounts to view their data
    pub fn grant_access(&mut self, viewer: AccountId) {
        let signer = env::signer_account_id();
        let mut viewers = self.access_control.get(&signer).unwrap_or_else(|| UnorderedSet::new(env::sha256(signer.as_bytes())));
        viewers.insert(&viewer);
        self.access_control.insert(&signer, &viewers);
        log!("Access granted to: {} by {}", viewer, signer);
    }

    // Retrieve encrypted data if authorized
    pub fn get_encrypted_data(&self, owner: AccountId) -> PromiseOrValue<String> {
        let signer = env::signer_account_id();
        let authorized_viewers = self.access_control.get(&owner).expect("No access control entry found");

        assert!(authorized_viewers.contains(&signer), "Unauthorized access attempt");
        
        let data = self.user_data.get(&owner).expect("Data not found");
        self.log_access_event(owner.clone(), signer);
        
        PromiseOrValue::Value(data)
    }

    // Logs an access event for auditing purposes
    fn log_access_event(&self, user: AccountId, viewer: AccountId) {
        let mut logs = self.audit_logs.get(&user).unwrap_or_else(|| Vector::new(env::sha256(user.as_bytes())));
        let log_entry = format!("Data accessed by: {} at {}", viewer, env::block_timestamp());
        logs.push(&log_entry);
        self.audit_logs.insert(&user, &logs);
    }

    // Additional methods for revoking access, updating data, etc...
}