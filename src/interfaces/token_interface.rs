use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PromiseOrValue, Balance, Promise};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, UnorderedMap};
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    owner_id: AccountId,
    total_supply: Balance,
    balances: LookupMap<AccountId, Balance>,
    allowances: LookupMap<(AccountId, AccountId), Balance>,
    metadata: UnorderedMap<String, String>,
}

#[near_bindgen]
impl FungibleToken {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        let mut balances = LookupMap::new(b"b".to_vec());
        balances.insert(&owner_id, &total_supply.0);

        Self {
            owner_id,
            total_supply: total_supply.0,
            balances,
            allowances: LookupMap::new(b"a".to_vec()),
            metadata: UnorderedMap::new(b"m".to_vec()),
        }
    }

    pub fn ft_total_supply(&self) -> U128 {
        U128(self.total_supply)
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        U128(*self.balances.get(&account_id).unwrap_or(&0))
    }

    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        let amount: u128 = amount.into();
        let sender_balance = self.balances.get(&sender_id).unwrap_or(0);
        assert!(sender_balance >= amount, "Insufficient balance");
        self.balances.insert(&sender_id, &(sender_balance - amount));
        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(0);
        self.balances.insert(&receiver_id, &(receiver_balance + amount));
    }

    pub fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let amount: u128 = amount.into();
        let allowance = self.allowances.get(&(sender_id.clone(), env::predecessor_account_id())).unwrap_or(0);
        assert!(allowance >= amount, "Allowance exceeded");
        self.allowances.insert(&(sender_id.clone(), env::predecessor_account_id()), &(allowance - amount));
        self.ft_transfer(receiver_id, U128(amount), memo);
    }

    pub fn ft_approve(&mut self, escrow_account_id: AccountId, amount: U128, memo: Option<String>) {
        self.allowances.insert(&(env::predecessor_account_id(), escrow_account_id), &amount.0);
    }

    pub fn ft_allowance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> U128 {
        U128(self.allowances.get(&(owner_id, escrow_account_id)).unwrap_or(0))
    }

    pub fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>, msg: String) -> PromiseOrValue<U128> {
        let promise = Promise::new(receiver_id.clone()).function_call(msg.into_bytes(), amount.into(), 100000000000000);
        PromiseOrValue::Promise(promise)
    }
    
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(&key, &value);
    }
    
    pub fn get_metadata(&self, key: String) -> Option<String> {
        self.metadata.get(&key)
    }
}
