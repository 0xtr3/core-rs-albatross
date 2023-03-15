//! This module contains all of the "gadgets" for the zk-SNARK circuits. These are smaller, modular pieces
//! of on-circuit code intended to facilitate the creation of larger circuits.

pub mod mnt4;
pub mod mnt6;

pub mod be_bytes;
pub mod bits;
pub mod pedersen;
pub mod recursive_input;
pub mod serialize;
pub mod y_to_bit;
