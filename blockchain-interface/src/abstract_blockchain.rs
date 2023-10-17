use futures::stream::BoxStream;
use nimiq_block::{Block, BlockType, MacroBlock};
use nimiq_hash::Blake2bHash;
use nimiq_primitives::{
    networks::NetworkId,
    policy::Policy,
    slots_allocation::{Validator, Validators},
};

use crate::{
    error::{BlockchainError, BlockchainEvent, Direction},
    ChainInfo, ForkEvent,
};

pub struct TaintedBlockchainConfig {
    // Produce blocks even when is not our turn.
    pub always_produce: bool,
    // Fork blocks (produce two different blocks at the same height)
    pub fork_blocks: bool,
    // Produce invalid blocks
    pub invalid_blocks: bool,
    // Tainted voting key
    pub tainted_voting_key: bool,
    // Tainted signing key
    pub tainted_signing_key: bool,
    // Tainted request macro chain
    pub tainted_request_macro_chain: bool,
    // Tainted request batch set
    pub tainted_request_batch_set: bool,
    // Tainted request history chunk
    pub tainted_request_history_chunk: bool,
    // Tainted request block
    pub tainted_request_block: bool,
    // Tainted request block
    pub tainted_request_missing_blocks: bool,
    // Tainted request head
    pub tainted_request_head: bool,
}

impl Default for TaintedBlockchainConfig {
    fn default() -> Self {
        Self {
            always_produce: false,
            fork_blocks: false,
            invalid_blocks: false,
            tainted_voting_key: false,
            tainted_signing_key: false,
            tainted_request_macro_chain: false,
            tainted_request_batch_set: false,
            tainted_request_history_chunk: false,
            tainted_request_block: false,
            tainted_request_missing_blocks: false,
            tainted_request_head: false,
        }
    }
}

/// Defines several basic methods for blockchains.
pub trait AbstractBlockchain {
    /// Returns the network id.
    fn network_id(&self) -> NetworkId;

    fn get_tainted_config(&self) -> &TaintedBlockchainConfig;

    /// Returns the current time.
    fn now(&self) -> u64;

    /// Returns the head of the main chain.
    fn head(&self) -> Block;

    /// Returns the last macro block.
    fn macro_head(&self) -> MacroBlock;

    /// Returns the last election macro block.
    fn election_head(&self) -> MacroBlock;

    /// Returns the hash of the head of the main chain.
    fn head_hash(&self) -> Blake2bHash {
        self.head().hash()
    }

    /// Returns the hash of the last macro block.
    fn macro_head_hash(&self) -> Blake2bHash {
        self.macro_head().hash()
    }

    /// Returns the hash of the last election macro block.
    fn election_head_hash(&self) -> Blake2bHash {
        self.election_head().hash()
    }

    /// Returns the block number at the head of the main chain.
    fn block_number(&self) -> u32 {
        self.head().block_number()
    }

    /// Returns the epoch number at the head of the main chain.
    fn epoch_number(&self) -> u32 {
        self.head().epoch_number()
    }

    /// Returns the timestamp at the head of the main chain.
    fn timestamp(&self) -> u64 {
        self.head().timestamp()
    }

    /// Returns the block type of the next block.
    // FIXME Get rid of this
    fn get_next_block_type(last_block_number: u32) -> BlockType {
        if Policy::is_macro_block_at(last_block_number + 1) {
            BlockType::Macro
        } else {
            BlockType::Micro
        }
    }

    /// Returns a flag indicating if the accounts tree is complete.
    fn accounts_complete(&self) -> bool;

    /// Returns the current set of validators.
    fn current_validators(&self) -> Option<Validators>;

    /// Returns the set of validators of the previous epoch.
    fn previous_validators(&self) -> Option<Validators>;

    /// Checks if the blockchain contains a specific block, by its hash.
    fn contains(&self, hash: &Blake2bHash, include_forks: bool) -> bool;

    /// Fetches a given block, by its block number.
    fn get_block_at(&self, height: u32, include_body: bool) -> Result<Block, BlockchainError>;

    /// Fetches a given block, by its hash.
    fn get_block(&self, hash: &Blake2bHash, include_body: bool) -> Result<Block, BlockchainError>;

    /// Get several blocks.
    fn get_blocks(
        &self,
        start_block_hash: &Blake2bHash,
        count: u32,
        include_body: bool,
        direction: Direction,
    ) -> Result<Vec<Block>, BlockchainError>;

    /// Fetches a given chain info, by its hash.
    fn get_chain_info(
        &self,
        hash: &Blake2bHash,
        include_body: bool,
    ) -> Result<ChainInfo, BlockchainError>;

    /// Calculates the slot owner (represented as the validator plus the slot number) at a given
    /// block number and offset.
    fn get_slot_owner_at(
        &self,
        block_number: u32,
        offset: u32,
    ) -> Result<(Validator, u16), BlockchainError>;

    /// Fetches a given number of macro blocks, starting at a specific block (by its hash).
    /// It can fetch only election macro blocks if desired.
    fn get_macro_blocks(
        &self,
        start_block_hash: &Blake2bHash,
        count: u32,
        include_body: bool,
        direction: Direction,
        election_blocks_only: bool,
    ) -> Result<Vec<Block>, BlockchainError>;

    /// Stream of Blockchain Events.
    // FIXME Naming
    fn notifier_as_stream(&self) -> BoxStream<'static, BlockchainEvent>;

    /// Stream of Fork Events.
    // FIXME Get rid of this
    fn fork_notifier_as_stream(&self) -> BoxStream<'static, ForkEvent>;
}
