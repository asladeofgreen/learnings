use clap::Parser;
use hex;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

const MSG_ED25519_1: &str = "656432353531392d636f6e73656e7375732074657374206d657373616765";
const SIG_ED25519_1: &str = "69261ea5df799b20fc6eeb49aa79f572c8f1e2ba88b37dff184cc55d4e3653d876419bffcc47e5343cdd5fd78121bb32f1c377a5ed505106ad37f19980218f0d";
const VK_ED25519_1: &str = "9194c3ead03f5848111db696fe1196fbbeffc69342d51c7cf5e91c502de91eb4";

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Set logging.
    sp1_sdk::utils::setup_logger();

    // Parse args.
    let args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    if args.execute {
        do_execute(args);
    } else {
        do_prove(args);
    }
}

fn do_execute(args: Args) {
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(hex::decode(MSG_ED25519_1).unwrap());
    stdin.write_vec(hex::decode(SIG_ED25519_1).unwrap());
    stdin.write_vec(hex::decode(VK_ED25519_1).unwrap());

    let client = ProverClient::new();

    let (output, report) = client.execute(_ELF, stdin).run().unwrap();
    println!("Program executed successfully.");

    // Process output.
    // TODO

    // Process report.
    println!("Number of cycles: {}", report.total_instruction_count());
}

fn do_prove(args: Args) {
    let mut stdin = SP1Stdin::new();
    let client = ProverClient::new();

    let (pk, vk) = client.setup(_ELF);

    // Set proof.
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");
    println!("Successfully generated proof!");

    // Verify proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}
