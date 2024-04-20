// src/utils.rs
use near_sdk::AccountId;
use regex::Regex;

pub fn is_valid_account_id(account_id: &AccountId) -> bool {
    let account_id_pattern = Regex::new(r"^(([a-z\d]+[-_])*[a-z\d]+\.)*([a-z\d]+[-_])*[a-z\d]+$").unwrap();
    account_id.len() >= 2 &&
    account_id.len() <= 64 &&
    account_id_pattern.is_match(account_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_account_id() {
        assert!(is_valid_account_id(&"example.testnet".parse().unwrap()));
    }

    #[test]
    fn test_invalid_account_id() {
        assert!(!is_valid_account_id(&"InvalidAccountID".parse().unwrap()));
    }
}