// src/dispute_resolutions.rs

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, env};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum DisputeStatus {
    Open,
    Closed,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Dispute {
    pub dispute_id: u64,
    pub initiator: AccountId,
    pub respondent: AccountId,
    pub reason: String,
    pub status: DisputeStatus,
    pub amount: U128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DisputeResolutions {
    disputes: UnorderedMap<u64, Dispute>,
    dispute_ids: Vector<u64>, // Helps in listing disputes
    next_dispute_id: u64,
}

#[near_bindgen]
impl DisputeResolutions {
    // Initialize the contract
    #[init]
    pub fn new() -> Self {
        Self {
            disputes: UnorderedMap::new(b"d"),
            dispute_ids: Vector::new(b"i"),
            next_dispute_id: 0,
        }
    }

    // Create a new dispute
    pub fn create_dispute(&mut self, respondent: AccountId, reason: String, amount: U128) {
        let dispute = Dispute {
            dispute_id: self.next_dispute_id,
            initiator: env::signer_account_id(),
            respondent,
            reason,
            status: DisputeStatus::Open,
            amount,
        };
        self.disputes.insert(&self.next_dispute_id, &dispute);
        self.dispute_ids.push(&self.next_dispute_id);
        self.next_dispute_id += 1;
        env::log_str(&format!("Dispute {} created", dispute.dispute_id));
    }

    // Resolve a dispute
    pub fn resolve_dispute(&mut self, dispute_id: u64) {
        let mut dispute = self.disputes.get(&dispute_id).expect("Dispute not found");
        assert_eq!(dispute.initiator, env::signer_account_id(), "Only the initiator can resolve the dispute");
        dispute.status = DisputeStatus::Closed;
        self.disputes.insert(&dispute_id, &dispute);
        env::log_str(&format!("Dispute {} resolved", dispute_id));
    }

    // View a dispute
    pub fn get_dispute(&self, dispute_id: u64) -> Dispute {
        self.disputes.get(&dispute_id).expect("Dispute not found")
    }

    // List all disputes
    pub fn list_disputes(&self) -> Vec<Dispute> {
        self.dispute_ids.iter().map(|id| self.disputes.get(&id).unwrap()).collect()
    }
}

