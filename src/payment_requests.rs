// src/payment_requests.rs

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};
use near_sdk::collections::{ UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentRequest {
    request_id: u64,
    from: AccountId,
    to: AccountId,
    amount: Balance,
    message: String,
    created_at: Timestamp,
    status: String, // "pending", "approved", "rejected"
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PaymentRequests {
    requests: UnorderedMap<u64, PaymentRequest>,
    next_request_id: u64,
}

impl Default for PaymentRequests {
    fn default() -> Self {
        Self {
            requests: UnorderedMap::new(b"r"),
            next_request_id: 0,
        }
    }
}

#[near_bindgen]
impl PaymentRequests {
    // Create a new payment request
    pub fn create_request(&mut self, to: AccountId, amount: U128, message: String) {
        let request = PaymentRequest {
            request_id: self.next_request_id,
            from: env::signer_account_id(),
            to,
            amount: amount.0,
            message,
            created_at: env::block_timestamp(),
            status: "pending".to_string(),
        };
        self.requests.insert(&self.next_request_id, &request);
        self.next_request_id += 1;
        env::log_str(&format!("Payment request {} created", request.request_id));
    }

    // Approve a payment request
    pub fn approve_request(&mut self, request_id: u64) {
        let mut request = self.requests.get(&request_id).expect("Request not found");
        assert_eq!(request.to, env::signer_account_id(), "Only the recipient can approve the request");
        request.status = "approved".to_string();
        self.requests.insert(&request_id, &request);
        env::log_str(&format!("Payment request {} approved", request_id));
    }

    // Reject a payment request
    pub fn reject_request(&mut self, request_id: u64) {
        let mut request = self.requests.get(&request_id).expect("Request not found");
        assert_eq!(request.to, env::signer_account_id(), "Only the recipient can reject the request");
        request.status = "rejected".to_string();
        self.requests.insert(&request_id, &request);
        env::log_str(&format!("Payment request {} rejected", request_id));
    }

    // View a specific payment request
    pub fn get_request(&self, request_id: u64) -> PaymentRequest {
        self.requests.get(&request_id).expect("Request not found")
    }

    // List all payment requests (for simplicity; in practice, might need filtering/pagination)
    pub fn list_requests(&self) -> Vec<PaymentRequest> {
        self.requests.values().collect()
    }
}