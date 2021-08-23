#![cfg_attr(not(feature = "std"), no_std)]

pub mod exec_params;

pub use exec_params::*;
pub use molecule;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::vec;
        pub use std::borrow::ToOwned;
        pub use ckb_types;
    } else  if #[cfg(feature = "no-std")] {
        extern crate alloc;
        pub use alloc::vec;
        pub use alloc::borrow::ToOwned;
        pub use ckb_std::{self, ckb_types};
    }
}
