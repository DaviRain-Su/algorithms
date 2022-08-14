use crate::utils::to_hex_string;

#[test]
fn test_rs_merkle_tree() {
    use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
    let leaf_values = ["a", "b", "c", "d"];
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    let leaves_hex_string = leaves.clone().iter().map(|x| to_hex_string(x)).collect::<Vec<_>>();
    println!("leaves_hex_string = {:#?}", leaves_hex_string);

    let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::new();
    merkle_tree.append(leaves.clone().as_mut());
    assert_eq!(merkle_tree.root(), None);

    merkle_tree.commit();
    println!("{:?}", merkle_tree.root_hex());

    let indx_to_prove = vec![0];
    let merkle_proof = merkle_tree.proof(&indx_to_prove);
    let proof_hash = merkle_proof.proof_hashes();
    let proof_hash_str = proof_hash.iter().map(|value| to_hex_string(value)).collect::<Vec<_>>();
    let merkle_root = merkle_tree.root().unwrap();

    println!("{:?}", to_hex_string(&merkle_root));
    println!("proof_hash = {:#?}", proof_hash_str);


    let a_hash = Sha256::hash(b"a");
    println!("a_hash = {:?}", to_hex_string(&a_hash));
    let b_hash = Sha256::hash(b"b");
    println!("b_hash = {:?}", to_hex_string(&b_hash));
    let ab_hash = crate::merkle_tree::merkle_parent(&a_hash, &proof_hash[0]);
    println!("ab_hash = {:?}", to_hex_string(&ab_hash));
    let abcd_hash = crate::merkle_tree::merkle_parent(&ab_hash, &proof_hash[1]);
    println!("abcd_hash = {:?}", to_hex_string(&abcd_hash));
    // let cdpp_hash = crate::merkle_tree::merkle_parent(&cdp_hash, &proof_hash[2]);
    // println!("cdpp_hash = {:?}", to_hex_string(&cdpp_hash));
}
