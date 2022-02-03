use hex::FromHex;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env::{self, keccak256},
    json_types::{U128, U64},
    near_bindgen, require, AccountId, Balance, EpochHeight, PanicOnDefault,
};

mod constant;
mod internal;
mod merkle_proof;
mod token;
mod util;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Account {
    pub claimed_amount: Balance,
    pub claimed_epoch_height: EpochHeight,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            claimed_amount: 0,
            claimed_epoch_height: 0,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MerkleDistributor {
    pub owner_id: AccountId,
    pub token_id: AccountId,
    pub balance: Balance,
    pub merkle_root: Vec<u8>,
    pub accounts: UnorderedMap<AccountId, Account>,
    pub paused: bool,
}

#[near_bindgen]
impl MerkleDistributor {
    #[init]
    pub fn initialize(owner_id: AccountId, token_id: AccountId, merkle_root: String) -> Self {
        require!(!env::state_exists(), "Already initialized");
        require!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        Self {
            owner_id,
            token_id,
            balance: 0,
            merkle_root: <[u8; 32]>::from_hex(merkle_root).ok().unwrap().to_vec(),
            accounts: UnorderedMap::new(b"c".to_vec()),
            paused: false,
        }
    }

    pub fn pause(&mut self) -> () {
        self.assert_owner();
        require!(!self.paused, "The contract is already paused");

        self.paused = true;
    }

    pub fn resume(&mut self) -> () {
        self.assert_owner();
        require!(self.paused, "The contract is not paused");

        self.paused = false;
    }

    pub fn get_balance(&self) -> Balance {
        self.balance
    }

    pub fn get_claimed_amount(&self, account_id: AccountId) -> Balance {
        let account = self
            .accounts
            .get(&account_id)
            .unwrap_or_default();
        account.claimed_amount
    }

    pub fn get_is_claimed(&self, account_id: AccountId) -> bool {
        self.get_claimed_amount(account_id) > 0
    }

    fn set_claim(&mut self, amount: U128) -> () {
        self.accounts.insert(
            &env::predecessor_account_id(),
            &Account {
                claimed_amount: amount.into(),
                claimed_epoch_height: env::epoch_height(),
            },
        );
        self.balance -= u128::from(amount);
    }

    #[payable]
    pub fn claim(&mut self, index: U64, amount: U128, proof: Vec<String>) -> () {
        self.assert_paused();
        require!(!self.get_is_claimed(env::predecessor_account_id()), "Already claimed");
        require!(self.balance >= amount.into(), "Non-sufficient fund");

        let mut _index = u64::from(index).to_le_bytes().to_vec();
        let mut _account = env::predecessor_account_id().as_bytes().to_vec();
        let mut _amount = u128::from(amount).to_le_bytes().to_vec();

        _index.append(&mut _account);
        _index.append(&mut _amount);

        let node = keccak256(&_index);

        let _proof: Vec<[u8; 32]> = proof
            .into_iter()
            .map(|x| <[u8; 32]>::from_hex(x).ok().unwrap())
            .collect();
        let _root = merkle_proof::vec_to_array::<u8, 32>(self.merkle_root.clone());
        let _leaf = merkle_proof::vec_to_array::<u8, 32>(node);

        require!(
            merkle_proof::verify(_proof, _root, _leaf),
            "Failed to verify proof"
        );

        self.set_claim(amount);
        self.withdraw_token(amount.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::AccountId;
    use near_sdk::{testing_env, VMContext};

    struct Accounts {
        current: AccountId,
        owner: AccountId,
        predecessor: AccountId,
        token: AccountId,
    }

    struct Ctx {
        accounts: Accounts,
        vm: VMContext,
    }

    impl Ctx {
        fn create_accounts() -> Accounts {
            return Accounts {
                current: "alice.testnet".parse().unwrap(),
                owner: "robert.testnet".parse().unwrap(),
                predecessor: "jane.testnet".parse().unwrap(),
                token: "fungible_token.test".parse().unwrap(),
            };
        }

        pub fn new(input: Vec<u8>) -> Self {
            let accounts = Ctx::create_accounts();
            let vm = VMContext {
                current_account_id: accounts.current.to_string().parse().unwrap(),
                signer_account_id: accounts.owner.to_string().parse().unwrap(),
                signer_account_pk: vec![0, 1, 2],
                predecessor_account_id: accounts.predecessor.to_string().parse().unwrap(),
                input,
                block_index: 0,
                block_timestamp: 0,
                account_balance: 0,
                account_locked_balance: 0,
                storage_usage: 0,
                attached_deposit: 0,
                prepaid_gas: 10u64.pow(18),
                random_seed: vec![0, 1, 2],
                view_config: std::option::Option::None,
                output_data_receivers: vec![],
                epoch_height: 19,
            };
            return Self {
                accounts: accounts,
                vm: vm,
            };
        }
    }

    #[test]
    fn init_successful() {
        let context = Ctx::new(vec![]);
        testing_env!(context.vm);
        let contract = MerkleDistributor::initialize(
            env::signer_account_id(),
            context.accounts.token,
            "7b8bad907ecad0eab5c376a9926bbb9c38edd7303e1e22e46594eaaa333a5d12".to_string(),
        );
        assert_eq!(false, contract.get_is_claimed(context.accounts.predecessor));
    }

    #[test]
    fn claim_successful() {
        let mut context = Ctx::new(vec![]);
        context.vm.attached_deposit = 1100;
        testing_env!(context.vm);
        let mut contract = MerkleDistributor::initialize(
            env::signer_account_id(),
            context.accounts.token,
            "a53a837856e9004a7737f9cc344e2850ef385807298169004d690dabeea699b0".to_string(),
        );
        contract.deposit_token(contract.token_id.clone(), 1100);
        contract.claim(
            U64(0),
            U128(100),
            vec![
                "16aa0bf4f9a579de1bf742d4f854c322aa94b3eaf3254c13ae5820e3830061b5".to_string(),
                "afb188661bca1d37b7c6c4cffd2d62ed7bd19bc0844bcdf5f44ebeacb7b394bd".to_string(),
                "001d2522f71f331abd7bcd7626ef29c44f5769379e54a4a0dd6992bcd1a04793".to_string(),
                "5f1469d2fe519c64059195d61dbca371ac14314dcdd72e83eaab10ba4e5600c2".to_string(),
            ],
        );
        assert_eq!(100, contract.get_claimed_amount(context.accounts.predecessor.clone()));
        assert_eq!(true, contract.get_is_claimed(context.accounts.predecessor));
        assert_eq!(1000, contract.get_balance());
    }

    #[test]
    #[should_panic(expected = "Failed to verify proof")]
    fn claim_failed_because_input_wrong_amount() {
        let mut context = Ctx::new(vec![]);
        context.vm.attached_deposit = 1100;
        testing_env!(context.vm);
        let mut contract = MerkleDistributor::initialize(
            env::signer_account_id(),
            context.accounts.token,
            "a53a837856e9004a7737f9cc344e2850ef385807298169004d690dabeea699b0".to_string(),
        );
        contract.deposit_token(contract.token_id.clone(), 1100);
        contract.claim(
            U64(0),
            U128(1000),
            vec![
                "16aa0bf4f9a579de1bf742d4f854c322aa94b3eaf3254c13ae5820e3830061b5".to_string(),
                "afb188661bca1d37b7c6c4cffd2d62ed7bd19bc0844bcdf5f44ebeacb7b394bd".to_string(),
                "001d2522f71f331abd7bcd7626ef29c44f5769379e54a4a0dd6992bcd1a04793".to_string(),
                "5f1469d2fe519c64059195d61dbca371ac14314dcdd72e83eaab10ba4e5600c2".to_string(),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "Already claimed")]
    fn claim_failed_because_already_claimed() {
        let mut context = Ctx::new(vec![]);
        context.vm.attached_deposit = 1100;
        testing_env!(context.vm);
        let mut contract = MerkleDistributor::initialize(
            env::signer_account_id(),
            context.accounts.token,
            "a53a837856e9004a7737f9cc344e2850ef385807298169004d690dabeea699b0".to_string(),
        );
        contract.deposit_token(contract.token_id.clone(), 1100);
        contract.claim(
            U64(0),
            U128(100),
            vec![
                "16aa0bf4f9a579de1bf742d4f854c322aa94b3eaf3254c13ae5820e3830061b5".to_string(),
                "afb188661bca1d37b7c6c4cffd2d62ed7bd19bc0844bcdf5f44ebeacb7b394bd".to_string(),
                "001d2522f71f331abd7bcd7626ef29c44f5769379e54a4a0dd6992bcd1a04793".to_string(),
                "5f1469d2fe519c64059195d61dbca371ac14314dcdd72e83eaab10ba4e5600c2".to_string(),
            ],
        );

        contract.claim(
            U64(0),
            U128(100),
            vec![
                "16aa0bf4f9a579de1bf742d4f854c322aa94b3eaf3254c13ae5820e3830061b5".to_string(),
                "afb188661bca1d37b7c6c4cffd2d62ed7bd19bc0844bcdf5f44ebeacb7b394bd".to_string(),
                "001d2522f71f331abd7bcd7626ef29c44f5769379e54a4a0dd6992bcd1a04793".to_string(),
                "5f1469d2fe519c64059195d61dbca371ac14314dcdd72e83eaab10ba4e5600c2".to_string(),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "Can only be called when not paused")]
    fn claim_failed_because_contract_is_paused() {
        let mut context = Ctx::new(vec![]);
        context.vm.attached_deposit = 1100;
        testing_env!(context.vm);
        let mut contract = MerkleDistributor::initialize(
            env::predecessor_account_id(),
            context.accounts.token,
            "a53a837856e9004a7737f9cc344e2850ef385807298169004d690dabeea699b0".to_string(),
        );
        contract.deposit_token(contract.token_id.clone(), 1100);
        contract.pause();
        contract.claim(
            U64(0),
            U128(100),
            vec![
                "16aa0bf4f9a579de1bf742d4f854c322aa94b3eaf3254c13ae5820e3830061b5".to_string(),
                "afb188661bca1d37b7c6c4cffd2d62ed7bd19bc0844bcdf5f44ebeacb7b394bd".to_string(),
                "001d2522f71f331abd7bcd7626ef29c44f5769379e54a4a0dd6992bcd1a04793".to_string(),
                "5f1469d2fe519c64059195d61dbca371ac14314dcdd72e83eaab10ba4e5600c2".to_string(),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "Can only be called by the owner")]
    fn claim_failed_because_pause_contract_not_by_owner() {
        let mut context = Ctx::new(vec![]);
        context.vm.attached_deposit = 1100;
        testing_env!(context.vm);
        let mut contract = MerkleDistributor::initialize(
            env::signer_account_id(),
            context.accounts.token,
            "a53a837856e9004a7737f9cc344e2850ef385807298169004d690dabeea699b0".to_string(),
        );
        contract.pause();
    }
}
