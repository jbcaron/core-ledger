use crate::block::{self, Block};
use crate::account::Account;
use std::collections::HashMap;
use crate::utils::is_valid_public_key;
use crate::transaction::Transaction;

struct Blockchain {
	blocks: Vec<Block>,
	pending_block: Block,
	accounts: HashMap<Vec<u8>, Account>,
}

impl Blockchain {
	pub fn new(genesis_block: &Block) -> Result<Blockchain, &'static str> {
		check_genesis_block(genesis_block)?;

		let mut blockchain = Blockchain {
			blocks: vec![genesis_block.clone()],
			pending_block: Block::new(1, genesis_block.hash()),
			accounts: HashMap::new(),
		};

		for tx in genesis_block.transactions() {
			blockchain.execute_transaction_genesis(tx)?;
		}
		Ok(blockchain)
	}

	fn is_existing_account(&self, address: &[u8]) -> bool {
		self.accounts.contains_key(address)
	}

	fn add_account(&mut self, address: Vec<u8>) -> Result<&Account, &'static str> {
		if !is_valid_public_key(&address) {
			return Err("Invalid public key");
		}
		if self.is_existing_account(&address) {
			return Err("Account already exists");
		}
		let account = Account::new(address.clone());
		self.accounts.insert(address.clone(), account);
		Ok(self.accounts.get(&address).unwrap())
	}

	fn get_account(&self, address: &[u8]) -> Option<&Account> {
		self.accounts.get(address)
	}

	fn get_account_mut(&mut self, address: &[u8]) -> Option<&mut Account> {
		self.accounts.get_mut(address)
	}

	pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), &'static str> {
		self.execute_transaction(&tx)?;
		self.pending_block.add_transaction(&tx);
		Ok(())
	}

	fn execute_transaction(&mut self, tx: &Transaction, ) -> Result<(), &'static str> {
		if tx.verify() == false {
			return Err("Invalid transaction");
		}

		let amount = tx.amount();
		if amount == 0 {
			return Err("Invalid transaction amount");
		}

		let from_account = self.get_account(&tx.from()).ok_or("From account not found")?;

		if from_account.nonce() != tx.nonce() {
			return Err("Invalid nonce");
		}

		if from_account.balance() < amount {
			return Err("Insufficient funds");
		}

		let to_account = match self.get_account(&tx.to()) {
			Some(account) => account,
			None => self.add_account(tx.to().clone()).unwrap(),
		};

		if to_account.balance() + tx.amount() < to_account.balance() {
			return Err("Overflow error");
		}

		self.get_account_mut(&tx.from()).unwrap().transfer(amount);
		self.get_account_mut(&tx.to()).unwrap().deposit(amount);
		self.get_account_mut(&tx.from()).unwrap().increment_nonce();


		Ok(())
	}

	/// ignore the nonce check and from account balance check
	fn execute_transaction_genesis(&mut self, tx: &Transaction) -> Result<(), &'static str> {
		if tx.verify() == false {
			return Err("Invalid transaction");
		}
		let amount = tx.amount();
		if amount == 0 {
			return Err("Invalid transaction amount");
		}
		let to_acount = match self.get_account(&tx.to()) {
			Some(account) => account,
			None => self.add_account(tx.to().clone()).unwrap(),
		};
		if to_acount.balance() + amount < to_acount.balance() {
			return Err("Overflow error");
		}
		self.get_account_mut(&tx.to()).unwrap().deposit(amount);
		Ok(())
	}

	pub fn last_block(&self) -> Option<&Block> {
		self.blocks.last()
	}

	pub fn pending_block(&self) -> &Block {
		&self.pending_block
	}

	pub fn finalize_and_mint_pending_block(&mut self) {
		self.pending_block.finalize();
		self.blocks.push(self.pending_block.to_owned());
		self.pending_block = Block::new(self.last_block().unwrap().index() + 1, self.last_block().unwrap().hash());
	}


	pub fn last_block_hash(&self) -> Option<Vec<u8>> {
		self.last_block().map(|b| b.hash())
	}

	pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
		self.blocks.get(index as usize)
	}

	pub fn get_block_by_hash(&self, hash: &[u8]) -> Option<&Block> {
		self.blocks.iter().find(|b| b.hash() == hash)
	}

	
}

fn check_genesis_block(genesis_block: &Block) -> Result<(), &'static str> {
	if genesis_block.index() != 0 {
		return Err("Invalid index for genesis block");
	}
	if genesis_block.previous_hash() != vec![] {
		return Err("Invalid previous hash for genesis block");
	}
	genesis_block.verify()?;
	Ok(())
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::block::Block;
	use crate::{transaction, utils};
use crate::utils::hash;


	#[test]
	fn test_new() {
		let genesis_block = Block::new_genesis(vec![], 0);
		let blockchain = Blockchain::new(&genesis_block).unwrap();
		assert_eq!(blockchain.blocks.len(), 1);
		assert_eq!(blockchain.last_block().unwrap(), &genesis_block);
		assert_eq!(blockchain.pending_block().index(), 1);
		assert_eq!(blockchain.pending_block().previous_hash(), genesis_block.hash());
	}

	#[test]
	fn test_finalize_and_mint_pending_block() {
		let genesis_block = Block::new_genesis(vec![], 0);
		let mut blockchain = Blockchain::new(&genesis_block).unwrap();
		blockchain.finalize_and_mint_pending_block();
		assert_eq!(blockchain.blocks.len(), 2);
		assert_eq!(blockchain.last_block().unwrap().index(), 1);
		assert_eq!(blockchain.last_block().unwrap().previous_hash(), genesis_block.hash());
		assert_eq!(blockchain.pending_block().index(), 2);
		assert_eq!(blockchain.pending_block().previous_hash(), blockchain.last_block().unwrap().hash());
	}

	#[test]
	fn test_last_block_hash() {
		let genesis_block = Block::new_genesis(vec![], 0);
		let mut blockchain = Blockchain::new(&genesis_block).unwrap();
		assert_eq!(blockchain.last_block_hash().unwrap(), genesis_block.hash());
		blockchain.finalize_and_mint_pending_block();
		assert_eq!(blockchain.last_block_hash().unwrap(), blockchain.last_block().unwrap().hash());
	}

	#[test]
	fn test_get_block_by_index() {
		let genesis_block = Block::new_genesis(vec![], 0);
		let mut blockchain = Blockchain::new(&genesis_block).unwrap();
		blockchain.finalize_and_mint_pending_block();
		let last_block = blockchain.last_block().unwrap().to_owned();
		assert_eq!(blockchain.get_block_by_index(0).unwrap(), &genesis_block);
		assert_eq!(blockchain.get_block_by_index(1).unwrap(), &last_block);
	}

	#[test]
	fn test_get_block_by_hash() {
		let genesis_block = Block::new_genesis(vec![], 0);
		let mut blockchain = Blockchain::new(&genesis_block).unwrap();
		blockchain.finalize_and_mint_pending_block();
		let last_block = blockchain.last_block().unwrap().to_owned();
		assert_eq!(blockchain.get_block_by_hash(&genesis_block.hash()).unwrap(), &genesis_block);
		assert_eq!(blockchain.get_block_by_hash(&last_block.hash()).unwrap(), &last_block);
	}

	#[test]
	fn test_add_transaction() {
		let (private_key, public_key) = utils::generate_keypair();

		let transaction = transaction::Transaction::new_and_sign(public_key.clone(), public_key.clone(), 100, 0, &private_key);
		let genesis_block = Block::new_genesis(vec![transaction], 0);
		let mut blockchain = Blockchain::new(&genesis_block).unwrap();
		let (_, to) = utils::generate_keypair();
		let tx = Transaction::new_and_sign(public_key.clone(), to.clone(), 30, 0, &private_key);
		blockchain.add_transaction(tx).unwrap();
		assert_eq!(blockchain.pending_block().transactions().len(), 1);
		assert_eq!(blockchain.get_account(&public_key).unwrap().balance(), 70);
		assert_eq!(blockchain.get_account(&to).unwrap().balance(), 30);
	}
}