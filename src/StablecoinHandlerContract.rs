use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap},
    env, ext_contract,
    json_types::U128,
    near_bindgen, AccountId, Balance, Promise, PromiseResult, PanicOnDefault,
};

near_sdk::setup_alloc!();

// Define the interface for NEP-141 fungible token.
#[ext_contract]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StablecoinHandlerContract {
    owner: AccountId,
    // A mapping from stablecoin contract addresses to the registered stablecoin metadata
    stablecoins: LookupMap<AccountId, StablecoinMetadata>,
    // A mapping from user accounts to their balances for each stablecoin
    balances: UnorderedMap<(AccountId, AccountId), Balance>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StablecoinMetadata {
    name: String,
    symbol: String,
    decimals: u8,
}

#[near_bindgen]
impl StablecoinHandlerContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: env::signer_account_id(),
            stablecoins: LookupMap::new(b"s"),
            balances: UnorderedMap::new(b"b"),
        }
    }

    // Registers a new stablecoin in the contract
    pub fn register_stablecoin(&mut self, contract_id: AccountId, metadata: StablecoinMetadata) {
        self.assert_owner();
        self.stablecoins.insert(&contract_id, &metadata);
        env::log_str(&format!("Registered stablecoin: {} ({})", metadata.name, contract_id));
    }

    // Transfers specified amount of stablecoin to a receiver
    pub fn transfer_stablecoin(&mut self, stablecoin_contract: AccountId, receiver_id: AccountId, amount: U128) -> Promise {
        self.assert_stablecoin_registered(&stablecoin_contract);
        let sender_id = env::predecessor_account_id();
        let transfer_amount: Balance = amount.into();
        self.transfer_internal(&stablecoin_contract, &sender_id, &receiver_id, transfer_amount)
    }

    // Internal function to handle the transfer of stablecoins
    fn transfer_internal(&mut self, stablecoin_contract: &AccountId, sender_id: &AccountId, receiver_id: &AccountId, amount: Balance) -> Promise {
        let mut promise = Promise::new(stablecoin_contract.clone()).function_call(
            "ft_transfer".to_string().into_bytes(),
            json!({
                "receiver_id": receiver_id,
                "amount": amount,
                "memo": "Stablecoin transfer via handler"
            }).to_string().into_bytes(),
            1, // Attached deposit. This may vary based on the stablecoin contract requirements.
            env::prepaid_gas() / 4, // Allocate a fraction of the prepaid gas for this call
        );

        self.balances.insert(&(sender_id.clone(), stablecoin_contract.clone()), 0);
        self.balances.insert(&(receiver_id.clone(), stablecoin_contract.clone()), 0);
        
        promise = promise.then(ext_self::on_transfer_complete(
            sender_id.clone(),
            receiver_id.clone(),
            amount,
            env::current_account_id(),
            0,
            env::prepaid_gas() / 4,
        ));

        promise
    }

    // Callback function after the transfer is complete
    #[private]
    pub fn on_transfer_complete(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: Balance) -> Promise {
        let sender_balance = self.balances.get(&(sender_id.clone(), env::current_account_id())).unwrap_or(0);
        let receiver_balance = self.balances.get(&(receiver_id.clone(), env::current_account_id())).unwrap_or(0);

        self.balances.insert(&(sender_id.clone(), env::current_account_id()), sender_balance - amount);
        self.balances.insert(&(receiver_id.clone(), env::current_account_id()), receiver_balance + amount);
        
        Promise::new(sender_id).resolve(())
    }

    // Queries the balance of a given account for a specified stablecoin
    pub fn query_balance(&self, stablecoin_contract: AccountId, account_id: AccountId) -> Promise {
        self.assert_stablecoin_registered(&stablecoin_contract);
        env::log_str(&format!("Querying balance of {} on {}", account_id, stablecoin_contract));

        // Assuming the stablecoin contract adheres to NEP-141
        Promise::new(stablecoin_contract).function_call(
            "ft_balance_of".to_string().into_bytes(),
            json!({ "account_id": account_id }).to_string().into_bytes(),
            0, // No attached deposit needed for balance query.
            env::prepaid_gas() / 4, // Allocate a fraction of the prepaid gas for this call
        )
    }

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the contract owner can call this method.");
    }

    fn assert_stablecoin_registered(&self, contract_id: &AccountId) {
        assert!(self.stablecoins.contains_key(contract_id), "Stablecoin not registered");
    }
}
