use secp256k1::hashes::{sha256, Hash as Hash_lib};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{
    ecdsa::Signature as Signature_lib, Message, PublicKey as PublicKey_lib, Secp256k1, SecretKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash([u8; 32]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivateKey([u8; 32]);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicKey([u8; 33]);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature([u8; 64]);

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for Hash {
    fn from(data: &[u8]) -> Self {
        let hash_bytes = sha256::Hash::hash(data).as_byte_array().clone();
        Hash(hash_bytes)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash([0; 32])
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0; 64])
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(data: &PrivateKey) -> Self {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(data.0.as_slice()).unwrap();
        let public_key = PublicKey_lib::from_secret_key(&secp, &secret_key);
        PublicKey(public_key.serialize())
    }
}

pub fn sign_hash(hash: &Hash, private_key: &PrivateKey) -> Result<Signature, String> {
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(hash.0.as_slice()).map_err(|e| e.to_string())?;
    let secret_key = SecretKey::from_slice(private_key.0.as_slice()).map_err(|e| e.to_string())?;
    let signature = secp.sign_ecdsa(&message, &secret_key).serialize_compact();
    Ok(Signature(signature))
}

pub fn verify_signature(
    public_key: &PublicKey,
    hash: &Hash,
    signature: &Signature,
) -> Result<(), String> {
    let secp = Secp256k1::new();
    let public_key =
        PublicKey_lib::from_slice(public_key.0.as_slice()).map_err(|e| e.to_string())?;
    let message = Message::from_digest_slice(hash.0.as_slice()).map_err(|e| e.to_string())?;
    let signature =
        Signature_lib::from_compact(signature.0.as_slice()).map_err(|e| e.to_string())?;
    secp.verify_ecdsa(&message, &signature, &public_key)
        .map_err(|e| e.to_string())
}

/// Generate a new keypair for the secp256k1 curve
/// Returns a tuple (secret_key, public_key)
pub fn generate_keypair() -> (PrivateKey, PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    (
        PrivateKey(secret_key.secret_bytes()),
        PublicKey(public_key.serialize()),
    )
}

pub fn generate_keypair_from_secret(secret: &[u8]) -> (PrivateKey, PublicKey) {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_hashed_data::<sha256::Hash>(secret);
    let public_key = PublicKey_lib::from_secret_key(&secp, &secret_key);
    (
        PrivateKey(secret_key.secret_bytes()),
        PublicKey(public_key.serialize()),
    )
}

#[cfg(test)]
mod tests {
    use std::hash;

    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (private_key, public_key) = generate_keypair();
        assert_eq!(private_key.0.len(), 32);
        assert_eq!(public_key.0.len(), 33);
    }

    #[test]
    fn test_sign_and_verify() {
        let (private_key, public_key) = generate_keypair();
        let hash = Hash::from("hello world".as_bytes());
        let signature = sign_hash(&hash, &private_key).unwrap();
        verify_signature(&public_key, &hash, &signature).unwrap();
    }

    #[test]
    fn test_generate_keypair_from_secret() {
        let secret = b"hello world";
        let (private_key, public_key) = generate_keypair_from_secret(secret);
        assert_eq!(private_key.0.len(), 32);
        assert_eq!(public_key.0.len(), 33);
        let hash = Hash::from("Hello world".as_bytes());
        let signature = sign_hash(&hash, &private_key).unwrap();
        verify_signature(&public_key, &hash, &signature).unwrap();
    }

    #[test]
    fn test_generate_keypair_from_same_secret() {
        let secret = b"secret";
        let (private_key1, public_key1) = generate_keypair_from_secret(secret);
        let (private_key2, public_key2) = generate_keypair_from_secret(secret);
        assert_eq!(private_key1, private_key2);
        assert_eq!(public_key1, public_key2);
    }
}
