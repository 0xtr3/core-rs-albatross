use std::cmp;

use nimiq_keys::Address;
use once_cell::sync::OnceCell;

/// Global policy
static GLOBAL_POLICY: OnceCell<Policy> = OnceCell::new();

#[derive(Clone, Copy)]
pub struct Policy {
    /// Length of a batch including the macro block
    pub blocks_per_batch: u32,
    /// How many batches constitute an epoch
    pub batches_per_epoch: u16,
    /// Tendermint's initial timeout, in milliseconds.
    ///
    /// See <https://arxiv.org/abs/1807.04938v3> for more information.
    pub tendermint_timeout_init: u64,
    /// Tendermint's timeout delta, in milliseconds.
    ///
    /// See <https://arxiv.org/abs/1807.04938v3> for more information.
    pub tendermint_timeout_delta: u64,
    /// Maximum size of accounts trie chunks.
    pub state_chunks_max_size: u32,
    /// Number of blocks a transaction is valid with Albatross consensus.
    /// This should be a multiple of `blocks_per_batch`.
    pub transaction_validity_window: u32,
    /// Genesis block number
    pub genesis_block_number: u32,
}

impl Policy {
    /// This is the address for the staking contract. Corresponds to
    /// 'NQ77 0000 0000 0000 0000 0000 0000 0000 0001'
    pub const STAKING_CONTRACT_ADDRESS: Address = Address([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01,
    ]);

    /// This is the address for the coinbase. Note that this is not a real account, it is just the
    /// address we use to denote that some coins originated from a coinbase event. Corresponds to
    /// 'NQ81 C01N BASE 0000 0000 0000 0000 0000 0000'
    pub const COINBASE_ADDRESS: Address = Address([
        0x60, 0x03, 0x65, 0xab, 0x4e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    /// The maximum allowed size, in bytes, for a micro block body.
    pub const MAX_SIZE_MICRO_BODY: usize = 100_000;

    /// The current version number of the protocol. Changing this always results in a hard fork.
    pub const VERSION: u16 = 1;

    /// Number of available validator slots. Note that a single validator may own several validator slots.
    pub const SLOTS: u16 = 512;

    /// Calculates 2f+1 slots which is the minimum number of slots necessary to produce a macro block,
    /// a skip block and other actions.
    /// It is also the minimum number of slots necessary to be guaranteed to have a majority of honest
    /// slots. That's because from a total of 3f+1 slots at most f will be malicious. If in a group of
    /// 2f+1 slots we have f malicious ones (which is the worst case scenario), that still leaves us
    /// with f+1 honest slots. Which is more than the f slots that are not in this group (which must all
    /// be honest).
    /// It is calculated as `ceil(SLOTS*2/3)` and we use the formula `ceil(x/y) = (x+y-1)/y` for the
    /// ceiling division.
    pub const TWO_F_PLUS_ONE: u16 = (2 * Self::SLOTS + 3 - 1) / 3;

    /// Calculates f+1 slots which is the minimum number of slots necessary to be guaranteed to have at
    /// least one honest slots. That's because from a total of 3f+1 slots at most f will be malicious.
    /// It is calculated as `ceil(SLOTS/3)` and we use the formula `ceil(x/y) = (x+y-1)/y` for the
    /// ceiling division.
    pub const F_PLUS_ONE: u16 = (Self::SLOTS + 3 - 1) / 3;

    /// The timeout in milliseconds for a validator to produce a block (2s)
    pub const BLOCK_PRODUCER_TIMEOUT: u64 = 2 * 1000;

    /// The optimal time in milliseconds between blocks (1s)
    pub const BLOCK_SEPARATION_TIME: u64 = 1000;

    /// Minimum number of epochs that the ChainStore will store fully
    pub const MIN_EPOCHS_STORED: u32 = 1;

    /// The maximum drift, in milliseconds, that is allowed between any block's timestamp and the node's
    /// system time. We only care about drifting to the future.
    pub const TIMESTAMP_MAX_DRIFT: u64 = 600000;

    /// The slope of the exponential decay used to punish validators for not producing block in time
    pub const BLOCKS_DELAY_DECAY: f64 = 1.1e-9;

    /// The minimum rewards percentage that we allow
    pub const MINIMUM_REWARDS_PERCENTAGE: f64 = 0.5;

    /// The deposit necessary to create a validator in Lunas (1 NIM = 100,000 Lunas).
    /// A validator is someone who actually participates in block production. They are akin to miners
    /// in proof-of-work.
    pub const VALIDATOR_DEPOSIT: u64 = 1_000_000_000;

    /// The number of epochs a validator is put in jail for. The jailing only happens for severe offenses.
    pub const JAIL_EPOCHS: u32 = 8;

    /// Total supply in units.
    pub const TOTAL_SUPPLY: u64 = 2_100_000_000_000_000;

    /// This is the number of Lunas (1 NIM = 100,000 Lunas) created by millisecond at the genesis of the
    /// Nimiq 2.0 chain. The velocity then decreases following the formula:
    /// Supply_velocity (t) = Initial_supply_velocity * e^(- Supply_decay * t)
    /// Where e is the exponential function and t is the time in milliseconds since the genesis block.
    pub const INITIAL_SUPPLY_VELOCITY: f64 = 875.0;

    /// The supply decay is a constant that is calculated so that the supply velocity decreases at a
    /// steady 1.47% per year.
    pub const SUPPLY_DECAY: f64 = 4.692821935e-13;

    /// The maximum size of the BLS public key cache.
    pub const BLS_CACHE_MAX_CAPACITY: usize = 1000;

    /// Maximum size of history chunks.
    /// 25 MB.
    pub const HISTORY_CHUNKS_MAX_SIZE: u64 = 25 * 1024 * 1024;

    #[inline]
    fn get_blocks_per_epoch(&self) -> u32 {
        self.blocks_per_batch * self.batches_per_epoch as u32
    }

    #[inline]
    fn get_genesis_block_number(&self) -> u32 {
        self.genesis_block_number
    }

    #[inline]
    pub fn transaction_validity_window() -> u32 {
        GLOBAL_POLICY
            .get_or_init(Self::default)
            .transaction_validity_window
    }

    #[inline]
    pub fn batches_per_epoch() -> u16 {
        GLOBAL_POLICY.get_or_init(Self::default).batches_per_epoch
    }

    #[inline]
    pub fn blocks_per_batch() -> u32 {
        GLOBAL_POLICY.get_or_init(Self::default).blocks_per_batch
    }

    #[inline]
    pub fn blocks_per_epoch() -> u32 {
        GLOBAL_POLICY
            .get_or_init(Self::default)
            .get_blocks_per_epoch()
    }

    #[inline]
    pub fn genesis_block_number() -> u32 {
        GLOBAL_POLICY
            .get_or_init(Self::default)
            .get_genesis_block_number()
    }

    #[inline]
    pub fn tendermint_timeout_init() -> u64 {
        GLOBAL_POLICY
            .get_or_init(Self::default)
            .tendermint_timeout_init
    }

    #[inline]
    pub fn tendermint_timeout_delta() -> u64 {
        GLOBAL_POLICY
            .get_or_init(Self::default)
            .tendermint_timeout_delta
    }

    #[inline]
    pub fn state_chunks_max_size() -> u32 {
        GLOBAL_POLICY
            .get_or_init(Policy::default)
            .state_chunks_max_size
    }

    /// Returns the epoch number at a given block number (height).
    #[inline]
    pub fn epoch_at(block_number: u32) -> u32 {
        // If the block number is less than the genesis, we are at epoch 0
        if block_number <= Self::genesis_block_number() {
            0
        } else {
            let block_number = block_number - Self::genesis_block_number();
            let blocks_per_epoch = Self::blocks_per_epoch();
            (block_number + blocks_per_epoch - 1) / blocks_per_epoch
        }
    }

    /// Returns the epoch index at a given block number. The epoch index is the number of a block relative
    /// to the epoch it is in. For example, the first block of any epoch always has an epoch index of 0.
    #[inline]
    pub fn epoch_index_at(block_number: u32) -> u32 {
        // Any block before the genesis is considered to be part of epoch 0
        if block_number < Self::genesis_block_number() {
            block_number
        } else {
            let blocks_per_epoch = Self::blocks_per_epoch();
            let block_number = block_number - Self::genesis_block_number();
            (block_number + blocks_per_epoch - 1) % blocks_per_epoch
        }
    }

    /// Returns the batch number at a given `block_number` (height)
    #[inline]
    pub fn batch_at(block_number: u32) -> u32 {
        // If the block number is less than the genesis, we are at batch 0
        if block_number <= Self::genesis_block_number() {
            0
        } else {
            let block_number = block_number - Self::genesis_block_number();
            let blocks_per_batch = Self::blocks_per_batch();
            (block_number + blocks_per_batch - 1) / blocks_per_batch
        }
    }

    /// Returns the batch index at a given block number. The batch index is the number of a block relative
    /// to the batch it is in. For example, the first block of any batch always has an batch index of 0.
    #[inline]
    pub fn batch_index_at(block_number: u32) -> u32 {
        // No batches before the genesis block number
        if block_number < Self::genesis_block_number() {
            block_number
        } else {
            let blocks_per_batch = Self::blocks_per_batch();
            let block_number = block_number - Self::genesis_block_number();
            (block_number + blocks_per_batch - 1) % blocks_per_batch
        }
    }

    /// Returns the number (height) of the next election macro block after a given block number (height).
    #[inline]
    pub fn election_block_after(block_number: u32) -> u32 {
        // The next election block of any block before the genesis, is the genesis itself
        if block_number < Self::genesis_block_number() {
            Self::genesis_block_number()
        } else {
            let blocks_per_epoch = Self::blocks_per_epoch();
            let block_number = block_number - Self::genesis_block_number();
            ((block_number / blocks_per_epoch + 1) * blocks_per_epoch)
                + Self::genesis_block_number()
        }
    }

    /// Returns the block number (height) of the preceding election macro block before a given block number (height).
    /// If the given block number is an election macro block, it returns the election macro block before it.
    #[inline]
    pub fn election_block_before(block_number: u32) -> u32 {
        match block_number.cmp(&Self::genesis_block_number()) {
            std::cmp::Ordering::Less => {
                panic!("No election blocks before the genesis block");
            }
            std::cmp::Ordering::Equal => {
                // The genesis is the first election block
                Self::genesis_block_number()
            }
            std::cmp::Ordering::Greater => {
                let blocks_per_epoch = Self::blocks_per_epoch();
                let block_number = block_number - Self::genesis_block_number();
                ((block_number - 1) / blocks_per_epoch * blocks_per_epoch)
                    + Self::genesis_block_number()
            }
        }
    }

    /// Returns the block number (height) of the last election macro block at a given block number (height).
    /// If the given block number is an election macro block, then it returns that block number.
    #[inline]
    pub fn last_election_block(block_number: u32) -> u32 {
        // The last election block of any block before the genesis, is the genesis itself
        if block_number < Self::genesis_block_number() {
            panic!("No election blocks before the genesis block");
        } else {
            let blocks_per_epoch = Self::blocks_per_epoch();
            let block_number = block_number - Self::genesis_block_number();
            (block_number / blocks_per_epoch * blocks_per_epoch) + Self::genesis_block_number()
        }
    }

    /// Returns a boolean expressing if the block at a given block number (height) is an election macro block.
    #[inline]
    pub fn is_election_block_at(block_number: u32) -> bool {
        Self::epoch_index_at(block_number) == Self::blocks_per_epoch() - 1
    }

    /// Returns the block number (height) of the next macro block after a given block number (height).
    #[inline]
    pub fn macro_block_after(block_number: u32) -> u32 {
        // The next macro block of any block before the genesis, is the genesis itself
        if block_number < Self::genesis_block_number() {
            Self::genesis_block_number()
        } else {
            let block_number = block_number - Self::genesis_block_number();
            let blocks_per_batch = Self::blocks_per_batch();
            ((block_number / blocks_per_batch + 1) * blocks_per_batch)
                + Self::genesis_block_number()
        }
    }

    /// Returns the block number (height) of the preceding macro block before a given block number (height).
    /// If the given block number is a macro block, it returns the macro block before it.
    #[inline]
    pub fn macro_block_before(block_number: u32) -> u32 {
        if block_number <= Self::genesis_block_number() {
            panic!("No macro blocks before genesis block");
        } else {
            let blocks_per_batch = Self::blocks_per_batch();
            let block_number = block_number - Self::genesis_block_number();
            ((block_number - 1) / blocks_per_batch * blocks_per_batch)
                + Self::genesis_block_number()
        }
    }

    /// Returns the block number (height) of the last macro block at a given block number (height).
    /// If the given block number is a macro block, then it returns that block number.
    #[inline]
    pub fn last_macro_block(block_number: u32) -> u32 {
        // There is no macro block before the genesis
        if block_number < Self::genesis_block_number() {
            panic!("No macro blocks before genesis block");
        } else {
            let blocks_per_batch = Self::blocks_per_batch();
            let block_number = block_number - Self::genesis_block_number();
            (block_number / blocks_per_batch * blocks_per_batch) + Self::genesis_block_number()
        }
    }

    /// Returns a boolean expressing if the block at a given block number (height) is a macro block.
    #[inline]
    pub fn is_macro_block_at(block_number: u32) -> bool {
        // No macro blocks before genesis
        if block_number < Self::genesis_block_number() {
            false
        } else {
            Self::batch_index_at(block_number) == Self::blocks_per_batch() - 1
        }
    }

    /// Returns a boolean expressing if the block at a given block number (height) is a micro block.
    #[inline]
    pub fn is_micro_block_at(block_number: u32) -> bool {
        // No micro blocks before genesis
        if block_number < Self::genesis_block_number() {
            false
        } else {
            Self::batch_index_at(block_number) != Self::blocks_per_batch() - 1
        }
    }

    /// Returns the block number of the first block of the given epoch (which is always a micro block).
    /// If the index is out of bounds, None is returned
    pub fn first_block_of(epoch: u32) -> Option<u32> {
        if epoch == 0 {
            panic!("Called first_block_of for epoch 0");
        }

        (epoch - 1)
            .checked_mul(Self::blocks_per_epoch())?
            .checked_add(1)?
            .checked_add(Self::genesis_block_number())
    }

    /// Returns the block number of the first block of the given batch (which is always a micro block).
    /// If the index is out of bounds, None is returned
    pub fn first_block_of_batch(batch: u32) -> Option<u32> {
        if batch == 0 {
            panic!("Called first_block_of_batch for batch 0");
        }

        (batch - 1)
            .checked_mul(Self::blocks_per_batch())?
            .checked_add(1)?
            .checked_add(Self::genesis_block_number())
    }

    /// Returns the block number of the election macro block of the given epoch (which is always the last block).
    /// If the index is out of bounds, None is returned
    pub fn election_block_of(epoch: u32) -> Option<u32> {
        epoch
            .checked_mul(Self::blocks_per_epoch())?
            .checked_add(Self::genesis_block_number())
    }

    /// Returns the block number of the macro block (checkpoint or election) of the given batch (which
    /// is always the last block).
    /// If the index is out of bounds, None is returned
    pub fn macro_block_of(batch: u32) -> Option<u32> {
        batch
            .checked_mul(Self::blocks_per_batch())?
            .checked_add(Self::genesis_block_number())
    }

    /// Returns a boolean expressing if the batch at a given block number (height) is the first batch
    /// of the epoch.
    #[inline]
    pub fn first_batch_of_epoch(block_number: u32) -> bool {
        Self::epoch_index_at(block_number) < Self::blocks_per_batch()
    }

    /// Returns the block height for the last block of the reporting window of a given block number.
    /// Note: This window is meant for reporting malicious behaviour (aka `jailable` behaviour).
    #[inline]
    pub fn last_block_of_reporting_window(block_number: u32) -> u32 {
        block_number + Self::blocks_per_epoch()
    }

    /// Returns the first block after the reporting window of a given block number has ended.
    #[inline]
    pub fn block_after_reporting_window(block_number: u32) -> u32 {
        Self::last_block_of_reporting_window(block_number) + 1
    }

    /// Returns the first block after the jail period of a given block number has ended.
    #[inline]
    pub fn block_after_jail(block_number: u32) -> u32 {
        block_number + Self::blocks_per_epoch() * Self::JAIL_EPOCHS + 1
    }

    /// Returns the supply at a given time (as Unix time) in Lunas (1 NIM = 100,000 Lunas). It is
    /// calculated using the following formula:
    /// Supply (t) = Genesis_supply + Initial_supply_velocity / Supply_decay * (1 - e^(- Supply_decay * t))
    /// Where e is the exponential function, t is the time in milliseconds since the genesis block and
    /// Genesis_supply is the supply at the genesis of the Nimiq 2.0 chain.
    pub fn supply_at(genesis_supply: u64, genesis_time: u64, current_time: u64) -> u64 {
        assert!(current_time >= genesis_time);

        let t = (current_time - genesis_time) as f64;

        let exponent = -Policy::SUPPLY_DECAY * t;

        let supply = genesis_supply
            + (Self::INITIAL_SUPPLY_VELOCITY / Self::SUPPLY_DECAY * (1.0 - exponent.exp())) as u64;

        cmp::min(supply, Policy::TOTAL_SUPPLY)
    }

    /// Returns the percentage reduction that should be applied to the rewards due to a delayed batch.
    /// This function returns a float in the range [0, 1]
    /// I.e 1 means that the full rewards should be given, whereas 0.5 means that half of the rewards should be given
    /// The input to this function is the batch delay, in milliseconds
    /// The function is: [(1 - MINIMUM_REWARDS_PERCENTAGE) * e ^(-BLOCKS_DELAY_DECAY * t^2)] + MINIMUM_REWARDS_PERCENTAGE
    pub fn batch_delay_penalty(delay: u64) -> f64 {
        let t = delay as f64;
        let exponent = -Self::BLOCKS_DELAY_DECAY * t * t;

        (1.0 - Self::MINIMUM_REWARDS_PERCENTAGE) * exponent.exp() + Self::MINIMUM_REWARDS_PERCENTAGE
    }

    #[inline]
    pub fn get_or_init(policy: Policy) -> Policy {
        *GLOBAL_POLICY.get_or_init(|| policy)
    }
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            blocks_per_batch: 60,
            batches_per_epoch: 360,
            tendermint_timeout_init: 1000,
            tendermint_timeout_delta: 1000,
            state_chunks_max_size: 200, // #Nodes/accounts 200, TODO: Simulate with different sizes
            transaction_validity_window: 7200,
            genesis_block_number: 0,
        }
    }
}

pub const TEST_POLICY: Policy = Policy {
    blocks_per_batch: 32,
    batches_per_epoch: 4,
    tendermint_timeout_init: 1000,
    tendermint_timeout_delta: 1000,
    state_chunks_max_size: 2,
    transaction_validity_window: 64,
    genesis_block_number: 0,
};

#[cfg(test)]
mod tests {
    use nimiq_test_log::test;

    use super::*;

    fn initialize_policy() {
        let mut policy_config = TEST_POLICY;
        policy_config.genesis_block_number = 200;

        let _ = Policy::get_or_init(policy_config);
    }

    #[test]
    fn it_correctly_computes_epoch() {
        initialize_policy();
        assert_eq!(Policy::epoch_at(Policy::genesis_block_number()), 0);
        assert_eq!(Policy::epoch_at(1 + Policy::genesis_block_number()), 1);
        assert_eq!(
            Policy::epoch_at(Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()),
            1
        );
        assert_eq!(
            Policy::epoch_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            2
        );
    }

    #[test]
    fn it_correctly_computes_epoch_index() {
        initialize_policy();
        assert_eq!(
            Policy::epoch_index_at(1 + Policy::genesis_block_number()),
            0
        );
        assert_eq!(
            Policy::epoch_index_at(2 + Policy::genesis_block_number()),
            1
        );
        assert_eq!(
            Policy::epoch_index_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            127
        );
        assert_eq!(
            Policy::epoch_index_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            0
        );
    }

    #[test]
    fn it_correctly_computes_batch() {
        initialize_policy();
        assert_eq!(Policy::batch_at(Policy::genesis_block_number()), 0);
        assert_eq!(Policy::batch_at(1 + Policy::genesis_block_number()), 1);
        assert_eq!(
            Policy::batch_at(Policy::blocks_per_batch() as u32 + Policy::genesis_block_number()),
            1
        );
        assert_eq!(
            Policy::batch_at(
                Policy::blocks_per_batch() as u32 + Policy::genesis_block_number() + 1
            ),
            2
        );
    }

    #[test]
    fn it_correctly_computes_batch_index() {
        initialize_policy();
        assert_eq!(
            Policy::batch_index_at(1 + Policy::genesis_block_number()),
            0
        );
        assert_eq!(
            Policy::batch_index_at(2 + Policy::genesis_block_number()),
            1
        );
        assert_eq!(
            Policy::batch_index_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            31
        );
        assert_eq!(
            Policy::batch_index_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            0
        );
    }

    #[test]
    fn it_correctly_computes_block_positions() {
        initialize_policy();
        assert_eq!(
            Policy::is_macro_block_at(Policy::genesis_block_number()),
            true
        );
        assert_eq!(
            !Policy::is_micro_block_at(Policy::genesis_block_number()),
            true
        );
        assert_eq!(
            Policy::is_election_block_at(Policy::genesis_block_number()),
            true
        );

        assert_eq!(
            Policy::is_macro_block_at(1 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            !Policy::is_micro_block_at(1 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(1 + Policy::genesis_block_number()),
            false
        );

        assert_eq!(
            Policy::is_macro_block_at(2 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            !Policy::is_micro_block_at(2 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(2 + Policy::genesis_block_number()),
            false
        );

        assert_eq!(
            Policy::is_macro_block_at(
                Policy::blocks_per_batch() as u32 + Policy::genesis_block_number()
            ),
            true
        );
        assert_eq!(
            Policy::is_micro_block_at(
                Policy::blocks_per_batch() as u32 + Policy::genesis_block_number()
            ),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(
                Policy::blocks_per_batch() as u32 + Policy::genesis_block_number()
            ),
            false
        );

        assert_eq!(
            Policy::is_macro_block_at(127 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            !Policy::is_micro_block_at(127 + Policy::genesis_block_number()),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(127 + Policy::genesis_block_number()),
            false
        );

        assert_eq!(
            Policy::is_macro_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            true
        );
        assert_eq!(
            !Policy::is_micro_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            true
        );
        assert_eq!(
            Policy::is_election_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            true
        );

        assert_eq!(
            Policy::is_macro_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            false
        );
        assert_eq!(
            !Policy::is_micro_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            false
        );

        assert_eq!(
            Policy::is_macro_block_at(
                Policy::blocks_per_epoch()
                    + Policy::blocks_per_batch() as u32
                    + Policy::genesis_block_number()
            ),
            true
        );
        assert_eq!(
            Policy::is_micro_block_at(
                Policy::blocks_per_epoch()
                    + Policy::blocks_per_batch() as u32
                    + Policy::genesis_block_number()
            ),
            false
        );
        assert_eq!(
            Policy::is_election_block_at(
                Policy::blocks_per_epoch()
                    + Policy::blocks_per_batch() as u32
                    + Policy::genesis_block_number()
            ),
            false
        );
    }

    #[test]
    fn it_correctly_computes_macro_numbers() {
        initialize_policy();
        assert_eq!(
            Policy::macro_block_after(Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_batch() as u32
        );
        assert_eq!(
            Policy::macro_block_after(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_batch() as u32
        );
        assert_eq!(
            Policy::macro_block_after(127 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::macro_block_after(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            Policy::genesis_block_number() + 160
        );
        assert_eq!(
            Policy::macro_block_after(129 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + 160
        );

        assert_eq!(
            Policy::macro_block_before(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::macro_block_before(2 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::macro_block_before(127 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + 96
        );
        assert_eq!(
            Policy::macro_block_before(128 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + 96
        );
        assert_eq!(
            Policy::macro_block_before(129 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::macro_block_before(130 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::last_macro_block(Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::last_macro_block(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::last_macro_block(31 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );

        assert_eq!(
            Policy::last_macro_block(
                Policy::blocks_per_batch() + Policy::genesis_block_number() + 1
            ),
            Policy::genesis_block_number() + 32
        );
    }

    #[test]
    fn it_correctly_computes_election_numbers() {
        initialize_policy();
        assert_eq!(
            Policy::election_block_after(Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::election_block_after(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::election_block_after(127 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::election_block_after(128 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + 256
        );
        assert_eq!(
            Policy::election_block_after(129 + Policy::genesis_block_number()),
            Policy::genesis_block_number() + 256
        );

        assert_eq!(
            Policy::election_block_before(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::election_block_before(2 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::election_block_before(127 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::election_block_before(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::election_block_before(
                Policy::blocks_per_epoch() as u32 + 1 + Policy::genesis_block_number()
            ),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::election_block_before(
                Policy::blocks_per_epoch() as u32 + 2 + Policy::genesis_block_number()
            ),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );

        assert_eq!(
            Policy::last_election_block(Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::last_election_block(1 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::last_election_block(127 + Policy::genesis_block_number()),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::last_election_block(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
        assert_eq!(
            Policy::last_election_block(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number() + 1
            ),
            Policy::genesis_block_number() + Policy::blocks_per_epoch() as u32
        );
    }

    #[test]
    fn it_correctly_commutes_first_ofs() {
        initialize_policy();
        assert_eq!(
            Policy::first_block_of(1),
            Some(Policy::genesis_block_number() + 1)
        );
        assert_eq!(
            Policy::first_block_of(2),
            Some(Policy::genesis_block_number() + Policy::blocks_per_epoch() + 1)
        );

        assert_eq!(
            Policy::first_block_of_batch(1),
            Some(1 + Policy::genesis_block_number())
        );
        assert_eq!(
            Policy::first_block_of_batch(2),
            Some(33 + Policy::genesis_block_number())
        );
        assert_eq!(
            Policy::first_block_of_batch(3),
            Some(65 + Policy::genesis_block_number())
        );
        assert_eq!(
            Policy::first_block_of_batch(4),
            Some(97 + Policy::genesis_block_number())
        );
        assert_eq!(
            Policy::first_block_of_batch(5),
            Some(129 + Policy::genesis_block_number())
        );
        assert_eq!(Policy::first_block_of_batch(4294967295), None);
    }

    #[test]
    fn it_correctly_computes_first_batch_of_epoch() {
        initialize_policy();
        assert_eq!(
            Policy::first_batch_of_epoch(1 + Policy::genesis_block_number()),
            true
        );
        assert_eq!(
            Policy::first_batch_of_epoch(
                Policy::blocks_per_batch() as u32 + Policy::genesis_block_number()
            ),
            true
        );
        assert_eq!(
            Policy::first_batch_of_epoch(
                Policy::blocks_per_batch() as u32 + 1 + Policy::genesis_block_number()
            ),
            false
        );
        assert_eq!(
            Policy::first_batch_of_epoch(
                Policy::blocks_per_epoch() as u32 + Policy::genesis_block_number()
            ),
            false
        );
        assert_eq!(
            Policy::first_batch_of_epoch(
                Policy::blocks_per_epoch() as u32 + 1 + Policy::genesis_block_number()
            ),
            true
        );
    }

    #[test]
    fn non_zero_genesis_extra_tests() {
        initialize_policy();

        // Anything prior to genesis belongs to epoch 0
        assert_eq!(Policy::epoch_at(Policy::genesis_block_number()), 0);
        assert_eq!(Policy::epoch_at(40), 0);
        // Epoch 1 starts at genesis + 1
        assert_eq!(Policy::epoch_at(1 + Policy::genesis_block_number()), 1);

        // If genesis is 200, this corresponds to block 401.
        assert_eq!(
            Policy::epoch_index_at(2 * Policy::genesis_block_number() + 1),
            401 - (Policy::genesis_block_number() + Policy::blocks_per_epoch()) - 1
        );

        //First batch starts after genesis
        assert_eq!(Policy::batch_at(Policy::genesis_block_number() + 1), 1);
        //Anything prior to genesis belongs to batch 0
        assert_eq!(Policy::batch_at(Policy::genesis_block_number() - 15), 0);

        assert_eq!(
            Policy::batch_index_at(Policy::genesis_block_number() + 1),
            0
        );
        assert_eq!(
            Policy::batch_index_at(Policy::genesis_block_number() + 2),
            1
        );

        // No macro blocks before genesis
        assert_eq!(Policy::is_macro_block_at(1), false);
        assert_eq!(
            Policy::is_macro_block_at(Policy::genesis_block_number()),
            true
        );

        // No micro blocks before genesis
        assert_eq!(
            Policy::is_micro_block_at(Policy::genesis_block_number() - 20),
            false
        );
        assert_eq!(Policy::is_micro_block_at(15), false);

        // Genesis is a macro/election block
        assert_eq!(
            Policy::is_macro_block_at(Policy::genesis_block_number()),
            true
        );
        assert_eq!(
            Policy::is_election_block_at(Policy::genesis_block_number()),
            true
        );

        // The next macro for any pre-genesis block is the genesis itself
        assert_eq!(Policy::macro_block_after(0), Policy::genesis_block_number());
        assert_eq!(Policy::macro_block_after(5), Policy::genesis_block_number());

        // The next election for any pre-genesis block is the genesis itself
        assert_eq!(
            Policy::election_block_after(0),
            Policy::genesis_block_number()
        );
        assert_eq!(
            Policy::election_block_after(10),
            Policy::genesis_block_number()
        );
    }
}
