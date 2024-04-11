use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas};

const MIN_DEPOSIT_AMOUNT: Balance = 1_000_000_000_000_000_000_000; // 1 NEAR in yoctoNEAR
const EXCHANGE_GAS: Gas = 50_000_000_000_000; // Gas allowance for exchange transactions

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenExchangeContract {
    owner: AccountId,
    // Maps token pair to their exchange rate (e.g., "TOKEN_A:TOKEN_B" -> rate)
    exchange_rates: LookupMap<String, f64>,
    // Tracks user deposits for each token
    user_deposits: LookupMap<AccountId, LookupMap<String, Balance>>,
}

#[near_bindgen]
impl TokenExchangeContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            exchange_rates: LookupMap::new(b"r"),
            user_deposits: LookupMap::new(b"d"),
        }
    }

    // Allows the owner to set exchange rates between token pairs
    pub fn set_exchange_rate(&mut self, token_pair: String, rate: f64) {
        self.assert_owner();
        self.exchange_rates.insert(&token_pair, &rate);
        env::log_str(&format!("Exchange rate set for {}: {}", token_pair, rate));
    }

    // Users deposit tokens to the contract for later exchanges
    #[payable]
    pub fn deposit_tokens(&mut self, token_id: String) {
        let deposit_amount = env::attached_deposit();
        assert!(
            deposit_amount >= MIN_DEPOSIT_AMOUNT,
            "Deposit amount is too low"
        );

        let account_deposits = self.user_deposits.get(&env::predecessor_account_id())
            .unwrap_or_else(|| LookupMap::new(env::sha256(env::predecessor_account_id().as_bytes())));
        
        let current_balance = account_deposits.get(&token_id).unwrap_or(0);
        account_deposits.insert(&token_id, &(current_balance + deposit_amount));
        self.user_deposits.insert(&env::predecessor_account_id(), &account_deposits);

        env::log_str(&format!("Deposited {} of {} by {}", deposit_amount, token_id, env::predecessor_account_id()));
    }

    // Users can exchange their deposited tokens based on set exchange rates
    pub fn exchange_tokens(&mut self, from_token_id: String, to_token_id: String, amount: U128) -> Promise {
        let token_pair = format!("{}:{}", from_token_id, to_token_id);
        let rate = self.exchange_rates.get(&token_pair).expect("Exchange rate not set");

        let account_deposits = self.user_deposits.get(&env::predecessor_account_id()).expect("No deposits found");
        let from_token_balance = account_deposits.get(&from_token_id).expect("Insufficient from_token balance");

        assert!(from_token_balance >= &amount.0, "Insufficient balance for exchange");

        let exchange_amount = (amount.0 as f64 * rate) as u128;
        // Logic to call external token contract to perform the exchange or update balances internally
        // Placeholder for demonstration:
        Promise::new(env::predecessor_account_id()).transfer(exchange_amount) // Simplified example

        // Update user deposit balances accordingly
        // Note: Implement logic to handle token transfers and balance updates

        // Placeholder for success logging
        env::log_str(&format!("Exchanged {} {} for {} {}", amount.0, from_token_id, exchange_amount, to_token_id));

        // Placeholder Promise
        Promise::new(env::current_account_id()).value(0)
    }

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only the contract owner can call this method."
        );
    }

    // Additional methods for withdrawals, liquidity provision, etc.
}