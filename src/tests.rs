#[test]
fn test_rs_merkle_tree() {
    use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
    let leaf_values = ["a", "b"];
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::new();
    merkle_tree.append(leaves.clone().as_mut());
    assert_eq!(merkle_tree.root(), None);

    merkle_tree.commit();
    println!("{:?}", merkle_tree.root_hex());

    // assert_eq!(
    //     merkle_tree.root_hex(),
    //     Some("1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2".to_string())
    // );
}