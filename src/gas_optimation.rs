use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Gas, Promise, Balance};

// Define common constants for gas calculation
const BASE_GAS: Gas = 25_000_000_000_000;
const SINGLE_CALL_GAS: Gas = BASE_GAS * 5;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OptimizedContract {
    owner: AccountId,
    data: LookupMap<String, String>,
    lazy_data: Vector<String>,
}

#[near_bindgen]
impl OptimizedContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            owner: owner_id,
            data: LookupMap::new(b"d"),
            lazy_data: Vector::new(b"l"),
        }
    }

    // Method to demonstrate optimized storage access
    pub fn store_data(&mut self, key: String, value: String) {
        // Store data efficiently with minimum writes
        self.data.insert(&key, &value);
        env::log_str(&format!("Data stored successfully: {} -> {}", key, value));
    }

    // Lazy initialization pattern example
    pub fn get_or_init_data(&mut self, index: u64) -> String {
        if self.lazy_data.get(index).is_none() {
            let init_value = "Lazy Init Value".to_string();
            self.lazy_data.push(&init_value);
            return init_value;
        }
        self.lazy_data.get(index).unwrap()
    }

    // Batch processing example to minimize gas usage
    pub fn batch_process(&mut self, transactions: Vec<(AccountId, Balance)>) -> Promise {
        let mut promise_chain = Promise::new(env::current_account_id()).transfer(1);
        for (account_id, amount) in transactions.iter() {
            promise_chain = promise_chain.then(
                Promise::new(account_id.clone())
                    .transfer(*amount)
                    .gas(SINGLE_CALL_GAS)
            );
        }
        promise_chain
    }

    // Advanced method utilizing pre-computations and optimized logic flow
    pub fn advanced_calculation(&self, inputs: Vec<u128>) -> u128 {
        // Example of an optimized computational method which minimizes
        // the computational overhead by using memoization or pre-computation
        inputs.iter().fold(0, |acc, &val| acc.wrapping_add(val * 2))
    }
}

// Note: Advanced usage may require implementing additional methods for state management,
// efficient data retrieval, and processing. Always test for gas usage to ensure
// your contract remains efficient across different operations.