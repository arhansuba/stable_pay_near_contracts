use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, log};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TransactionContract {
    owner: AccountId,
    transactions: LookupMap<u64, Transaction>,
    authorized_signers: UnorderedSet<AccountId>,
    transaction_count: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Transaction {
    creator: AccountId,
    destination: AccountId,
    amount: Balance,
    approvals: UnorderedSet<AccountId>,
    executed: bool,
}

#[near_bindgen]
impl TransactionContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner,
            transactions: LookupMap::new(b"t"),
            authorized_signers: UnorderedSet::new(b"s"),
            transaction_count: 0,
        }
    }

    // Create a new transaction
    pub fn create_transaction(&mut self, destination: AccountId, amount: U128) -> u64 {
        self.assert_authorized(env::signer_account_id());
        let transaction_id = self.transaction_count;
        let new_transaction = Transaction {
            creator: env::signer_account_id(),
            destination,
            amount: amount.into(),
            approvals: UnorderedSet::new(format!("a{}", transaction_id).as_bytes()),
            executed: false,
        };

        self.transactions.insert(&transaction_id, &new_transaction);
        self.transaction_count += 1;

        log!("Transaction {} created", transaction_id);
        transaction_id
    }

    // Sign (approve) a transaction
    pub fn sign_transaction(&mut self, transaction_id: u64) {
        self.assert_authorized(env::signer_account_id());
        let mut transaction = self.transactions.get(&transaction_id).expect("Transaction not found");
        assert!(!transaction.executed, "Transaction already executed");
        transaction.approvals.insert(&env::signer_account_id());

        self.transactions.insert(&transaction_id, &transaction);
        log!("Transaction {} signed by {}", transaction_id, env::signer_account_id());
    }

    // Execute a signed transaction
    pub fn execute_transaction(&mut self, transaction_id: u64) {
        let transaction = self.transactions.get(&transaction_id).expect("Transaction not found");
        assert!(transaction.approvals.len() >= 2, "Not enough approvals to execute the transaction");
        assert!(!transaction.executed, "Transaction already executed");

        Promise::new(transaction.destination.clone()).transfer(transaction.amount);
        self.mark_transaction_executed(transaction_id);

        log!("Transaction {} executed", transaction_id);
    }

    #[private]
    fn mark_transaction_executed(&mut self, transaction_id: u64) {
        let mut transaction = self.transactions.get(&transaction_id).unwrap();
        transaction.executed = true;
        self.transactions.insert(&transaction_id, &transaction);
    }

    fn assert_authorized(&self, account_id: AccountId) {
        assert!(
            self.authorized_signers.contains(&account_id) || account_id == self.owner,
            "Unauthorized"
        );
    }

    // Additional helper methods like adding/removing authorized signers...
}