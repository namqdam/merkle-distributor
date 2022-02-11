use near_sdk::env::sha256;

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash = sha256(&[computed_hash, proof_element].concat())
                .try_into()
                .unwrap();
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash = sha256(&[proof_element, computed_hash].concat())
                .try_into()
                .unwrap();
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}
