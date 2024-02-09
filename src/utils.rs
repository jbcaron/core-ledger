use secp256k1::hashes::{sha256, Hash};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};

pub fn hash(data: &[u8]) -> Vec<u8> {
    sha256::Hash::hash(data).as_byte_array().to_vec()
}

pub fn sign_hash(hash: &[u8], private_key: &[u8]) -> Vec<u8> {
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(hash).expect("Invalid message");
    let secret_key = SecretKey::from_slice(private_key).expect("Invalid private key");
    secp.sign_ecdsa(&message, &secret_key)
        .serialize_compact()
        .to_vec()
}

pub fn verify_signature(public_key: &[u8], hash: &[u8], signature: &[u8]) -> bool {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_slice(public_key).expect("Invalid public key");
    let message = Message::from_digest_slice(hash).expect("Invalid message");
    let signature = Signature::from_compact(signature).expect("Invalid signature");
    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

/// Generate a new keypair for the secp256k1 curve
/// Returns a tuple (secret_key, public_key)
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    (
        secret_key.as_ref().to_vec(),
        public_key.serialize().to_vec(),
    )
}

pub fn is_valid_public_key(public_key: &[u8]) -> bool {
	PublicKey::from_slice(public_key).is_ok()
}
