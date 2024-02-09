use crate::{is_valid_public_key, utils::{hash, sign_hash, verify_signature}};

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    from: Vec<u8>,
    to: Vec<u8>,
    amount: u64,
    nonce: u64,
    signature: Vec<u8>,
}

impl Transaction {
    pub fn new(from: Vec<u8>, to: Vec<u8>, amount: u64, nonce: u64) -> Transaction {
        Transaction {
            from,
            to,
            amount,
            nonce,
            signature: vec![],
        }
    }

	pub fn new_signed(from: Vec<u8>, to: Vec<u8>, amount: u64, nonce: u64, signature: Vec<u8>) -> Transaction {
		Transaction {
			from,
			to,
			amount,
			nonce,
			signature,
		}
	}

	pub fn new_and_sign(from: Vec<u8>, to: Vec<u8>, amount: u64, nonce: u64, private_key: &[u8]) -> Transaction {
		let mut tx = Transaction::new(from, to, amount, nonce);
		tx.sign(private_key);
		tx
	}

	pub fn from(&self) -> Vec<u8> {
		self.from.clone()
	}

	pub fn to(&self) -> Vec<u8> {
		self.to.clone()
	}

	pub fn amount(&self) -> u64 {
		self.amount
	}

	pub fn nonce(&self) -> u64 {
		self.nonce
	}

	pub fn signature(&self) -> Vec<u8> {
		self.signature.clone()
	}

    pub fn hash(&self) -> Vec<u8> {
        hash(&self.to_vec())
    }

    pub fn sign(&mut self, private_key: &[u8]) {
        let hash = self.hash();
        self.signature = sign_hash(&hash, private_key);
    }

    pub fn verify(&self) -> bool {
        let hash = self.hash();
		is_valid_public_key(&self.from) && is_valid_public_key(&self.to) &&
        verify_signature(&self.from, &hash, &self.signature)
    }

    fn to_vec(&self) -> Vec<u8> {
        let amount_bytes = self.amount.to_be_bytes();
        let nonce_bytes = self.nonce.to_be_bytes();

        [
            &self.from[..],
            &self.to[..],
            &amount_bytes[..],
            &nonce_bytes[..],
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use crate::utils::generate_keypair;

    #[test]
    fn test_transaction() {
        let (private_key, public_key) = generate_keypair();
        let from = public_key;
        let (_, to) = generate_keypair();
        let amount = 100;
        let nonce = 0;
        let mut tx = Transaction::new(from.clone(), to.clone(), amount, nonce);
        tx.sign(&private_key);
        assert!(tx.verify());
    }

    #[test]
    fn test_invalid_transaction() {
        let (_private_key, public_key) = generate_keypair();
        let from = public_key;
        let (_, to) = generate_keypair();
        let amount = 100;
        let nonce = 0;
        let mut tx = Transaction::new(from.clone(), to.clone(), amount, nonce);
        let (private_key2, _public_key2) = generate_keypair();
        tx.sign(&private_key2);
        assert!(!tx.verify());
    }

	#[test]
	fn test_new_and_sign() {
		let (private_key, public_key) = generate_keypair();
		let from = public_key;
		let (_, to) = generate_keypair();
		let amount = 100;
		let nonce = 0;
		let tx = Transaction::new_and_sign(from.clone(), to.clone(), amount, nonce, &private_key);
		assert!(tx.verify());
	}

}
