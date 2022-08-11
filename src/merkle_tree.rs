use core::hash;

use sha2::{Digest, Sha256};

/// Merkle tree general use the blockchain
/// In the block header have merkle tree word in block chain.
pub struct MerkleTree;

// contruct merkle tree step
// 1. use hash function for all element do hash
// 2. if list just only one element will return this function.
// 3. otherwise if elements number is odd, we copy the last element and appent the list last.
// so we have the event number list.
// 4. double-couple, merge hash, so we the they are parent node.
// 5. repeat 2

/// calcuate merkle parent hash
pub fn merkle_parent(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.append(&mut right);

    let mut hasher = Sha256::new();
    hasher.update(left);

    let mut result = [0u8; 32];
    result.copy_from_slice(&hasher.finalize().to_vec());
    result
}

pub fn merkle_parent_level(hasher: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut hasher_clone = hasher.clone();
    if hasher.len() % 2 == 1 {
        hasher_clone.push(hasher.last().unwrap().clone());
    }
    let mut parent_level = Vec::<[u8; 32]>::new();
    for (index, _) in hasher_clone.iter().enumerate().step_by(2) {
        let parent = merkle_parent(&hasher_clone[index], &hasher_clone[index + 1]);
        parent_level.push(parent);
    }
    parent_level
}


// FIXME: when hasher is odd is not same rs_merkle generate merkle root
pub fn merkle_root(hasher: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut current_hashes = hasher.clone();
    while current_hashes.len() > 1 {
        current_hashes = merkle_parent_level(current_hashes);
    }
    current_hashes
}

pub fn hash_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_value = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(&hash_value.to_vec());
    result
}

#[test]
fn test_hash_256() {
    use crate::utils::to_hex_string;
    let result = hash_256(b"hello world");

    println!("{}", to_hex_string(&result));
    assert_eq!(
        to_hex_string(&result),
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string()
    );
}

#[test]
fn test_merkle_parent() {
    use crate::utils::to_hex_string;

    let left = hash_256(b"a");
    let right = hash_256(b"b");
    let parent = merkle_parent(&left, &right);

    println!("{}", to_hex_string(&parent));
    println!("e5a01fee14e0ed5c48714f22180f25ad8365b53f9779f79dc4a3d7e93963f94a");
}

// >>> from helper import hash256
//>>> hash0 = bytes.fromhex('c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5')
//>>> hash1 = bytes.fromhex('c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5')
//>>> parent = hash256(hash0 + hash1)
//>>> print(parent.hex())
//8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd

#[test]
fn test_string_concat() {
    let hash0 = "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5";
    let hash1 = "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5";
    let sum_hash = format!("{}{}", hash0, hash1);
    println!("{}", sum_hash);

    let mut hash0_vec = hash0.as_bytes().to_vec();
    let mut hash1_vec = hash1.as_bytes().to_vec();
    hash0_vec.append(&mut hash1_vec);
    let sum_hash_string = String::from_utf8(hash0_vec).unwrap();
    println!("{}", sum_hash_string);
}

#[test]
fn test_merkle_parent_level() {
    use crate::utils::to_hex_string;
    let leaf_values = ["a", "b", "c", "d", "e"];
    let leaf: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|value| hash_256(value.as_bytes()))
        .collect();
    let result = merkle_parent_level(leaf);
    for item in result.iter() {
        println!("{:?}", to_hex_string(item));
    }
}


#[test]
fn test_merkle_root() {
    use crate::utils::to_hex_string;
    let leaf_values = ["a", "b", "c", "d"];
    let leaf: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|value| hash_256(value.as_bytes()))
        .collect();
    let result = merkle_root(leaf);
    for item in result.iter() {
        println!("{:?}", to_hex_string(item));
    }
    // 14ede5e8e97ad9372327728f5099b95604a39593cac3bd38a343ad76205213e7
}