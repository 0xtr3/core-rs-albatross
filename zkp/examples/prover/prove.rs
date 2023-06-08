use std::{
    fs::{DirBuilder, File},
    io,
    path::{Path, PathBuf},
    time::Instant,
};

use ark_groth16::Proof;
use ark_serialize::CanonicalSerialize;
use log::metadata::LevelFilter;
use nimiq_log::TargetsExt;
use nimiq_primitives::policy::Policy;
use nimiq_test_utils::{
    block_production::TemporaryBlockProducer, blockchain_with_rng::produce_macro_blocks_with_rng,
    test_rng::test_rng,
};
use nimiq_zkp::prove::prove;
use tracing_subscriber::{filter::Targets, prelude::*};

fn initialize() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(io::stderr))
        .with(
            Targets::new()
                .with_default(LevelFilter::INFO)
                .with_nimiq_targets(LevelFilter::DEBUG)
                .with_target("r1cs", LevelFilter::WARN)
                .with_env(),
        )
        .init();
}

/// Generates a proof for a chain of election blocks. The random parameters generation uses always
/// the same seed, so it will always generate the same data (validators, signatures, etc).
/// This function will simply output a proof for the final epoch and store it in file.
/// Run this example with `cargo run --all-features --release --example prove`.
fn main() {
    initialize();
    // Ask user for the number of epochs.
    println!("Enter the number of epochs to prove:");

    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Couldn't read user input.");

    let number_epochs: u32 = data.trim().parse().expect("Couldn't read user input.");

    println!("====== Proof generation for Nano Sync initiated ======");

    let start = Instant::now();

    let mut genesis_header_hash = [0; 32];
    let mut genesis_data = None;
    let mut proof = Proof::default();

    let block_producer = TemporaryBlockProducer::new();
    produce_macro_blocks_with_rng(
        &block_producer.producer,
        &block_producer.blockchain,
        number_epochs as usize * Policy::batches_per_epoch() as usize,
        &mut test_rng(true),
    );

    for i in 0..number_epochs {
        // Get random parameters.
        let blockchain_rg = block_producer.blockchain.read();
        let prev_block = blockchain_rg
            .get_block_at(i * Policy::blocks_per_epoch(), true, None)
            .unwrap()
            .unwrap_macro();
        let final_block = blockchain_rg
            .get_block_at((i + 1) * Policy::blocks_per_epoch(), true, None)
            .unwrap()
            .unwrap_macro();

        log::error!("Block 1 {:?}", prev_block);
        log::error!("Block 2 {:?}", final_block);

        // Create genesis data.
        if i == 0 {
            genesis_header_hash = prev_block.hash_blake2s().0;
        } else {
            genesis_data = Some((proof, genesis_header_hash.clone()))
        }

        println!("Proving epoch {}", i + 1);

        // Generate proof.
        proof = prove(
            prev_block,
            final_block,
            genesis_data.clone(),
            true,
            true,
            &PathBuf::new(), // use the current directory
        )
        .unwrap();

        // Save proof to file.
        if !Path::new("proofs/").is_dir() {
            DirBuilder::new().create("proofs/").unwrap();
        }

        let mut file = File::create(format!("proofs/proof_epoch_{}.bin", i + 1)).unwrap();

        proof.serialize_uncompressed(&mut file).unwrap();

        file.sync_all().unwrap();
    }

    println!("====== Proof generation for Nano Sync finished ======");
    println!("Total time elapsed: {:?}", start.elapsed());
}
