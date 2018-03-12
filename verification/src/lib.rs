extern crate xmr_primitives as primitives;

mod pow;

pub use pow::{Difficulty, is_valid_proof_of_work};
