use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct UserManagementContract {
    owner: AccountId,
    users: LookupMap<AccountId, UserProfile>,
    access_levels: LookupMap<AccountId, AccessLevel>,
    // Optional: Track users' registration timestamps
    registration_dates: LookupMap<AccountId, u64>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserProfile {
    username: String,
    bio: String,
    // Extend with more fields as necessary
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq)]
pub enum AccessLevel {
    RegularUser,
    Moderator,
    Administrator,
}

#[near_bindgen]
impl UserManagementContract {
    #[init]
    pub fn new() -> Self {
        let owner_id = env::signer_account_id();
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: owner_id.clone(),
            users: LookupMap::new(b"u"),
            access_levels: LookupMap::new(b"a"),
            registration_dates: LookupMap::new(b"r"),
        }
        .with_default_access_levels(owner_id)
    }

    // Allow users to register
    pub fn register_user(&mut self, username: String, bio: String) -> PromiseOrValue<bool> {
        let caller_id = env::signer_account_id();
        assert!(!self.users.contains_key(&caller_id), "User already exists");
        
        let profile = UserProfile { username, bio };
        self.users.insert(&caller_id, &profile);
        self.access_levels.insert(&caller_id, &AccessLevel::RegularUser);
        self.registration_dates.insert(&caller_id, &env::block_timestamp());

        log!("User registered: {}", caller_id);
        PromiseOrValue::Value(true)
    }

    // Profile update method
    pub fn update_profile(&mut self, username: Option<String>, bio: Option<String>) {
        let caller_id = env::signer_account_id();
        let mut profile = self.users.get(&caller_id).expect("User does not exist");

        if let Some(new_username) = username {
            profile.username = new_username;
        }
        if let Some(new_bio) = bio {
            profile.bio = new_bio;
        }

        self.users.insert(&caller_id, &profile);
        log!("Profile updated for user: {}", caller_id);
    }

    // Method for changing access levels - restricted to owner
    pub fn change_access_level(&mut self, user_id: AccountId, new_level: AccessLevel) {
        self.assert_owner();
        assert!(self.users.contains_key(&user_id), "User does not exist");
        
        self.access_levels.insert(&user_id, &new_level);
        log!("Access level changed for user: {} to {:?}", user_id, new_level);
    }

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can call this method.");
    }

    fn with_default_access_levels(mut self, owner_id: AccountId) -> Self {
        self.access_levels.insert(&owner_id, &AccessLevel::Administrator);
        self
    }

    // Additional methods for querying user profiles, access levels, etc.
}
