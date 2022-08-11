use sha2::{Sha256, Digest};

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

/// 
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

// left_value:  c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5
// right_value: c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5
// parent hash: 8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd

pub fn hash_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_value = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(&hash_value.to_vec());
    result
}

fn byte_to_hex(byte: &u8) -> String {
    format!("{:02x}", byte)
}

/// Serializes bytes into a hex string
pub fn to_hex_string<T: Clone + Into<Vec<u8>>>(bytes: &T) -> String {
    let hex_vec: Vec<String> = bytes.clone().into().iter().map(byte_to_hex).collect();

    hex_vec.join("")
}

#[test]
fn test_hash_256() {
   

    let result = hash_256(b"hello world");
    
    println!("{}", to_hex_string(&result));
    assert_eq!(to_hex_string(&result),"b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string());
}

#[test]
fn test_merkle_parent() {
    use hex_literal::hex;
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