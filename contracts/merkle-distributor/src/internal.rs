use crate::*;

impl MerkleDistributor {
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
