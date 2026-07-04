#![cfg(feature = "coin")]

use kdk_entropy::EntropyError;
use kdk_mnemonic::{coin_mnemonic, CoinMnemonic, MnemonicError};
use kdk_zeroize::prelude::*;

macro_rules! coin_produces_words {
    ($name:ident, $w:literal, $input:expr) => {
        #[test]
        fn $name() {
            let m: CoinMnemonic<$w> = coin_mnemonic::<$w>(&$input).unwrap();
            assert_eq!(m.expose_secret().word_count(), $w);
        }
    };
}

macro_rules! coin_rejects_entropy {
    ($name:ident, $w:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = coin_mnemonic::<$w>(&$input).err().unwrap();
            match err {
                MnemonicError::Entropy(e) => assert_eq!(e, $err),
                other => panic!("expected MnemonicError::Entropy, got {other:?}"),
            }
        }
    };
}

#[cfg(feature = "words-12")]
coin_produces_words!(all_zeros_128_flips_yield_12_words, 12, [0u8; 128]);
#[cfg(feature = "words-12")]
coin_produces_words!(all_ones_128_flips_yield_12_words, 12, [1u8; 128]);

#[cfg(feature = "words-24")]
coin_produces_words!(all_zeros_256_flips_yield_24_words, 24, [0u8; 256]);
#[cfg(feature = "words-24")]
coin_produces_words!(all_ones_256_flips_yield_24_words, 24, [1u8; 256]);

#[cfg(feature = "words-12")]
coin_rejects_entropy!(too_few_flips_for_12_words, 12, [0u8; 127] => EntropyError::TooFewRolls(128, 127));
#[cfg(feature = "words-12")]
coin_rejects_entropy!(too_many_flips_for_12_words, 12, [0u8; 257] => EntropyError::TooManyRolls(256, 257));

#[cfg(feature = "words-24")]
coin_rejects_entropy!(too_few_flips_for_24_words, 24, [0u8; 255] => EntropyError::TooFewRolls(256, 255));

#[cfg(feature = "words-12")]
#[test]
fn deterministic_for_same_input() {
    let flips = [0u8; 128];
    let a: CoinMnemonic<12> = coin_mnemonic::<12>(&flips).unwrap();
    let b: CoinMnemonic<12> = coin_mnemonic::<12>(&flips).unwrap();
    assert_eq!(a.expose_secret().to_string(), b.expose_secret().to_string());
}

#[cfg(feature = "words-12")]
#[test]
fn order_sensitive() {
    let mut a = [0u8; 128];
    a[0] = 1;
    let mut b = [0u8; 128];
    b[127] = 1;
    let ma: CoinMnemonic<12> = coin_mnemonic::<12>(&a).unwrap();
    let mb: CoinMnemonic<12> = coin_mnemonic::<12>(&b).unwrap();
    assert_ne!(
        ma.expose_secret().to_string(),
        mb.expose_secret().to_string()
    );
}

#[cfg(feature = "words-12")]
#[test]
fn flip_value_two_rejected() {
    let mut flips = [0u8; 128];
    flips[10] = 2;
    let err = coin_mnemonic::<12>(&flips).err().unwrap();
    match err {
        MnemonicError::Entropy(EntropyError::RollOutOfRange(2, 10)) => {}
        other => panic!("expected RollOutOfRange(2, 10), got {other:?}"),
    }
}

#[cfg(feature = "words-15")]
coin_produces_words!(all_zeros_160_flips_yield_15_words, 15, [0u8; 160]);
#[cfg(feature = "words-18")]
coin_produces_words!(all_zeros_192_flips_yield_18_words, 18, [0u8; 192]);
#[cfg(feature = "words-21")]
coin_produces_words!(all_zeros_224_flips_yield_21_words, 21, [0u8; 224]);

// W ∉ {12,15,18,21,24}: hits the catch-all → InvalidWordCount.
#[test]
fn invalid_word_count_rejected() {
    let flips = [0u8; 128];
    let err = coin_mnemonic::<13>(&flips).err().unwrap();
    match err {
        MnemonicError::InvalidWordCount(13) => {}
        other => panic!("expected InvalidWordCount(13), got {other:?}"),
    }
}
