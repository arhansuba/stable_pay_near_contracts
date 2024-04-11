use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey};

near_sdk::setup_alloc!();

// Define a structure for asset metadata
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AssetMetadata {
    title: String,
    description: String,
    // Extend with more metadata fields as necessary (e.g., image URL, creation date)
}

// Define a structure for an asset, including its metadata and owner
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Asset {
    metadata: AssetMetadata,
    owner: AccountId,
    // Track the transfer history as a vector of account IDs
    transfer_history: Vector<AccountId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AssetManagementContract {
    // Owner of the contract (e.g., the dApp admin)
    owner: AccountId,
    // A mapping from asset IDs to their respective asset structures
    assets: UnorderedMap<String, Asset>,
    // Optionally, maintain a reverse mapping from account IDs to assets they own
    ownership: LookupMap<AccountId, Vector<String>>,
}

#[near_bindgen]
impl AssetManagementContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { 
            owner: env::signer_account_id(),
            assets: UnorderedMap::new(b"a"),
            ownership: LookupMap::new(b"o"),
        }
    }

    // Create a new asset with given metadata
    pub fn create_asset(&mut self, asset_id: String, metadata: AssetMetadata) {
        let caller_id = env::signer_account_id();
        assert_eq!(caller_id, self.owner, "Only the owner can create assets");

        let new_asset = Asset {
            metadata,
            owner: caller_id.clone(),
            transfer_history: Vector::new(StorageKeys::TransferHistory { asset_id: asset_id.clone() }),
        };
        new_asset.transfer_history.push(&caller_id); // Initial owner
        
        self.assets.insert(&asset_id, &new_asset);
        self.add_to_ownership(&caller_id, asset_id);
    }

    // Transfer an asset to another user
    pub fn transfer_asset(&mut self, asset_id: String, new_owner: AccountId) {
        let mut asset = self.assets.get(&asset_id).expect("Asset not found");
        let caller_id = env::signer_account_id();

        assert_eq!(asset.owner, caller_id, "Caller is not the asset owner");
        
        // Update asset's owner and transfer history
        asset.owner = new_owner.clone();
        asset.transfer_history.push(&new_owner);
        
        self.assets.insert(&asset_id, &asset);
        self.add_to_ownership(&new_owner, asset_id);
        
        // Remove from previous owner's list (not shown)
        // Consider edge cases like transfer restrictions
    }

    // Utility function to add asset ID to a user's ownership list
    fn add_to_ownership(&mut self, owner: &AccountId, asset_id: String) {
        let mut owned = self.ownership.get(owner).unwrap_or_default();
        owned.push(&asset_id);
        self.ownership.insert(owner, &owned);
    }
    
    // Additional functions to query assets, update metadata, enforce transfer restrictions, etc.
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    TransferHistory { asset_id: String },
    // Define additional storage keys as necessary
}