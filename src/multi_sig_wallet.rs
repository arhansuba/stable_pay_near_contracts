use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Promise, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MultiSigWallet {
    owners: Vec<AccountId>,
    confirmations_required: u64,
    transactions: UnorderedMap<u64, Transaction>,
    pending_transactions: Vec<u64>,
    transaction_count: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Transaction {
    destination: AccountId,
    amount: u128,
    executed: bool,
    confirmations: Vec<AccountId>,
}

#[near_bindgen]
impl MultiSigWallet {
    #[init]
    pub fn new(owners: Vec<AccountId>, confirmations_required: u64) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(confirmations_required > 0 && confirmations_required <= owners.len() as u64, "Invalid number of required confirmations");

        Self {
            owners,
            confirmations_required,
            transactions: UnorderedMap::new(b"t".to_vec()),
            pending_transactions: vec![],
            transaction_count: 0,
        }
    }

    pub fn submit_transaction(&mut self, destination: AccountId, amount: u128) -> u64 {
        assert!(self.is_owner(&env::signer_account_id()), "Only owners can call this method");

        let transaction = Transaction {
            destination,
            amount,
            executed: false,
            confirmations: vec![],
        };

        self.transactions.insert(&self.transaction_count, &transaction);
        self.pending_transactions.push(self.transaction_count);
        self.transaction_count += 1;
        self.transaction_count - 1
    }

    pub fn confirm_transaction(&mut self, transaction_id: u64) {
        assert!(self.is_owner(&env::signer_account_id()), "Only owners can call this method");
        
        if let Some(mut transaction) = self.transactions.get(&transaction_id) {
            assert!(!transaction.executed, "Transaction already executed");
            assert!(!transaction.confirmations.contains(&env::signer_account_id()), "Transaction already confirmed by this owner");

            transaction.confirmations.push(env::signer_account_id());
            if transaction.confirmations.len() as u64 >= self.confirmations_required {
                self.execute_transaction(transaction_id);
            } else {
                self.transactions.insert(&transaction_id, &transaction);
            }
        } else {
            env::panic_str("Transaction does not exist");
        }
    }

    fn execute_transaction(&mut self, transaction_id: u64) {
        if let Some(mut transaction) = self.transactions.get(&transaction_id) {
            assert!(!transaction.executed, "Transaction already executed");
            assert!(transaction.confirmations.len() as u64 >= self.confirmations_required, "Not enough confirmations");

            Promise::new(transaction.destination.clone()).transfer(transaction.amount);
            transaction.executed = true;
            self.transactions.insert(&transaction_id, &transaction);
        }
    }

    // Additional methods and utility functions here...

    fn is_owner(&self, account_id: &AccountId) -> bool {
        self.owners.contains(account_id)
    }
}