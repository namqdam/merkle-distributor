use near_contract_standards::fungible_token::{
    core_impl::ext_fungible_token, receiver::FungibleTokenReceiver,
};
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, PromiseOrValue};

use crate::{constant::GAS_FOR_FT_TRANSFER, *};

#[near_bindgen]
impl FungibleTokenReceiver for MerkleDistributor {
    // Callback on receiving tokens by this contract.
    // Returns zero.
    #[allow(unused_variables)]
    #[payable]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_id = env::predecessor_account_id();
        let sender_id = AccountId::from(sender_id);

        self.deposit_token(token_id, amount.into());
        return PromiseOrValue::Value(U128(0));
    }
}

#[near_bindgen]
impl MerkleDistributor {
    // Record deposit of some number of tokens to this contract.
    pub(crate) fn deposit_token(&mut self, token_id: AccountId, amount: Balance) {
        require!(
            token_id.to_string() == self.token_id.to_string(),
            "Wrong token on deposit"
        );
        env_log!("Deposit {} of {} token", amount, self.token_id);
        self.balance += amount
    }

    // Withdraws tokens
    #[payable]
    pub(crate) fn withdraw_token(&mut self, amount: Balance) {
        let account_id = env::predecessor_account_id();

        ext_fungible_token::ft_transfer(
            account_id,
            amount.into(),
            Some("Withdraw token".to_string()),
            self.token_id.clone(),
            1, // required 1yNEAR for transfers
            GAS_FOR_FT_TRANSFER,
        );
    }
}
