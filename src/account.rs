use crate::crypto::PublicKey;

#[derive(Debug, Clone)]
pub struct Account {
    address: PublicKey,
    balance: u64,
    nonce: u64,
}

impl From<PublicKey> for Account {
    fn from(address: PublicKey) -> Self {
        Account::new(&address)
    }
}

impl Account {
    pub fn new(address: &PublicKey) -> Account {
        Account {
            address: address.clone(),
            balance: 0,
            nonce: 0,
        }
    }

    pub fn address(&self) -> PublicKey {
        self.address.clone()
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn transfer(&mut self, amount: u64) {
        self.balance -= amount;
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
}
