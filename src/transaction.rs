use crate::crypto::{sign_hash, verify_signature, Hash, PrivateKey, PublicKey, Signature};

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    from: PublicKey,
    to: PublicKey,
    amount: u64,
    nonce: u64,
    signature: Signature,
}

impl Transaction {
    pub fn new(from: &PublicKey, to: &PublicKey, amount: u64, nonce: u64) -> Transaction {
        Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
            nonce,
            signature: Signature::default(),
        }
    }

    pub fn new_signed(
        from: &PublicKey,
        to: &PublicKey,
        amount: u64,
        nonce: u64,
        signature: &Signature,
    ) -> Result<Transaction, String> {
        let tx = Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
            nonce,
            signature: signature.clone(),
        };
        tx.verify()?;
        Ok(tx)
    }

    pub fn new_and_sign(
        from: &PublicKey,
        to: &PublicKey,
        amount: u64,
        nonce: u64,
        private_key: &PrivateKey,
    ) -> Result<Transaction, String> {
        let mut tx = Transaction::new(from, to, amount, nonce);
        tx.sign(private_key)?;
        Ok(tx)
    }

    pub fn from(&self) -> PublicKey {
        self.from.clone()
    }

    pub fn to(&self) -> PublicKey {
        self.to.clone()
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }

    pub fn hash(&self) -> Hash {
        let data = [
            self.from.as_ref(),
            self.to.as_ref(),
            &self.amount.to_be_bytes(),
            &self.nonce.to_be_bytes(),
        ];
        Hash::from(data.concat().as_ref())
    }

    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<(), String> {
        let hash = self.hash();
        self.signature = sign_hash(&hash, private_key)?;
        Ok(())
    }

    pub fn verify(&self) -> Result<(), String> {
        let hash = self.hash();
        verify_signature(&self.from, &hash, &self.signature)
    }
}
