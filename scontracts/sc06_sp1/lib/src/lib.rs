use ed25519_consensus::{Signature as Ed25519Signature, VerificationKey as Ed25519VerificationKey};

pub type MessageBytes = Vec<u8>;
pub type SignatureBytes = [u8; 64];
pub type VerificationKeyBytes = [u8; 32];

pub fn is_valid_signature_ed25519_1(msg: Vec<u8>, sig: Vec<u8>, vk: Vec<u8>) -> bool {
    println!("BEGIN :: cycle-tracker :: is_valid_signature_ed25519");

    let sig = Ed25519Signature::try_from(sig.as_slice()).unwrap();
    let vk = Ed25519VerificationKey::try_from(vk.as_slice()).unwrap();
    assert_eq!(vk.verify(&sig, &msg[..]), Ok(()));

    println!("END :: cycle-tracker :: is_valid_signature_ed25519");

    true
}
