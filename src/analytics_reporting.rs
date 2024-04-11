use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct PaymentAnalytics {
    payments: Vector<Payment>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Payment {
    from: AccountId,
    to: AccountId,
    amount: u128,
    timestamp: u64,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Payments,
}

#[near_bindgen]
impl PaymentAnalytics {
    #[init]
    pub fn new() -> Self {
        Self {
            payments: Vector::new(StorageKey::Payments),
        }
    }

    // Function to record a payment
    pub fn record_payment(&mut self, from: AccountId, to: AccountId, amount: u128) {
        let payment = Payment {
            from,
            to,
            amount,
            timestamp: env::block_timestamp(),
        };
        self.payments.push(&payment);
    }

    // Function to get total payments made
    pub fn get_total_payments(&self) -> u64 {
        self.payments.len()
    }

    // Function to get total volume of payments
    pub fn get_total_volume(&self) -> u128 {
        self.payments.iter().map(|payment| payment.amount).sum()
    }

    // Function to get payments within a time range
    pub fn get_payments_in_range(&self, start_timestamp: u64, end_timestamp: u64) -> Vec<Payment> {
        self.payments.iter().filter(|payment| payment.timestamp >= start_timestamp && payment.timestamp <= end_timestamp).collect()
    }

    // Add more analytics functions as needed
}