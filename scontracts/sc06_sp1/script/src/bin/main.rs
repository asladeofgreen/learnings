use clap::Parser;
use hex;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

const MSG_ED25519: &str = "656432353531392d636f6e73656e7375732074657374206d657373616765";
const SIG_ED25519: &str = "69261ea5df799b20fc6eeb49aa79f572c8f1e2ba88b37dff184cc55d4e3653d876419bffcc47e5343cdd5fd78121bb32f1c377a5ed505106ad37f19980218f0d";
const VK_ED25519: &str = "9194c3ead03f5848111db696fe1196fbbeffc69342d51c7cf5e91c502de91eb4";

const MSG_SECP256K1: &str = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎";
const SIG_SECP256K1: &str = "7b32f21b5861de8faa1c41205e6c17c6ff1167f59fe701a143cd588f1ef4833741a901c2539a7f755a7164458b1ba4acbf1ef209ce84533567a00ccfb13e3625";
const VK_SECP256K1: &str = "0272dcc1d384a6ddad06fde1ceb2f1fe524f84ddf5ee3bdb3682eb7b927de0e682";

const DIGEST_BLAKE2B: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const MSG_BLAKE2B: &str = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎";

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

    let mut stdin = SP1Stdin::new();
    stdin.write_vec(hex::decode(MSG_ED25519).unwrap());
    stdin.write_vec(hex::decode(SIG_ED25519).unwrap());
    stdin.write_vec(hex::decode(VK_ED25519).unwrap());
    stdin.write_vec(MSG_SECP256K1.as_bytes().to_vec());
    stdin.write_vec(hex::decode(SIG_SECP256K1).unwrap());
    stdin.write_vec(hex::decode(VK_SECP256K1).unwrap());
    stdin.write_vec(MSG_BLAKE2B.as_bytes().to_vec());
    stdin.write_vec(hex::decode(DIGEST_BLAKE2B).unwrap());

    if args.execute {
        do_execute(stdin);
    } else {
        do_prove(stdin);
    }
}

fn do_execute(stdin: SP1Stdin) {
    let client = ProverClient::new();
    let (output, report) = client.execute(_ELF, stdin).run().unwrap();

    // Process output.
    // TODO

    // Process report.
    println!("Number of cycles: {}", report.total_instruction_count());
    println!("Number of sys calls: {}", report.total_syscall_count());
}

fn do_prove(stdin: SP1Stdin) {
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
