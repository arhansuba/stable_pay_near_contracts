use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, Promise, PromiseResult};

const BASE_GAS: Gas = 25_000_000_000_000; // Base gas for function calls
const DEPOSIT_GAS: Gas = BASE_GAS * 10; // Gas allocated for deposit calls
const CALLBACK_GAS: Gas = BASE_GAS * 5; // Gas allocated for handling callbacks

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DeFiIntegration {
    owner: AccountId,
    supported_protocols: Vector<AccountId>, // List of supported DeFi protocols
    deposits: LookupMap<AccountId, Balance>, // Mapping of user deposits per protocol
}

#[near_bindgen]
impl DeFiIntegration {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            supported_protocols: Vector::new(b"supported_protocols"),
            deposits: LookupMap::new(b"deposits"),
        }
    }

    // Registers a DeFi protocol as supported by this contract
    pub fn add_supported_protocol(&mut self, protocol_address: AccountId) {
        self.assert_owner();
        self.supported_protocols.push(&protocol_address);
    }

    // Function to deposit tokens to a DeFi protocol with callback for confirmation
    pub fn deposit_to_defi(&mut self, user_id: AccountId, amount: Balance, protocol_address: AccountId) -> Promise {
        assert!(self.is_protocol_supported(&protocol_address), "Protocol not supported");
        env::log_str(&format!("Initiating deposit of {} to {}", amount, protocol_address));

        Promise::new(protocol_address.clone())
            .function_call(
                "deposit".to_string(),
                &serde_json::to_vec(&amount).unwrap(),
                1, // Attached deposit (adjust based on protocol requirements)
                DEPOSIT_GAS,
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(CALLBACK_GAS)
                    .on_deposit_complete(user_id, amount, protocol_address, env::current_account_id()),
            )
    }

    // Callback function to handle deposit completion
    #[private]
    pub fn on_deposit_complete(
        &mut self, 
        user_id: AccountId, 
        amount: Balance, 
        protocol_address: AccountId,
        #[callback_result] call_result: Result<(), PromiseResult>,
    ) {
        match call_result {
            Ok(_) => {
                let key = self.deposit_key(&user_id, &protocol_address);
                let new_balance = self.deposits.get(&key).unwrap_or(0) + amount;
                self.deposits.insert(&key, &new_balance);
                env::log_str(&format!("Deposit of {} to {} completed", amount, protocol_address));
            },
            Err(_) => env::log_str(&format!("Deposit to {} failed", protocol_address)),
        }
    }

    // Utility functions
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the contract owner can call this method"
        );
    }

    fn is_protocol_supported(&self, protocol_address: &AccountId) -> bool {
        // Check if a protocol is supported
        for i in 0..self.supported_protocols.len() {
            if &self.supported_protocols.get(i).unwrap() == protocol_address {
                return true;
            }
        }
        false
    }
    
    fn deposit_key(&self, user_id: &AccountId, protocol_address: &AccountId) -> String {
        format!("{}:{}", user_id, protocol_address)
    }
}

near_sdk::ext_contract!(ext_self {
    fn on_deposit