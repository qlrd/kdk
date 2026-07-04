// SPDX-License-Identifier: MIT

//! BIP39 mnemonic conversion for KDK.
//!
//! Each `*_mnemonic` pipeline returns a [`GameMnemonic<O, W>`] — a
//! sensitive wrapper carrying an origin marker `O` (e.g.
//! [`kdk_entropy::Coin`], [`kdk_entropy::Dice<F>`],
//! [`kdk_entropy::Deck<C>`]) and the BIP39 word count
//! `W in [12, 15, 18, 21, 24]`.
//!
//! Supported word counts are gated by Cargo features. Defaults
//! (`words-12`, `words-24`) enable the Krux-compatible 12 and 24-word
//! mnemonics; `extended` enables 15/18/21-word.

#![no_std]
#![doc(html_logo_url = "https://qlrd.github.io/kdk/logo.png")]

mod error;
mod game;

#[cfg(feature = "coin")]
mod coin;

#[cfg(feature = "deck")]
mod deck;

#[cfg(feature = "dice")]
mod dice;

pub use error::MnemonicError;
pub use game::{entropy_to_mnemonic, GameMnemonic, SensitiveGame};

#[cfg(feature = "coin")]
pub use coin::{coin_mnemonic, CoinMnemonic};

#[cfg(feature = "deck")]
pub use deck::{deck_mnemonic, DeckMnemonic};

#[cfg(feature = "dice")]
pub use dice::{dice_mnemonic, DiceMnemonic};
