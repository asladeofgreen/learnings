#![no_main]
sp1_zkvm::entrypoint!(main);

use sc06_lib::{DigestBytes, verify_digest, VerificationKeyBytes, verify_signature};

pub fn main() {
    verify_signature(
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        VerificationKeyBytes::ED25519(
            sp1_zkvm::io::read_vec().try_into().unwrap()
        ),
    );
    verify_signature(
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        VerificationKeyBytes::SECP256K1(
            sp1_zkvm::io::read_vec().try_into().unwrap()
        ),
    );
    verify_digest(
        sp1_zkvm::io::read_vec(),
        DigestBytes::BLAKE2B(
            sp1_zkvm::io::read_vec().try_into().unwrap()
        ),
    );
}
