//! Conway's Game of Life

#![cfg_attr(not(test), no_std)]

mod state;
#[doc(inline)]
pub use state::{State, new_state, tick};
#[cfg(feature = "rand")]
#[doc(inline)]
pub use state::random_state;

pub mod naive_state;

mod rand;
#[cfg(feature = "rand")]
#[doc(inline)]
pub use rand::{ByteGenerator, fill};
