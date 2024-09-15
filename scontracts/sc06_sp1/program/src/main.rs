#![no_main]
sp1_zkvm::entrypoint!(main);

use sc06_lib::{verify_digest_blake2b, verify_signature_ed25519, verify_signature_secp256k1};

pub fn main() {
    verify_signature_ed25519(
        sp1_zkvm::io::read_vec(),
        sp1_zkvm::io::read_vec(),
        sp1_zkvm::io::read_vec(),
    );
    verify_signature_secp256k1(
        sp1_zkvm::io::read_vec(),
        sp1_zkvm::io::read_vec(),
        sp1_zkvm::io::read_vec(),
    );
    verify_digest_blake2b(sp1_zkvm::io::read_vec(), sp1_zkvm::io::read_vec());
}
