use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MultiSigWallet {
    owner: AccountId,
    signers: UnorderedSet<AccountId>,
    required_signatures: u64,
    proposals: LookupMap<u64, Proposal>,
    proposal_nonce: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposal {
    action: String,
    signatures: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl MultiSigWallet {
    #[init]
    pub fn new(owner: AccountId, signers: Vec<AccountId>, required_signatures: u64) -> Self {
        let mut signers_set = UnorderedSet::new(b"s");
        for signer in signers.into_iter() {
            signers_set.insert(&signer);
        }
        Self {
            owner,
            signers: signers_set,
            required_signatures,
            proposals: LookupMap::new(b"p"),
            proposal_nonce: 0,
        }
    }

    pub fn add_proposal(&mut self, action: String) -> u64 {
        self.assert_owner();
        let proposal_id = self.proposal_nonce;
        let proposal = Proposal {
            action,
            signatures: UnorderedSet::new(format!("sig{}", proposal_id).as_bytes()),
        };
        self.proposals.insert(&proposal_id, &proposal);
        self.proposal_nonce += 1;
        proposal_id
    }

    pub fn sign_proposal(&mut self, proposal_id: u64) {
        assert!(self.signers.contains(&env::predecessor_account_id()), "Unauthorized signer");
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        proposal.signatures.insert(&env::predecessor_account_id());
        self.proposals.insert(&proposal_id, &proposal);
        if proposal.signatures.len() as u64 >= self.required_signatures {
            self.execute_proposal(proposal_id);
        }
    }

    fn execute_proposal(&self, proposal_id: u64) {
        let proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        // Placeholder for executing the proposal's action. In a real application, this might involve calling another contract's method or transferring funds.
        env::log(format!("Executing proposal {}: {}", proposal_id, proposal.action).as_bytes());
    }

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can call this method");
    }
    
    // Add additional methods as needed
}