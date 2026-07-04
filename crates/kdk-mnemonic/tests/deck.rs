#![cfg(feature = "deck")]

use kdk_entropy::EntropyError;
use kdk_mnemonic::{deck_mnemonic, DeckMnemonic, MnemonicError};
use kdk_zeroize::prelude::*;

const fn ascending<const K: usize>() -> [u8; K] {
    let mut arr = [0u8; K];
    let mut i = 0;
    while i < K {
        arr[i] = i as u8;
        i += 1;
    }
    arr
}

macro_rules! deck_produces_words {
    ($name:ident, $cards:literal, $w:literal, $input:expr) => {
        #[test]
        fn $name() {
            let m: DeckMnemonic<$cards, $w> = deck_mnemonic::<$cards, $w>(&$input).unwrap();
            assert_eq!(m.expose_secret().word_count(), $w);
        }
    };
}

macro_rules! deck_rejects_entropy {
    ($name:ident, $cards:literal, $w:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = deck_mnemonic::<$cards, $w>(&$input).err().unwrap();
            match err {
                MnemonicError::Entropy(e) => assert_eq!(e, $err),
                other => panic!("expected MnemonicError::Entropy, got {other:?}"),
            }
        }
    };
}

// --- 12-word (N=16) success vectors across deck sizes ---

#[cfg(feature = "words-12")]
deck_produces_words!(d40_28_draws_yield_12_words, 40, 12, ascending::<28>());
#[cfg(feature = "words-12")]
deck_produces_words!(d48_26_draws_yield_12_words, 48, 12, ascending::<26>());
#[cfg(feature = "words-12")]
deck_produces_words!(d52_25_draws_yield_12_words, 52, 12, ascending::<25>());
#[cfg(feature = "words-12")]
deck_produces_words!(d58_24_draws_yield_12_words, 58, 12, ascending::<24>());
#[cfg(feature = "words-12")]
deck_produces_words!(d78_22_draws_yield_12_words, 78, 12, ascending::<22>());
#[cfg(feature = "words-12")]
deck_produces_words!(d108_22_draws_yield_12_words, 108, 12, ascending::<22>());

// --- 24-word (N=32) success vectors — only decks that have a (CARDS, 32) row ---

#[cfg(feature = "words-24")]
deck_produces_words!(d58_55_draws_yield_24_words, 58, 24, ascending::<55>());
#[cfg(feature = "words-24")]
deck_produces_words!(d78_45_draws_yield_24_words, 78, 24, ascending::<45>());
#[cfg(feature = "words-24")]
deck_produces_words!(d108_41_draws_yield_24_words, 108, 24, ascending::<41>());

// --- length errors ---

#[cfg(feature = "words-12")]
deck_rejects_entropy!(d52_24_draws_too_few, 52, 12, ascending::<24>() => EntropyError::TooFewRolls(25, 24));
// max_draws(52, 16) = min(2*25, 52) = 50; 51 draws should be rejected.
#[cfg(feature = "words-12")]
deck_rejects_entropy!(d52_51_draws_too_many, 52, 12, ascending::<51>() => EntropyError::TooManyRolls(50, 51));

// --- per-card validation ---

#[cfg(feature = "words-12")]
#[test]
fn card_index_out_of_range_for_d52() {
    let mut cards = ascending::<25>();
    cards[10] = 52; // 52 is out of range for CARDS=52 (valid indices: 0..52)
    let err = deck_mnemonic::<52, 12>(&cards).err().unwrap();
    match err {
        MnemonicError::Entropy(EntropyError::RollOutOfRange(52, 10)) => {}
        other => panic!("expected RollOutOfRange(52, 10), got {other:?}"),
    }
}

#[cfg(feature = "words-12")]
#[test]
fn duplicate_card_rejected_for_d52() {
    let mut cards = ascending::<25>();
    cards[24] = 0; // duplicates cards[0]
    let err = deck_mnemonic::<52, 12>(&cards).err().unwrap();
    match err {
        MnemonicError::Entropy(EntropyError::DuplicateCard(0, 24)) => {}
        other => panic!("expected DuplicateCard(0, 24), got {other:?}"),
    }
}

// --- (CARDS, N) pair not in the table ---

// (52, 32) is not supported — 52-card deck can't reach 256 bits.
#[cfg(feature = "words-24")]
deck_rejects_entropy!(d52_w24_unsupported, 52, 24, ascending::<50>() => EntropyError::UnsupportedConfig(52, 32));

// --- determinism & order sensitivity ---

#[cfg(feature = "words-12")]
#[test]
fn deterministic_for_same_input() {
    let cards = ascending::<25>();
    let a: DeckMnemonic<52, 12> = deck_mnemonic::<52, 12>(&cards).unwrap();
    let b: DeckMnemonic<52, 12> = deck_mnemonic::<52, 12>(&cards).unwrap();
    assert_eq!(a.expose_secret().to_string(), b.expose_secret().to_string());
}

#[cfg(feature = "words-12")]
#[test]
fn order_sensitive() {
    let a = ascending::<25>();
    let mut b = ascending::<25>();
    b.swap(0, 24);
    let ma: DeckMnemonic<52, 12> = deck_mnemonic::<52, 12>(&a).unwrap();
    let mb: DeckMnemonic<52, 12> = deck_mnemonic::<52, 12>(&b).unwrap();
    assert_ne!(
        ma.expose_secret().to_string(),
        mb.expose_secret().to_string()
    );
}

// --- W ∉ {12,15,18,21,24}: catch-all in deck_mnemonic ---

#[test]
fn invalid_word_count_rejected() {
    let cards = ascending::<25>();
    let err = deck_mnemonic::<52, 13>(&cards).err().unwrap();
    match err {
        MnemonicError::InvalidWordCount(13) => {}
        other => panic!("expected InvalidWordCount(13), got {other:?}"),
    }
}
