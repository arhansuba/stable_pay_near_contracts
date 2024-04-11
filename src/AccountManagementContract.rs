use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AccountManagementContract {
    owner: AccountId,
    accounts: LookupMap<AccountId, UserAccount>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserAccount {
    pub is_active: bool,
    pub data: String, // Simplified - in practice, this could be more structured data
}

impl Default for AccountManagementContract {
    fn default() -> Self {
        env::panic_str("AccountManagementContract should be initialized before usage")
    }
}

#[near_bindgen]
impl AccountManagementContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self { 
            owner,
            accounts: LookupMap::new(b"a"),
        }
    }

    // Registers a new account in the contract
    pub fn register_account(&mut self, account_id: AccountId, data: String) {
        self.assert_owner();
        let new_user_account = UserAccount {
            is_active: true,
            data,
        };
        self.accounts.insert(&account_id, &new_user_account);
    }

    // Updates the data associated with an account
    pub fn update_account_data(&mut self, account_id: AccountId, new_data: String) {
        self.assert_owner();
        if let Some(mut user_account) = self.accounts.get(&account_id) {
            user_account.data = new_data;
            self.accounts.insert(&account_id, &user_account);
        } else {
            env::panic_str("Account not found");
        }
    }

    // Disables an account by setting its 'is_active' flag to false
    pub fn disable_account(&mut self, account_id: AccountId) {
        self.assert_owner();
        if let Some(mut user_account) = self.accounts.get(&account_id) {
            user_account.is_active = false;
            self.accounts.insert(&account_id, &user_account);
        } else {
            env::panic_str("Account not found");
        }
    }

    // Utility function to ensure that only the owner can perform certain operations
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the contract owner can call this method"
        );
    }
}