use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct UpgradeableContract {
    owner: AccountId,
    // Example state that you might want to preserve across upgrades
    data: UnorderedMap<String, String>,
}

#[near_bindgen]
impl UpgradeableContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            data: UnorderedMap::new(b"d".to_vec()),
        }
    }

    // Function to allow the owner to upgrade the contract
    pub fn upgrade(&self, code: Vec<u8>) {
        self.assert_owner();
        // Deploy the new code to the current contract account
        near_sdk::env::deploy_contract(&code);
        // Note: State migration logic might be needed here if the new contract version requires it
    }

    // Helper function to assert that the caller is the owner of the contract
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the contract owner can call this method"
        );
    }

    // Add your contract methods here
}
