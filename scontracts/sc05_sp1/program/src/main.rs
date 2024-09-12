// Set ZK-VM entrypoint.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sc05_lib::{hypotoneuse, ComputeHyptoneusePublicParams};

pub fn main() {
    // Set inputs.
    let x = sp1_zkvm::io::read::<u8>();
    let y = sp1_zkvm::io::read::<u32>();

    // Set function result.
    let z = hypotoneuse(x, y);

    // Set bytes of program public values.
    let bytes =
        ComputeHyptoneusePublicParams::abi_encode(&ComputeHyptoneusePublicParams { x, y, z });

    // Commit to public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
