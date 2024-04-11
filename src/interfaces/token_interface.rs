// src/interfaces/token_interface.rs
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk_sim::near_bindgen;

/// NEP-141: Fungible Token standard
/// Reference: https://nomicon.io/Standards/Tokens/FungibleToken/Core
#[near_bindgen]
pub trait FungibleToken {
    // Returns the total supply of the token.
    fn ft_total_supply(&self) -> U128;

    // Returns the balance of an account.
    fn ft_balance_of(&self, account_id: AccountId) -> U128;

    // Transfers an amount of tokens from the caller to the receiver.
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

    // Optional: Transfers tokens from one account to another.
    // Requires previous approval.
    fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: U128, memo: Option<String>);

    // Optional: Allows `escrow_account_id` to transfer up to `amount` tokens on behalf of the method caller.
    fn ft_approve(&mut self, escrow_account_id: AccountId, amount: U128, memo: Option<String>);

    // Optional: Returns the amount which `escrow_account_id` is allowed to spend on behalf of `owner_id`.
    fn ft_allowance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> U128;
}
