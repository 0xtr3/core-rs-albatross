use std::{
    collections::VecDeque,
    error::Error,
    future,
    path::PathBuf,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::{future::BoxFuture, stream::BoxStream, FutureExt, Stream, StreamExt};
use nimiq_block::{Block, MacroBlock};
use nimiq_blockchain::Blockchain;
use nimiq_blockchain_interface::{AbstractBlockchain, BlockchainEvent, Direction};
use nimiq_genesis::NetworkInfo;
use nimiq_network_interface::network::Network;
use nimiq_primitives::policy::Policy;
use nimiq_zkp_primitives::state_commitment;
use parking_lot::{lock_api::RwLockUpgradableReadGuard, RwLock, RwLockWriteGuard};
use tokio::sync::oneshot::{channel, Sender};

use crate::{proof_gen_utils::*, types::*};

/// ZK Prover generates the zk proof for an election block. It has:
///
/// - The network
/// - The current zkp state
/// - The channel to kill the current process generating the proof
/// - The election blocks stream
/// - The genesis state
/// - The current proof generation future if a proof is being generated
/// - The path of the proving keys directory
/// - The path of the prover binary
///
/// The proofs are returned by polling the components.
pub struct ZKProver<N: Network> {
    network: Arc<N>,
    zkp_state: Arc<RwLock<ZKPState>>,
    sender: Option<Sender<()>>,
    pending_election_blocks: VecDeque<MacroBlock>,
    election_stream: BoxStream<'static, MacroBlock>,
    genesis_state: [u8; 95],
    proof_future:
        Option<BoxFuture<'static, Result<(ZKPState, MacroBlock), ZKProofGenerationError>>>,
    prover_keys_path: PathBuf,
    prover_path: Option<PathBuf>,
}

impl<N: Network> ZKProver<N> {
    pub async fn new(
        blockchain: Arc<RwLock<Blockchain>>,
        network: Arc<N>,
        zkp_state: Arc<RwLock<ZKPState>>,
        prover_path: Option<PathBuf>,
        prover_keys_path: PathBuf,
    ) -> Self {
        let network_info = NetworkInfo::from_network_id(blockchain.read().network_id());
        let genesis_block = network_info.genesis_block().unwrap_macro();

        let genesis_state = state_commitment(
            genesis_block.block_number(),
            &genesis_block.hash().into(),
            &genesis_block.pk_tree_root().unwrap(),
        );

        // Prepends the election blocks from the blockchain for which we don't have a proof yet
        let blockchain_rg = blockchain.read();
        let current_state_height = zkp_state.read().latest_block_number;
        let blockchain_election_height = blockchain_rg.state.election_head.block_number();

        let pending_election_blocks = if blockchain_election_height > current_state_height {
            blockchain_rg
                .get_macro_blocks(
                    &zkp_state.read().latest_header_hash,
                    (blockchain_election_height - current_state_height)
                        / Policy::blocks_per_epoch(),
                    true,
                    Direction::Forward,
                    true,
                    None,
                )
                .expect("Fetching election blocks for zkp prover initialization failed")
                .drain(..)
                .map(|block| block.unwrap_macro())
                .collect()
        } else {
            VecDeque::new()
        };

        // Gets the stream of blockchain events and converts it into an election macro block stream
        let blockchain_election_rx = blockchain_rg.notifier_as_stream();
        let blockchain2 = Arc::clone(&blockchain);

        let blockchain_election_rx = blockchain_election_rx.filter_map(move |event| {
            let result = match event {
                BlockchainEvent::EpochFinalized(hash) => {
                    let block = blockchain2.read().get_block(&hash, true, None);
                    if let Ok(Block::Macro(block)) = block {
                        Some(block)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            future::ready(result)
        });

        Self {
            network,
            zkp_state,
            sender: None,
            genesis_state,
            pending_election_blocks,
            election_stream: Box::pin(blockchain_election_rx),
            proof_future: None,
            prover_keys_path,
            prover_path,
        }
    }

    /// This sends the kill signal to the proof generation process.
    pub(crate) fn cancel_current_proof_production(&mut self) {
        if let Some(sender) = self.sender.take() {
            sender.send(()).unwrap();
        }
    }

    /// The broadcasting of the generated zk proof.
    fn broadcast_zk_proof(network: &Arc<N>, zk_proof: ZKProof) {
        let network = Arc::clone(network);
        tokio::spawn(async move {
            if let Err(e) = network.publish::<ZKProofTopic>(zk_proof).await {
                log::warn!(error = &e as &dyn Error, "Failed to publish the zk proof");
            }
        });
    }

    // Upon every election block, a proof generation process is launched.
    // The assertion holds true since we should only start generating the next proof after its predecessor's
    // proof has been pushed into our state.
    //
    // Note: The election block stream may have blocks that are too old relative to our zkp state;
    // thus we will filter those blocks out.
    fn launch_proof_generation(&mut self, block: MacroBlock) {
        let zkp_state = self.zkp_state.read();
        assert!(
                zkp_state.latest_block_number
                >= block.block_number() - Policy::blocks_per_epoch(),
                "The current state (block height: {}) should never lag behind more than one epoch. Current height: {}",
                zkp_state.latest_block_number,
                block.block_number(),
            );
        if zkp_state.latest_block_number == block.block_number() - Policy::blocks_per_epoch() {
            let (sender, recv) = channel();
            self.proof_future = Some(
                launch_generate_new_proof(
                    recv,
                    ProofInput {
                        block: block.clone(),
                        latest_pks: zkp_state.latest_pks.clone(),
                        latest_header_hash: zkp_state.latest_header_hash.clone(),
                        previous_proof: zkp_state.latest_proof.clone(),
                        genesis_state: self.genesis_state,
                        prover_keys_path: self.prover_keys_path.clone(),
                    },
                    self.prover_path.clone(),
                )
                .map(|res| res.map(|state| (state, block)))
                .boxed(),
            );
            self.sender = Some(sender);
        } else {
            log::debug!(
                block_height = zkp_state.latest_block_number,
                current_height = block.block_number(),
                "Won't generate zkp for a block of the past",
            );
        }
    }
}

impl<N: Network> Stream for ZKProver<N> {
    type Item = (ZKProof, MacroBlock);

    fn poll_next(mut self: Pin<&mut ZKProver<N>>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // Fetch and store the new election blocks.
        while let Poll::Ready(Some(block)) = self.election_stream.poll_next_unpin(cx) {
            self.pending_election_blocks.push_back(block);
        }

        // Launches new proof generation, if we have a pending election block are and no proof generation is launched yet.
        // We need to loop in case we already received a zkp for one of the blocks in the list.
        while self.proof_future.is_none() {
            if let Some(block) = self.pending_election_blocks.pop_front() {
                self.launch_proof_generation(block);
            } else {
                break;
            }
        }

        // If a new proof was generated it sets the state and broadcasts the new proof.
        if let Some(ref mut proof_future) = self.proof_future {
            if let Poll::Ready(proof) = proof_future.poll_unpin(cx) {
                self.proof_future = None;
                self.sender = None;
                match proof {
                    Ok((new_zkp_state, block)) => {
                        assert!(
                            new_zkp_state.latest_proof.is_some(),
                            "The generate new proof should never produces a empty proof"
                        );
                        let zkp_state_lock = self.zkp_state.upgradable_read();

                        // If we received a more recent proof in the meanwhile, we should have cancelled the proof generation process already.
                        assert!(
                            zkp_state_lock.latest_block_number < new_zkp_state.latest_block_number,
                            "The generated proof should always be more recent than the current state"
                        );

                        let mut zkp_state_lock = RwLockUpgradableReadGuard::upgrade(zkp_state_lock);
                        *zkp_state_lock = new_zkp_state;

                        let zkp_state_lock = RwLockWriteGuard::downgrade(zkp_state_lock);

                        let proof: ZKProof = zkp_state_lock.clone().into();
                        Self::broadcast_zk_proof(&self.network, proof.clone());
                        return Poll::Ready(Some((proof, block)));
                    }
                    Err(e) => {
                        log::error!(error = %e, "Error generating ZK Proof for block");
                    }
                };
            }
        }

        Poll::Pending
    }
}
