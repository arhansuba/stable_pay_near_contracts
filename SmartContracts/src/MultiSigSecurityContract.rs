use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, Vector};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MultiSigSecurityContract {
    owner: AccountId,
    required_approvals: u64,
    transactions: LookupMap<u64, Transaction>,
    pending_transactions: Vector<u64>,
    approvals: LookupMap<u64, UnorderedSet<AccountId>>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Transaction {
    destination: AccountId,
    amount: u128,
    executed: bool,
}

#[near_bindgen]
impl MultiSigSecurityContract {
    #[init]
    pub fn new(owner: AccountId, required_approvals: u64) -> Self {
        assert!(!env::state_exists(), "Contract is already initialized");

        Self {
            owner,
            required_approvals,
            transactions: LookupMap::new(b"t"),
            pending_transactions: Vector::new(b"p"),
            approvals: LookupMap::new(b"a"),
        }
    }

    pub fn propose_transaction(&mut self, transaction_id: u64, destination: AccountId, amount: u128) {
        self.assert_owner();
        let new_transaction = Transaction {
            destination,
            amount,
            executed: false,
        };
        self.transactions.insert(&transaction_id, &new_transaction);
        self.pending_transactions.push(&transaction_id);
