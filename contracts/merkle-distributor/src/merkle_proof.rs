use near_sdk::env::keccak256;
use std::convert::TryInto;

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash =
                vec_to_array::<u8, 32>(keccak256(&[computed_hash, proof_element].concat()));
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash =
                vec_to_array::<u8, 32>(keccak256(&[proof_element, computed_hash].concat()));
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    let boxed_slice = v.into_boxed_slice();
    let boxed_array: Box<[T; N]> = match boxed_slice.try_into() {
        Ok(ba) => ba,
        Err(o) => panic!("Expected a Vec of length {} but it was {}", N, o.len()),
    };
    *boxed_array
}
