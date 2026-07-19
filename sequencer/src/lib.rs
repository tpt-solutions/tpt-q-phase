// SPDX-License-Identifier: MIT OR Apache-2.0

//! Deterministic, nanosecond-precision pulse sequencer core.
//!
//! This crate provides the `no_std`-compatible core logic that translates
//! quantum gate sequences into pulse parameter streams, plus a state-vector
//! quantum simulator used as a stand-in for real hardware.

#![no_std]
#![cfg_attr(feature = "std", allow(unused_imports))]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod gates;
pub mod pulse;
pub mod schedule;
pub mod simulator;
pub mod state;
