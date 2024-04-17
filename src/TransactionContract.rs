use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Gas, assert_one_yocto};
use near_sdk::serde::{Deserialize, Serialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TransactionContract {
    owner: AccountId,
    transactions: LookupMap<u64, TransactionDetail>,
    authorized_signers: UnorderedSet<AccountId>,
    transaction_count: u64,
    stablecoin_handler: AccountId, // Account ID of the StablecoinHandlerContract
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct TransactionDetail {
    creator: AccountId,
    destination: AccountId,
    amount: Balance,
    stablecoin_contract: AccountId, // NEP-141 stablecoin contract address
    executed: bool,
}

#[near_bindgen]
impl TransactionContract {
    #[init]
    pub fn new(owner: AccountId, stablecoin_handler: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner,
            transactions: LookupMap::new(b"t"),
            authorized_signers: UnorderedSet::new(b"s"),
            transaction_count: 0,
            stablecoin_handler,
        }
    }

    // Initiates a stablecoin transfer transaction
    pub fn create_stablecoin_transaction(&mut self, destination: AccountId, amount: U128, stablecoin_contract: AccountId) -> u64 {
        self.assert_authorized(env::signer_account_id());
        let transaction_id = self.transaction_count;

        let transaction_detail = TransactionDetail {
            creator: env::signer_account_id(),
            destination,
            amount: amount.into(),
            stablecoin_contract,
            executed: false,
        };

        self.transactions.insert(&transaction_id, &transaction_detail);
        self.transaction_count += 1;

        env::log_str(&format!("Stablecoin transaction {} created", transaction_id));
        transaction_id
    }

    // Executes the stablecoin transfer via the StablecoinHandlerContract
    pub fn execute_stablecoin_transaction(&mut self, transaction_id: u64) -> Promise {
        let mut transaction_detail = self.transactions.get(&transaction_id).expect("Transaction not found");
        assert!(!transaction_detail.executed, "Transaction already executed");

        transaction_detail.executed = true;
        self.transactions.insert(&transaction_id, &transaction_detail);

        Promise::new(self.stablecoin_handler.clone()).function_call(
            "transfer_stablecoin".to_string().into_bytes(),
            json!({
                "stablecoin_contract": transaction_detail.stablecoin_contract,
                "receiver_id": transaction_detail.destination,
                "amount": U128(transaction_detail.amount)
            }).to_string().into_bytes(),
            1, // Attached deposit for the cross-contract call
            Gas(50_000_000_000_000) // Allocate gas for the cross-contract call
        )
    }

    fn assert_authorized(&self, account_id: AccountId) {
        assert!(
            self.authorized_signers.contains(&account_id) || account_id == self.owner,
            "Unauthorized"
        );
    }

    // Additional helper methods like adding/removing authorized signers...
}
