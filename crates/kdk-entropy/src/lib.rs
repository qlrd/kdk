// SPDX-License-Identifier: MIT

//! Origin-tagged entropy primitives for KDK. Every entropy buffer
//! carries a phantom marker that records *where the bytes came from*.
//!
//! Distinct sources are distinct types at compile time — `DiceEntropy<6, 16>`
//! cannot be handed to a function expecting `DiceEntropy<20, 16>`.
//!
//! # Security
//!
//! Entropy buffers wrap [`kdk_zeroize::SensitiveBytes`]. Anything that crate
//! guarantees, this one inherits.

#![no_std]
#![doc(html_logo_url = "https://qlrd.github.io/kdk/logo.png")]

use kdk_zeroize::SensitiveBytes;

/// Origin marker for coin-flip entropy. A coin always has 2 sides.
pub enum Coin {}

/// Origin arker for dice entropy. `FACES` is the die geometry — e.g.
/// `Dice<6>` for a d6, `Dice<20>` for a d20. Distinct values yield
/// distinct types at compile time.
pub enum Dice<const FACES: u8> {}

/// Origin marker for card-deck entropy. `CARDS` is the deck size; the
/// entropy a full shuffle delivers is `log₂(CARDS!)` bits.
///
/// - `Deck<32>` — Skat. ~118 bits, **too small for BIP39**.
/// - `Deck<40>` — Spanish baraja, stripped. ~159 bits, 12-word only.
/// - `Deck<48>` — Pinochle / Spanish baraja (full). ~202 bits, 12-word only.
/// - `Deck<52>` — standard poker / playing cards. ~226 bits, 12-word only;
///   **never** use for 24-word mnemonics.
/// - `Deck<58>` — smallest deck that clears 256 bits (~260). First size
///   that supports 24-word mnemonics.
/// - `Deck<78>` — Tarot. ~380 bits, 12- or 24-word.
/// - `Deck<108>` — UNO (pre-2018). ~575 bits, 12- or 24-word.
/// - `Deck<112>` — UNO (2018+). ~599 bits, 12- or 24-word.
///
/// Greater the deck, fewer the draws, but the harder the UX.
pub enum Deck<const CARDS: u8> {}

/// Coin-flip entropy. `N` is the buffer length in bytes.
pub type CoinEntropy<const N: usize> = SensitiveBytes<N, Coin>;

/// Dice-origin entropy. `FACES` is the die geometry, `N` the buffer length in bytes.
pub type DiceEntropy<const FACES: u8, const N: usize> = SensitiveBytes<N, Dice<FACES>>;

/// Deck-origin entropy. `CARDS` is the card set game, `N` the buffer length in bytes.
pub type DeckEntropy<const CARDS: u8, const N: usize> = SensitiveBytes<N, Deck<CARDS>>;

mod coin;
mod deck;
mod dice;
mod error;
mod utils;

pub use coin::{coin_to_entropy, max_flips, min_flips};
pub use deck::{deck_to_entropy, max_draws, min_draws};
pub use dice::{dice_to_entropy, max_rolls, min_rolls};
pub use error::EntropyError;
