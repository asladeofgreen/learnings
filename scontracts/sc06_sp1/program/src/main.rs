//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sc06_lib::{
    is_valid_signature_ed25519_1,
    // MessageBytes,
    // SignatureBytes,
    // VerificationKeyBytes
};

pub fn main() {
    // Set inputs.
    let msg = sp1_zkvm::io::read_vec();
    let sig = sp1_zkvm::io::read_vec();
    let vk = sp1_zkvm::io::read_vec();

    println!("111 msg :: {:?}", &msg);
    println!("111 sig :: {:?}", &sig);
    println!("111 vk  :: {:?}", &vk);

    // Invoke business logic.
    let is_valid = is_valid_signature_ed25519_1(msg, sig, vk);

    // Set outputs.
    sp1_zkvm::io::commit(&is_valid);
}
