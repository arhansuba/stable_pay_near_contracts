use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise, PromiseOrValue};

const MINIMUM_STORAGE_BALANCE: Balance = 10_000_000_000_000_000_000_000; // 0.01 NEAR
const BASE_GAS: Gas = 5_000_000_000_000; // 5 TeraGas
const OPERATIONAL_GAS: Gas = BASE_GAS / 2;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MyContractOptimized {
    owner: AccountId,
    data_entries: UnorderedMap<String, String>,
    calculation_history: Vector<u128>,
    special_accounts: LazyOption<Vector<AccountId>>,
}

#[near_bindgen]
impl MyContractOptimized {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self {
            owner,
            data_entries: UnorderedMap::new(b"d"),
            calculation_history: Vector::new(b"c"),
            special_accounts: LazyOption::new(b"s", None),
        }
    }

    pub fn store_data_entry(&mut self, key: String, value: String) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner can store data");

        // Efficiently write to storage only if the key does not exist or the value is different
        let current_value = self.data_entries.get(&key);
        if current_value.is_none() || current_value.unwrap() != value {
            self.data_entries.insert(&key, &value);
        }
    }

    pub fn complex_computation(&mut self, inputs: Vec<u128>) -> u128 {
        // Example: Perform a complex computation and store the result history
        let result = inputs.iter().fold(0, |acc, &val| acc + val * 2);
        self.calculation_history.push(&result);

        // Optimizes gas by batching state updates
        if self.calculation_history.len() > 1000 {
            self.cleanup_history(); // Custom cleanup logic
        }

        result
    }

    pub fn add_special_account(&mut self, account_id: AccountId) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner can add special account");

        // Lazy initialization pattern
        let mut accounts = self.special_accounts.get().unwrap_or_else(|| Vector::new(b"sa"));
        accounts.push(&account_id);
        self.special_accounts.set(&accounts);
    }

    fn cleanup_history(&mut self) {
        while self.calculation_history.len() > 500 {
            self.calculation_history.pop();
        }
    }

    #[payable]
    pub fn add_funds(&mut self) -> PromiseOrValue<bool> {
        let deposit = env::attached_deposit();
        assert!(deposit >= MINIMUM_STORAGE_BALANCE, "Deposit not sufficient for storage");

        // Example: Manage fund allocation or trigger async operations
        PromiseOrValue::Value(true)
    }

    // Additional advanced methods and logic...
}

near_sdk::impl_near_bindgen!(MyContractOptimized, env, near_bindgen, PanicOnDefault);