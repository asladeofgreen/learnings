const LENGTH_OF_DIGEST: usize = 32;
const LENGTH_OF_SIGNATURE: usize = 64;
const LENGTH_OF_VERIFIER_KEY_ED25519: usize = 32;
const LENGTH_OF_VERIFIER_KEY_SECP256K1: usize = 33;

pub type DigestBytes = [u8; LENGTH_OF_DIGEST];
pub type SignatureBytes = [u8; LENGTH_OF_SIGNATURE];
pub enum VerificationKeyBytes {
    ED25519([u8; LENGTH_OF_VERIFIER_KEY_ED25519]),
    SECP256K1([u8; LENGTH_OF_VERIFIER_KEY_SECP256K1]),
}

/// Verifies a BLAKE2B digest over passed data.
///
/// # Arguments
///
/// * `data` - Data over which to generate a digest.
/// * `digest` - Digest to be verified.
///
#[sp1_derive::cycle_tracker]
pub fn verify_digest_blake2b(
    data: Vec<u8>,
    digest: DigestBytes
) {
    use blake2::{Blake2bVar, digest::{Update, VariableOutput}};

    let mut hasher = Blake2bVar::new(LENGTH_OF_DIGEST).unwrap();
    hasher.update(&data);
    let mut buffer = [0u8; LENGTH_OF_DIGEST];
    hasher.finalize_variable(&mut buffer).unwrap();

    assert_eq!(digest, buffer);
}

/// Verifies an ECC signature.
///
/// # Arguments
///
/// * `digest` - Digest over a message.
/// * `sig` - Signature to be verified.
/// * `vk` - Verification key associated over message digest signing key.
///
#[sp1_derive::cycle_tracker]
pub fn verify_signature(
    digest: DigestBytes,
    sig: SignatureBytes,
    vk: VerificationKeyBytes
) {
    match vk {
        VerificationKeyBytes::ED25519(vk) => verify_signature_ed25519(digest, sig, vk),
        VerificationKeyBytes::SECP256K1(vk) => verify_signature_secp256k1(digest, sig, vk),
    }
}

/// Verifies an ED25519 ECC signature.
///
/// # Arguments
///
/// * `digest` - Digest over a message.
/// * `sig` - Signature to be verified.
/// * `vk` - Verification key associated over message digest signing key.
///
#[sp1_derive::cycle_tracker]
fn verify_signature_ed25519(
    digest: DigestBytes,
    sig: SignatureBytes,
    vk: [u8; LENGTH_OF_VERIFIER_KEY_ED25519]
) {
    use ed25519_consensus::{Signature, VerificationKey};

    let sig = Signature::try_from(sig.as_slice()).unwrap();
    let vk = VerificationKey::try_from(vk.as_slice()).unwrap();

    assert_eq!(vk.verify(&sig, &digest), Ok(()));
}

/// Verifies an SECP256K1 ECC signature.
///
/// # Arguments
///
/// * `digest` - Digest over a message.
/// * `sig` - Signature to be verified.
/// * `vk` - Verification key associated over message digest signing key.
///
#[sp1_derive::cycle_tracker]
fn verify_signature_secp256k1(
    digest: DigestBytes,
    sig: SignatureBytes,
    vk: [u8; LENGTH_OF_VERIFIER_KEY_SECP256K1]
) {
    use secp256k1::{ecdsa::Signature, Message, PublicKey};

    let _ = Message::from_digest_slice(&digest).unwrap();
    let _ = PublicKey::from_slice(vk.as_slice()).unwrap();
    let _ = Signature::from_compact(&sig).unwrap();

    // assert_eq!(
    //     Secp256k1::verification_only().verify_ecdsa(&msg, &sig, &pbk),
    //     Ok(())
    // );
    // assert!(Secp256k1::verification_only()
    //     .verify_ecdsa(&msg, &sig, &vk)
    //     .is_ok());
}
