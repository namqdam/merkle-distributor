use near_sdk::Promise;

use crate::*;

impl MerkleDistributor {
    pub(crate) fn internal_deposit(&self) -> u128 {
        let amount = env::attached_deposit();
        amount
    }

    pub(crate) fn internal_transfer(&self, amount: Balance) {
        let account_id = env::predecessor_account_id();
        Promise::new(account_id).transfer(amount);
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn assert_paused(&self) {
        assert_eq!(false, self.paused, "Can only be called when not paused");
    }
}
