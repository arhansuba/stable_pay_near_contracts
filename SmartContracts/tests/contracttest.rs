use near_sdk::{testing_env, VMContext, AccountId};
use near_sdk::json_types::U128;
use crate::{CorePayment, PaymentRequests, DisputeResolutions, Rewards};








#[cfg(test)]
mod tests {
    use super::*;

    fn get_context(is_view: bool) -> VMContext {
        VMContext {
            current_account_id: AccountId::new_unchecked("test".to_string()),
            signer_account_id: AccountId::new_unchecked("test".to_string()),
            signer_account_pk: vec![0u8; 32].try_into().unwrap(), // assuming a 32-byte array is needed
            predecessor_account_id: AccountId::new_unchecked("predecessor.testnet".to_string()),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            epoch_height: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18).into(), // Sample gas amount
            random_seed: [0; 32], // 32 bytes array
            output_data_receivers: vec![],
            view_config: Default::default(),
        }
    }

    #[test]
    fn test_register_and_send_payment() {
        let context = get_context(false);
        testing_env!(context);

        // CorePayment kullanıldığını varsayalım.
        let mut contract = CorePayment::default();
        contract.register_account();
        assert_eq!(contract.get_balance(), 0);

        let context = get_context(true);
        testing_env!(context);
        contract.register_account();
        contract.send_payment(200);
        assert_eq!(contract.get_balance(), 200);
    }

    #[test]
    fn test_create_and_approve_request() {
        let context = get_context(false);
        testing_env!(context);

        // PaymentRequests kullanıldığını varsayalım.
        let mut contract = PaymentRequests::new();
        contract.create_request("test".to_string(), U128(200), "Test payment".to_string());

        let requests = contract.list_requests();
        assert_eq!(requests.len(), 1);

        contract.approve_request(0);
        let request = contract.get_request(0);
        assert_eq!(request.status, "approved".to_string());
    }

    #[test]
    fn test_create_and_resolve_dispute() {
        let context = get_context(false);
        testing_env!(context);

        // DisputeResolutions kullanıldığını varsayalım.
        let mut contract = DisputeResolutions::new();
        contract.create_dispute("test".to_string(), "Service not provided".to_string(), U128(100));

        let disputes = contract.list_disputes();
        assert_eq!(disputes.len(), 1);

        contract.resolve_dispute(0, "Resolution".to_string());
        let dispute = contract.get_dispute(0);
        assert_eq!(dispute.status, "resolved".to_string());
    }

    #[test]
    fn test_add_and_withdraw_rewards() {
        let context = get_context(false);
        testing_env!(context);

        // Rewards kullanıldığını varsayalım.
        let mut contract = Rewards::default();
        contract.add_rewards("test".to_string(), 500);
        assert_eq!(contract.get_reward_balance(), 500);

        let balance_before_withdraw = contract.withdraw_rewards("test".to_string());
        assert_eq!(balance_before_withdraw, 500);
        assert_eq!(contract.get_reward_balance(), 0);
    }
}
