use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::{hash, generate_keypair};
use crate::transaction::Transaction;
use crate::merkle;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
	index: u64,
	timestamp: u64,
	previous_hash: Vec<u8>,
	transactions_root: Vec<u8>,
	transactions: Vec<Transaction>,
	hash: Vec<u8>,
}

impl Block {
	pub fn new(index: u64, previous_hash: Vec<u8>) -> Block {
		Block {
			index,
			timestamp: 0,
			previous_hash,
			transactions_root: vec![],
			transactions: vec![],
			hash: vec![],
		}
	}

	pub fn new_genesis(transactions: Vec<Transaction>, timestamp: u64) -> Block {
		let mut block = Block::new(0, vec![]);
		block.transactions = transactions;
		block.timestamp = timestamp;
		block.transactions_root = merkle::root_hash(block.transactions.iter().map(|tx| tx.hash()).collect());
		block.hash = block.compute_hash();
		
		block
	}

	pub fn add_transaction(&mut self, transaction: &Transaction) {
		self.transactions.push(transaction.clone());
	}

	pub fn finalize(&mut self) {
		self.timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("error getting system time")
			.as_secs();
		self.transactions_root = merkle::root_hash(self.transactions.iter().map(|tx| tx.hash()).collect());
		self.hash = self.compute_hash();
	}

	pub fn verify(&self) -> Result<(), &'static str> {
		if self.transactions.iter().any(|tx| tx.verify() == false) {
			return Err("Invalid transaction signature");
		}
		if self.transactions_root != merkle::root_hash(self.transactions.iter().map(|tx| tx.hash()).collect()) {
			return Err("Invalid transactions root");
		}
		if self.hash != self.compute_hash() {
			return Err("Invalid block hash");
		}
		Ok(())
	}

	fn to_vec(&self) -> Vec<u8> {
		let timestamp_bytes = self.timestamp.to_be_bytes();
		let index_bytes = self.index.to_be_bytes();
		[
			&index_bytes[..],
			&timestamp_bytes[..],
			&self.previous_hash[..],
			&self.transactions_root[..],
		].concat()
	}

	fn compute_hash(&self) -> Vec<u8> {
		hash(&self.to_vec())
	}

	pub fn index(&self) -> u64 {
		self.index
	}

	pub fn timestamp(&self) -> u64 {
		self.timestamp
	}

	pub fn previous_hash(&self) -> Vec<u8> {
		self.previous_hash.clone()
	}

	pub fn transactions_root(&self) -> Vec<u8> {
		self.transactions_root.clone()
	}

	pub fn hash(&self) -> Vec<u8> {
		self.hash.clone()
	}

	pub fn transactions(&self) -> &Vec<Transaction> {
		&self.transactions
	}

}

#[cfg(test)]
mod tests {
	use secp256k1::PublicKey;

use super::*;
	use crate::account;
use crate::utils::hash;
	use crate::transaction::Transaction;

	#[test]
	fn test_new() {
		let index = 0;
		let previous_hash = hash(b"");
		let block = Block::new(index, previous_hash.clone());
		assert_eq!(block.index(), index);
		assert_eq!(block.previous_hash(), previous_hash);
		assert_eq!(block.transactions_root(), vec![]);
		assert_eq!(block.transactions(), &vec![]);
		assert_eq!(block.hash(), vec![]);
	}


	#[test]
	fn test_add_transaction() {
		let index = 0;
		let previous_hash = hash(b"");
		let mut block = Block::new(index, previous_hash.clone());
		let (private_key, _public_key) = generate_keypair();
		let tx = Transaction::new_and_sign(vec![1], vec![2], 100, 0, &private_key);
		block.add_transaction(&tx);
		assert_eq!(block.transactions(), &vec![tx]);
	}

	#[test]
	fn test_finalize() {
		let index = 0;
		let previous_hash = hash(b"");
		let mut block = Block::new(index, previous_hash.clone());
		let (private_key, public_key) = generate_keypair();
		let (_, to) = generate_keypair();
		let tx = Transaction::new_and_sign(public_key, to, 100, 0, &private_key);
		block.add_transaction(&tx);
		block.finalize();
		assert_eq!(block.transactions_root(), merkle::root_hash(vec![tx.hash()]));
		block.verify().unwrap();
	}

	#[test]
	fn test_finalize_empty() {
		let index = 0;
		let previous_hash = hash(b"");
		let mut block = Block::new(index, previous_hash.clone());
		block.finalize();
		block.verify().unwrap();
	}
}