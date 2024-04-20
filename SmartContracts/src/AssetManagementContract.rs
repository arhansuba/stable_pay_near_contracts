use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, ext_contract,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AssetManagementContract {
    owner: AccountId,
    assets: UnorderedMap<String, Asset>, // AssetID to Asset
    asset_prices: LookupMap<String, U128>, // AssetID to Price in stablecoin
    stablecoin_contract: AccountId, // NEP-141 stablecoin contract
    user_balances: LookupMap<AccountId, Balance>, // User balances in stablecoin
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Asset {
    title: String,
    description: String,
    owner: AccountId,
}

#[ext_contract(ext_stablecoin)]
pub trait StablecoinContract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: String);
}

#[near_bindgen]
impl AssetManagementContract {
    #[init]
    pub fn new(owner: AccountId, stablecoin_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            assets: UnorderedMap::new(b"a"),
            asset_prices: LookupMap::new(b"p"),
            stablecoin_contract,
            user_balances: LookupMap::new(b"u"),
        }
    }

    // List a new asset for sale with price in stablecoin
    pub fn list_asset_for_sale(&mut self, asset_id: String, asset: Asset, price: U128) {
        assert_eq!(self.owner, env::predecessor_account_id(), "Only the owner can list assets");
        self.assets.insert(&asset_id, &asset);
        self.asset_prices.insert(&asset_id, &price);
        env::log_str(&format!("Asset {} listed for sale at price {}", asset_id, price.0));
    }

    // Purchase an asset with stablecoin
    pub fn purchase_asset(&mut self, asset_id: String) -> Promise {
        let price = self.asset_prices.get(&asset_id).expect("Asset not found or not for sale");
        let asset = self.assets.get(&asset_id).expect("Asset not found");

        Promise::new(self.stablecoin_contract.clone()).function_call(
            "ft_transfer".to_string().into_bytes(),
            json!({
                "receiver_id": asset.owner.clone(),
                "amount": price.clone(),
                "memo": format!("Purchasing asset: {}", asset_id)
            }).to_string().into_bytes(),
            1, // Attached deposit
            env::prepaid_gas() / 2 // Allocate gas
        )
    }

    // Callback function after successful transfer of stablecoin
    #[private]
    pub fn on_stablecoin_transfer(&mut self, sender_id: AccountId, amount: U128) {
        let sender_balance = self.user_balances.get(&sender_id).unwrap_or(0);
        self.user_balances.insert(&sender_id, &(sender_balance - amount.0));
        
        let owner_balance = self.user_balances.get(&env::predecessor_account_id()).unwrap_or(0);
        self.user_balances.insert(&env::predecessor_account_id(), &(owner_balance + amount.0));
    }

    // Get user balance
    pub fn get_user_balance(&self, user_id: AccountId) -> U128 {
        self.user_balances.get(&user_id).unwrap_or(0).into()
    }

    // Additional functionality for asset management...
}

