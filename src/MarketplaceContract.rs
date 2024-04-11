use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Balance, Promise, Gas};

near_sdk::setup_alloc!();

const SALE_FEE_TGAS: Gas = 5_000_000_000_000;
const NEAR: Balance = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MarketplaceContract {
    owner: AccountId,
    listings: LookupMap<String, Listing>,
    active_trades: UnorderedSet<String>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Listing {
    asset_id: String,
    seller: AccountId,
    price: Balance,
}

#[near_bindgen]
impl MarketplaceContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");
        Self {
            owner: env::signer_account_id(),
            listings: LookupMap::new(b"l"),
            active_trades: UnorderedSet::new(b"t"),
        }
    }

    pub fn create_listing(&mut self, asset_id: String, price: U128) {
        let listing = Listing {
            asset_id: asset_id.clone(),
            seller: env::signer_account_id(),
            price: price.into(),
        };
        self.listings.insert(&asset_id, &listing);
        log!("Listing created: {}", asset_id);
    }

    pub fn purchase_asset(&mut self, asset_id: String) -> Promise {
        let listing = self.listings.get(&asset_id).expect("Listing not found");
        assert!(env::attached_deposit() >= listing.price, "Attached deposit is less than the price");

        self.active_trades.insert(&asset_id);
        Promise::new(listing.seller.clone()).transfer(listing.price - (listing.price / NEAR * SALE_FEE_TGAS as u128))
            .then(Promise::new(env::current_account_id()).function_call(
                b"finalize_purchase".to_vec(),
                env::json_format(&listing).as_bytes().to_vec(),
                0,
                SALE_FEE_TGAS,
            ))
    }

    #[private]
    pub fn finalize_purchase(&mut self, listing: Listing) {
        assert!(self.active_trades.contains(&listing.asset_id), "Trade not active");
        self.active_trades.remove(&listing.asset_id);
        self.listings.remove(&listing.asset_id);
        log!("Purchase finalized: {}", listing.asset_id);
    }
    
    // Methods for updating listings, removing listings, etc.
}