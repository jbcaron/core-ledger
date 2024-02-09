use crate::utils::hash;

pub fn root_hash(hashes: Vec<Vec<u8>>) -> Vec<u8> {
	if hashes.is_empty() {
		return vec![];
	}
	let mut parent_hashes= vec![];
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

fn combine_hashes(left: &[u8], right: &[u8]) -> Vec<u8> {
	hash(&[left, right].concat())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::hash;

	#[test]
	fn test_combine_hashes() {
		let left = hash(b"one");
		let right = hash(b"two");
		let combined = combine_hashes(&left, &right);
		assert_eq!(combined, hash(&[left, right].concat()));
	}

	#[test]
	fn test_root_hash_even() {
		let hashes = vec![
			hash(b"one"),
			hash(b"two"),
			hash(b"three"),
			hash(b"four"),
		];
		let root = root_hash(hashes);
		assert_eq!(root, combine_hashes(&combine_hashes(&hash(b"one"), &hash(b"two")), &combine_hashes(&hash(b"three"), &hash(b"four"))));
	}

	#[test]
	fn test_root_hash_odd() {
		let hashes = vec![
			hash(b"one"),
			hash(b"two"),
			hash(b"three"),
		];
		let root = root_hash(hashes);
		assert_eq!(root, combine_hashes(&combine_hashes(&hash(b"one"), &hash(b"two")), &combine_hashes(&hash(b"three"), &hash(b"three"))));
	}

	#[test]
	fn test_root_hash_single() {
		let hashes = vec![
			hash(b"one"),
		];
		let root = root_hash(hashes);
		assert_eq!(root, combine_hashes(&hash(b"one"), &hash(b"one")));
	}

	#[test]
	fn test_root_hash_empty() {
		let hashes = vec![];
		let root = root_hash(hashes);
		assert_eq!(root, vec![]);
	}
}