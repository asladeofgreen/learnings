use clap::Parser;
use hex;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

const MSG_ED25519: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const PBK_ED25519: &str = "764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
const PVK_ED25519: &str = "e541eed0cc0fa324fa8f34ab6faa186d98826777344c5518819fed5e20448282";
const SIG_ED25519: &str = "2fa8e929a7514496545d098e86841463ef66358ff0930073fde3b138f66a2cef5304d884baa693a971d002d7e071f658fb16de8c1e5c80ba5ecea8b3866f8106";

// const MSG_SECP256K1: &str = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎";
// const SIG_SECP256K1: &str = "7b32f21b5861de8faa1c41205e6c17c6ff1167f59fe701a143cd588f1ef4833741a901c2539a7f755a7164458b1ba4acbf1ef209ce84533567a00ccfb13e3625";
// const VK_SECP256K1: &str = "0272dcc1d384a6ddad06fde1ceb2f1fe524f84ddf5ee3bdb3682eb7b927de0e682";
const MSG_SECP256K1: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const PBK_SECP256K1: &str = "03eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
const PVK_SECP256K1: &str = "0b0e6094077ebae48bc119b35a134890424e0e5dbb71e667edae1978c49e7089";
const SIG_SECP256K1: &str = "ba07224f974d973682d93dca27517b2bbbacca42df551ef22be3f80c65bc4923bad512c31a85f996345be08a8a2557f814cefc04c3e8a8ddb58c3749a65d4ce4";

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
    stdin.write_vec(hex::decode(PBK_ED25519).unwrap());
    stdin.write_vec(hex::decode(MSG_SECP256K1).unwrap());
    stdin.write_vec(hex::decode(SIG_SECP256K1).unwrap());
    stdin.write_vec(hex::decode(PBK_SECP256K1).unwrap());
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
