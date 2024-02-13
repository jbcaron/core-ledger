use crate::account::Account;
use crate::block::{Block, BlockBuilder};
use crate::transaction::Transaction;
use crate::crypto::{PublicKey, Hash};
use std::collections::HashMap;

struct Blockchain {
    blocks: Vec<Block>,
    pending_block: BlockBuilder,
    accounts: HashMap<PublicKey, Account>,
}

impl Blockchain {
    pub fn new(transaction: Transaction, timestamp: u64) -> Result<Blockchain, String> {
        let genesis_block = Block::new_genesis(vec![transaction], timestamp)?;
        let hash = genesis_block.hash();
        
        let mut blockchain = Blockchain {
            blocks: vec![genesis_block.clone()],
            pending_block: BlockBuilder::new(1, &hash),
            accounts: HashMap::new(),
        };

        for tx in genesis_block.transactions() {
            blockchain.execute_transaction_genesis(tx)?;
        }
        Ok(blockchain)
    }

    fn is_existing_account(&self, address: &PublicKey) -> bool {
        self.accounts.contains_key(address)
    }

    fn add_account(&mut self, address: &PublicKey) -> Result<&Account, String> {
        if self.is_existing_account(&address) {
            return Err("Account already exists".to_string());
        }
        let account = Account::new(address);
        self.accounts.insert(address.clone(), account);
        Ok(self.accounts.get(&address).unwrap())
    }

    pub fn get_account(&self, address: &PublicKey) -> Option<&Account> {
        self.accounts.get(address)
    }

    fn get_account_mut(&mut self, address: &PublicKey) -> Option<&mut Account> {
        self.accounts.get_mut(address)
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        self.execute_transaction(&tx)?;
        self.pending_block.add_transaction(&tx);
        Ok(())
    }

    fn execute_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        tx.verify()?;

        let amount = tx.amount();
        if amount == 0 {
            return Err("Invalid transaction amount".to_string());
        }

        let from_account = self
            .get_account(&tx.from())
            .ok_or("From account not found")?;

        if from_account.nonce() != tx.nonce() {
            return Err("Invalid nonce".to_string());
        }

        if from_account.balance() < amount {
            return Err("Insufficient funds".to_string());
        }

        let to_account = match self.get_account(&tx.to()) {
            Some(account) => account,
            None => self.add_account(&tx.to()).unwrap(),
        };

        if to_account.balance() + tx.amount() < to_account.balance() {
            return Err("Overflow error".to_string());
        }

        {
            self.get_account_mut(&tx.from()).unwrap().transfer(amount);
        }
        {
            self.get_account_mut(&tx.to()).unwrap().deposit(amount);
        }
        {
            self.get_account_mut(&tx.from()).unwrap().increment_nonce();
        }

        Ok(())
    }

    /// ignore the nonce check and from account balance check
    fn execute_transaction_genesis(&mut self, tx: &Transaction) -> Result<(), String> {
        tx.verify()?;
        let amount = tx.amount();
        if amount == 0 {
            return Err("Invalid transaction amount".to_string());
        }
        let to_acount = match self.get_account(&tx.to()) {
            Some(account) => account,
            None => self.add_account(&tx.to()).unwrap(),
        };
        if to_acount.balance() + amount < to_acount.balance() {
            return Err("Overflow error".to_string());
        }
        self.get_account_mut(&tx.to()).unwrap().deposit(amount);
        Ok(())
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn pending_block(&self) -> &BlockBuilder {
        &self.pending_block
    }

    pub fn finalize_and_mint_pending_block(&mut self) {
        self.blocks.push(Block::from(self.pending_block.clone()));
        self.pending_block = BlockBuilder::new (
            self.last_block().unwrap().index() + 1,
            &self.last_block().unwrap().hash(),
        );
    }

    pub fn last_block_hash(&self) -> Option<Hash> {
        self.last_block().map(|b| b.hash())
    }

    pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
        self.blocks.get(index as usize)
    }

    pub fn get_block_by_hash(&self, hash: &Hash) -> Option<&Block> {
        self.blocks.iter().find(|b| b.hash() == *hash)
    }
}
