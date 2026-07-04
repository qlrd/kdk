#![cfg(feature = "dice")]

use kdk_entropy::EntropyError;
use kdk_mnemonic::{dice_mnemonic, DiceMnemonic, MnemonicError};
use kdk_zeroize::prelude::*;

macro_rules! dice_produces_words {
    ($name:ident, $faces:literal, $w:literal, $input:expr) => {
        #[test]
        fn $name() {
            let m: DiceMnemonic<$faces, $w> = dice_mnemonic::<$faces, $w>(&$input).unwrap();
            assert_eq!(m.expose_secret().word_count(), $w);
        }
    };
}

macro_rules! dice_rejects_entropy {
    ($name:ident, $faces:literal, $w:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = dice_mnemonic::<$faces, $w>(&$input).err().unwrap();
            match err {
                MnemonicError::Entropy(e) => assert_eq!(e, $err),
                other => panic!("expected MnemonicError::Entropy, got {other:?}"),
            }
        }
    };
}

// --- 12-word (N=16) success vectors ---

#[cfg(feature = "words-12")]
dice_produces_words!(d6_50_rolls_yield_12_words, 6, 12, [1u8; 50]);
#[cfg(feature = "words-12")]
dice_produces_words!(d20_30_rolls_yield_12_words, 20, 12, [1u8; 30]);

// --- 24-word (N=32) success vectors ---

#[cfg(feature = "words-24")]
dice_produces_words!(d6_99_rolls_yield_24_words, 6, 24, [1u8; 99]);
#[cfg(feature = "words-24")]
dice_produces_words!(d20_60_rolls_yield_24_words, 20, 24, [1u8; 60]);

// --- length errors ---

#[cfg(feature = "words-12")]
dice_rejects_entropy!(d6_49_rolls_too_few, 6, 12, [1u8; 49] => EntropyError::TooFewRolls(50, 49));
#[cfg(feature = "words-12")]
dice_rejects_entropy!(d6_101_rolls_too_many, 6, 12, [1u8; 101] => EntropyError::TooManyRolls(100, 101));
#[cfg(feature = "words-12")]
dice_rejects_entropy!(d20_29_rolls_too_few, 20, 12, [1u8; 29] => EntropyError::TooFewRolls(30, 29));

// --- per-roll validation ---

#[cfg(feature = "words-12")]
#[test]
fn d6_roll_zero_rejected() {
    let mut rolls = [1u8; 50];
    rolls[10] = 0;
    let err = dice_mnemonic::<6, 12>(&rolls).err().unwrap();
    match err {
        MnemonicError::Entropy(EntropyError::RollOutOfRange(0, 10)) => {}
        other => panic!("expected RollOutOfRange(0, 10), got {other:?}"),
    }
}

#[cfg(feature = "words-12")]
#[test]
fn d6_roll_seven_rejected() {
    let mut rolls = [1u8; 50];
    rolls[10] = 7; // 7 > FACES=6
    let err = dice_mnemonic::<6, 12>(&rolls).err().unwrap();
    match err {
        MnemonicError::Entropy(EntropyError::RollOutOfRange(7, 10)) => {}
        other => panic!("expected RollOutOfRange(7, 10), got {other:?}"),
    }
}

// --- (FACES, N) pair not in the table ---

// dice supports only FACES ∈ {6, 20}; FACES=10 is unsupported.
#[cfg(feature = "words-12")]
dice_rejects_entropy!(d10_unsupported, 10, 12, [1u8; 50] => EntropyError::UnsupportedConfig(10, 16));

// dice (FACES=6) supports only N ∈ {16, 32}; asking 15 words → N=20 → unsupported.
#[cfg(feature = "words-15")]
dice_rejects_entropy!(d6_w15_unsupported, 6, 15, [1u8; 50] => EntropyError::UnsupportedConfig(6, 20));
#[cfg(feature = "words-18")]
dice_rejects_entropy!(d6_w18_unsupported, 6, 18, [1u8; 50] => EntropyError::UnsupportedConfig(6, 24));
#[cfg(feature = "words-21")]
dice_rejects_entropy!(d6_w21_unsupported, 6, 21, [1u8; 50] => EntropyError::UnsupportedConfig(6, 28));

// --- determinism & order sensitivity ---

#[cfg(feature = "words-12")]
#[test]
fn deterministic_for_same_input() {
    let rolls = [1u8; 50];
    let a: DiceMnemonic<6, 12> = dice_mnemonic::<6, 12>(&rolls).unwrap();
    let b: DiceMnemonic<6, 12> = dice_mnemonic::<6, 12>(&rolls).unwrap();
    assert_eq!(a.expose_secret().to_string(), b.expose_secret().to_string());
}

#[cfg(feature = "words-12")]
#[test]
fn order_sensitive() {
    let mut a = [1u8; 50];
    a[0] = 6;
    let mut b = [1u8; 50];
    b[49] = 6;
    let ma: DiceMnemonic<6, 12> = dice_mnemonic::<6, 12>(&a).unwrap();
    let mb: DiceMnemonic<6, 12> = dice_mnemonic::<6, 12>(&b).unwrap();
    assert_ne!(
        ma.expose_secret().to_string(),
        mb.expose_secret().to_string()
    );
}

// --- W ∉ {12,15,18,21,24}: catch-all in dice_mnemonic ---

#[test]
fn invalid_word_count_rejected() {
    let rolls = [1u8; 50];
    let err = dice_mnemonic::<6, 13>(&rolls).err().unwrap();
    match err {
        MnemonicError::InvalidWordCount(13) => {}
        other => panic!("expected InvalidWordCount(13), got {other:?}"),
    }
}
