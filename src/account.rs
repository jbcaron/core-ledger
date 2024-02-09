#[derive(Debug, Clone)]
pub struct Account {
    address: Vec<u8>,
    balance: u64,
    nonce: u64,
}

impl Account {
    pub fn new(address: Vec<u8>) -> Account {
        Account {
            address,
            balance: 0,
            nonce: 0,
        }
    }

	pub fn address(&self) -> Vec<u8> {
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
