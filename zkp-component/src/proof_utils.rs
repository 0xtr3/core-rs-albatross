use ark_groth16::Proof;
use ark_mnt6_753::{G2Projective as G2MNT6, MNT6_753};
use nimiq_block::MacroBlock;
use nimiq_blockchain_interface::AbstractBlockchain;
use nimiq_blockchain_proxy::BlockchainProxy;
use nimiq_genesis::NetworkInfo;
use nimiq_zkp::{verify::verify, ZKP_VERIFYING_KEY};
use nimiq_zkp_primitives::NanoZKPError;

use super::types::ZKPState;
use crate::types::*;

/// Fully validates the proof by verifying both the zk proof and the blocks existence on the blockchain.
pub fn validate_proof(
    blockchain: &BlockchainProxy,
    proof: &ZKProof,
    election_block: Option<MacroBlock>,
) -> bool {
    // If it's a genesis block proof, then should have none as proof value to be valid.
    if proof.block_number == 0 && proof.proof.is_none() {
        return true;
    }

    // Fetches and verifies the election blocks for the proofs and then validates the proof
    if let Ok((new_block, genesis_block, proof)) =
        get_proof_macro_blocks(blockchain, proof, election_block)
    {
        return validate_proof_get_new_state(proof, &new_block, genesis_block).is_ok();
    }
    false
}

/// Fetches both the genesis and the proof block. Fails if the proof block is not an election block or the proof is empty.
/// If the given option already contains the new election block, we return it. Otherwise, we retrieve it from the blockchain.
pub(crate) fn get_proof_macro_blocks(
    blockchain: &BlockchainProxy,
    proof: &ZKProof,
    election_block: Option<MacroBlock>,
) -> Result<(MacroBlock, MacroBlock, Proof<MNT6_753>), Error> {
    let block_number = proof.block_number;
    let proof = proof.proof.clone().ok_or(NanoZKPError::EmptyProof)?;

    // Gets the block of the new proof
    let new_block = if let Some(block) = election_block {
        block
    } else {
        let new_block = blockchain
            .read()
            .get_block_at(block_number, true)
            .map_err(|_| Error::InvalidBlock)?;

        if !new_block.is_election() {
            return Err(Error::InvalidBlock);
        }
        new_block.unwrap_macro()
    };

    // Gets genesis block
    let network_info = NetworkInfo::from_network_id(blockchain.read().network_id());
    let genesis_block = network_info.genesis_block().unwrap_macro();

    Ok((new_block, genesis_block, proof))
}

/// Validates proof and returns the new zkp state. Assumes the blocks provided are valid.
pub(crate) fn validate_proof_get_new_state(
    proof: Proof<MNT6_753>,
    new_block: &MacroBlock,
    genesis_block: MacroBlock,
) -> Result<ZKPState, Error> {
    let new_pks: Vec<G2MNT6> = new_block
        .get_validators()
        .ok_or(Error::InvalidBlock)?
        .voting_keys()
        .into_iter()
        .map(|pub_key| pub_key.public_key)
        .collect();

    if verify(
        genesis_block.block_number(),
        genesis_block.hash().into(),
        new_block.block_number(),
        new_block.hash().into(),
        proof.clone(),
        &ZKP_VERIFYING_KEY,
    )? {
        return Ok(ZKPState {
            latest_pks: new_pks,
            latest_header_hash: new_block.hash_blake2s(),
            latest_block_number: new_block.block_number(),
            latest_proof: Some(proof),
        });
    }
    Err(Error::InvalidProof)
}
