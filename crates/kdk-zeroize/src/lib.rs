//! SPDX-License-Identifier: MIT
//!
//! # Security
//!
//! > Every secret byte that touches memory in a KDK-built signer flows through
//! > types defined here. A bug in this crate — a missed `Drop`, a leaky `Debug`,
//! > a copy that escapes the wipe — silently breaks the amnesia guarantee across
//! > EVERY downstream crate.
//!
//! # About
//!
//! Every KDK crate that touches secret bytes pulls **this** crate
//! rather than `zeroize` directly. We aim to be an auditable-friendly project
//! on level at purge secret bytes correctly.
//!
//! # Security notice
//!
//! Treat this crate's API surface as a **published security contract**.
//! Breaking changes here propagate amnesia regressions
//! through every downstream crate.

#![no_std]

mod sensitive;
mod wipe;

pub use sensitive::SensitiveBytes;
pub use wipe::wipe_in_place_mut;
