use crate::*;

impl MerkleDistributor {
    pub(crate) fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Can only be called by the owner"
        );
    }

    pub(crate) fn assert_paused(&self) {
        require!(!self.paused, "Can only be called when not paused");
    }
}
