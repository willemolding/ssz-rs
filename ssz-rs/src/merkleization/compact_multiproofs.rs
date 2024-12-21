//! Experimental support for compact multiproofs.
use crate::{
    lib::*,
    merkleization::{
        multiproofs::{get_helper_indices, get_path_indices},
        GeneralizedIndex, MerkleizationError as Error, Node,
    },
};
use bitvec::{index, prelude::*};
use sha2::{Digest, Sha256};

pub type Descriptor = BitVec<u8, Msb0>;

pub fn compute_proof_indices(indices: &[GeneralizedIndex]) -> Vec<GeneralizedIndex> {
    let mut indices_set: HashSet<GeneralizedIndex> = HashSet::new();
    for &index in indices {
        let helper_indices = get_helper_indices(&[index]);
        for helper_index in helper_indices {
            indices_set.insert(helper_index);
        }
    }
    for &index in indices {
        let path_indices = get_path_indices(index);
        for path_index in path_indices {
            indices_set.remove(&path_index);
        }
        indices_set.insert(index);
    }
    let mut sorted_indices: Vec<GeneralizedIndex> = indices_set.into_iter().collect();
    sorted_indices.sort_by_key(|index| format!("{:b}", *index));
    sorted_indices
}

pub fn compute_proof_descriptor(indices: &[GeneralizedIndex]) -> Result<BitVec<u8, Msb0>, Error> {
    let indices = compute_proof_indices(indices);
    let mut descriptor = BitVec::<u8, Msb0>::new();
    for index in indices {
        descriptor.extend(std::iter::repeat(false).take(index.trailing_zeros() as usize));
        descriptor.push(true);
    }
    Ok(descriptor)
}

struct Pointer {
    bit_index: usize,
    node_index: usize,
}

pub fn calculate_compact_multi_merkle_root(
    nodes: &[Node],
    descriptor: &BitVec<u8, Msb0>,
) -> Result<Node, Error> {
    let mut ptr = Pointer { bit_index: 0, node_index: 0 };
    let root = calculate_compact_multi_merkle_root_inner(nodes, &descriptor, &mut ptr)?;
    if ptr.bit_index != descriptor.len() || ptr.node_index != nodes.len() {
        Err(Error::InvalidProof)
    } else {
        Ok(root)
    }
}

fn calculate_compact_multi_merkle_root_inner(
    nodes: &[Node],
    descriptor: &BitVec<u8, Msb0>,
    ptr: &mut Pointer,
) -> Result<Node, Error> {
    let bit = descriptor[ptr.bit_index];
    ptr.bit_index += 1;
    if bit {
        let node = nodes[ptr.node_index];
        ptr.node_index += 1;
        Ok(node)
    } else {
        let left = calculate_compact_multi_merkle_root_inner(nodes, descriptor, ptr)?;
        let right = calculate_compact_multi_merkle_root_inner(nodes, descriptor, ptr)?;
        Ok(hash_pair(&left, &right))
    }
}

pub fn verify_compact_merkle_multiproof(
    nodes: &[Node],
    descriptor: &BitVec<u8, Msb0>,
    root: Node,
) -> Result<(), Error> {
    if calculate_compact_multi_merkle_root(nodes, descriptor)? == root {
        Ok(())
    } else {
        Err(Error::InvalidProof)
    }
}

fn hash_pair(left: &Node, right: &Node) -> Node {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    Node::from_slice(hasher.finalize().as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkleization::proofs::tests::decode_node_from_hex;

    #[test]
    fn test_compute_proof_descriptor() {
        assert_eq!(
            compute_proof_descriptor(&[42]).expect("can make descriptor").into_vec(),
            vec![0x25_u8, 0xe0_u8]
        );
        assert_eq!(
            compute_proof_descriptor(&[5567]).expect("can make descriptor").into_vec(),
            vec![0x25_u8, 0x2a_u8, 0xaf_u8, 0x80_u8]
        );
        assert_eq!(
            compute_proof_descriptor(&[66]).expect("can make descriptor").into_vec(),
            vec![0x5_u8, 0xf8_u8]
        );
    }

    #[test]
    fn test_verify_compact_merkle_multiproof() {
        let descriptor = compute_proof_descriptor(&[42]).expect("can make descriptor");

        let expected_state_root = decode_node_from_hex(
            "0x7903bc7cc62f3677c5c0e38562a122638a3627dd945d1f7992e4d32f1d4ef11e",
        );
        let invalid_state_root = decode_node_from_hex(
            "0x7903bc7cc62f3677c5c0e38562a122638a3627dd945d1f7992e4d32f1d4ef11f",
        );

        let branch = [
            "0xa00117d138e95bae66918e6476661d32755f67745f684c90d47f8965327024be",
            "0x822e4005e9a99822945a0fcb648506f3dae4335ca76da7b0cdfe9d4813db0451",
            "0x201d160000000000000000000000000000000000000000000000000000000000",
            "0x572135114f5b6d116e4a6630ba0379c1ea7bacdadc6bd5bf86279ae79279dde1",
            "0x28969b2b8d1a4eead3bbd1815ca49a1efcf9bbb448530b8f1ddac0eb8b96014d",
            "0xcad3a7c4a4edad9f266b0b4052da48011aa7febd52c4b9f3c5293e79c88aa4cf",
        ]
        .into_iter()
        .map(decode_node_from_hex)
        .collect::<Vec<_>>();

        assert!(verify_compact_merkle_multiproof(&branch, &descriptor, expected_state_root).is_ok());
        assert!(verify_compact_merkle_multiproof(&branch, &descriptor, invalid_state_root).is_err());
    }
}
