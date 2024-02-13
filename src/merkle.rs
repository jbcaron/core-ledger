use crate::crypto::Hash;

pub fn root_hash(hashes: Vec<Hash>) -> Hash {
    if hashes.is_empty() {
        return Hash::default();
    }
    let mut parent_hashes = vec![];
    for chunk in hashes.chunks(2) {
        let hash = if chunk.len() > 1 {
            combine_hashes(&chunk[0], &chunk[1])
        } else {
            combine_hashes(&chunk[0], &chunk[0])
        };
        parent_hashes.push(hash);
    }
    if parent_hashes.len() > 1 {
        root_hash(parent_hashes)
    } else {
        parent_hashes[0].clone()
    }
}

fn combine_hashes(left: &Hash, right: &Hash) -> Hash {
    Hash::from([left.as_ref(), right.as_ref()].concat().as_ref())
}
