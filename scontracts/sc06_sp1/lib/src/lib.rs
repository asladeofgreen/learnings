use ed25519_consensus::{Signature as Ed25519Signature, VerificationKey as Ed25519VerificationKey};
use secp256k1::{
    ecdsa::{
        RecoverableSignature as Secp256k1RecoverableSignature, RecoveryId as Secp256k1RecoveryId,
    },
    Message as Secp256k1Message,
};

pub type DigestBytes = Vec<u8>;
pub type MessageBytes = Vec<u8>;
pub type SignatureBytes = [u8; 64];
pub type VerificationKeyBytes = [u8; 32];

#[sp1_derive::cycle_tracker]
pub fn verify_digest_blake2b(data: Vec<u8>, digest: DigestBytes) {
    assert_eq!(digest.len(), 32);
}

#[sp1_derive::cycle_tracker]
pub fn verify_signature_ed25519(msg: Vec<u8>, sig: Vec<u8>, vk: Vec<u8>) {
    assert_eq!(sig.len(), 64);
    assert_eq!(vk.len(), 32);

    let sig = Ed25519Signature::try_from(sig.as_slice()).unwrap();
    let vk = Ed25519VerificationKey::try_from(vk.as_slice()).unwrap();
    assert_eq!(vk.verify(&sig, &msg[..]), Ok(()));
}

#[sp1_derive::cycle_tracker]
pub fn verify_signature_secp256k1(msg: Vec<u8>, sig: Vec<u8>, vk: Vec<u8>) {
    assert_eq!(sig.len(), 64);
    assert_eq!(vk.len(), 33);

    let msg = msg[..32].try_into().unwrap();
    let msg = Secp256k1Message::from_digest(msg);
}
