use std::time::{SystemTime, UNIX_EPOCH};

use crate::merkle;
use crate::transaction::Transaction;
use crate::crypto::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    index: u64,
    timestamp: u64,
    previous_hash: Hash,
    transactions_root: Hash,
    transactions: Vec<Transaction>,
    hash: Hash,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockBuilder {
    index: u64,
    previous_hash: Hash,
    transactions: Vec<Transaction>,
}

impl BlockBuilder {
    pub fn new(index: u64, previous_hash: &Hash) -> BlockBuilder {
        BlockBuilder {
            index,
            previous_hash: previous_hash.clone(),
            transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) {
        self.transactions.push(transaction.clone());
    }

    pub fn hash(&self, timestamp: u64) -> Hash {
        let timestamp_bytes = timestamp.to_be_bytes();
        let index_bytes = self.index.to_be_bytes();
        let transactions_root = self.transactions_root();
        let data = [
            &index_bytes[..],
            &timestamp_bytes[..],
            self.previous_hash.as_ref(),
            transactions_root.as_ref(),
        ];
        Hash::from(data.concat().as_ref())
    }

    pub fn build(self) -> Block {
        Block::from(self)
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn previous_hash(&self) -> Hash {
        self.previous_hash.clone()
    }

    pub fn transactions_root(&self) -> Hash {
        merkle::root_hash(self.transactions.iter().map(|tx| tx.hash()).collect())
    }
}

impl From<BlockBuilder> for Block {
    fn from(builder: BlockBuilder) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error getting system time")
            .as_secs();
        let hash = builder.hash(timestamp);
        Self {
            index: builder.index,
            timestamp,
            previous_hash: builder.previous_hash,
            transactions_root: merkle::root_hash(builder.transactions.iter().map(|tx| tx.hash()).collect()),
            transactions: builder.transactions,
            hash,
        }
    }
}

impl Block {

    pub fn new_genesis(transactions: Vec<Transaction>, timestamp: u64) -> Result<Block, String> {
        if transactions.is_empty() {
            return Err("Genesis block must have at least one transaction".to_string());
        }
        transactions.iter().try_for_each(|tx| tx.verify())?;
        let genesis_block = BlockBuilder {
            index: 0,
            previous_hash: Hash::default(),
            transactions: transactions.clone(),
        };

        let hash = genesis_block.hash(timestamp);
        Ok(Block {
            index: 0,
            timestamp,
            previous_hash: Hash::default(),
            transactions_root: genesis_block.transactions_root(),
            transactions,
            hash,
        })
    }

    pub fn hash(&self) -> Hash {
        self.hash.clone()
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn previous_hash(&self) -> Hash {
        self.previous_hash.clone()
    }

    pub fn transactions_root(&self) -> Hash {
        self.transactions_root.clone()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}
