use kdk_entropy::{coin_to_entropy, max_flips, min_flips, CoinEntropy, EntropyError};
use kdk_zeroize::prelude::*;

macro_rules! min_flips_ok {
    ($name:ident, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(min_flips::<$n>(), Ok($expected));
        }
    };
}

macro_rules! max_flips_ok {
    ($name:ident, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(max_flips::<$n>(), Ok($expected));
        }
    };
}

macro_rules! flips_unsupported {
    ($name:ident, $n:literal) => {
        #[test]
        fn $name() {
            assert_eq!(
                min_flips::<$n>(),
                Err(EntropyError::UnsupportedConfig(2, $n))
            );
            assert_eq!(
                max_flips::<$n>(),
                Err(EntropyError::UnsupportedConfig(2, $n))
            );
        }
    };
}

macro_rules! coin_vector {
    ($name:ident, $n:literal, $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let e: CoinEntropy<$n> = coin_to_entropy(&$input).unwrap();
            assert_eq!(e.expose_secret(), $expected);
        }
    };
}

macro_rules! coin_rejects {
    ($name:ident, $n:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = coin_to_entropy::<$n>(&$input).err().unwrap();
            assert_eq!(err, $err);
        }
    };
}

min_flips_ok!(min_flips_128bit, 16 => 128);
min_flips_ok!(min_flips_160bit, 20 => 160);
min_flips_ok!(min_flips_192bit, 24 => 192);
min_flips_ok!(min_flips_224bit, 28 => 224);
min_flips_ok!(min_flips_256bit, 32 => 256);

max_flips_ok!(max_flips_128bit, 16 => 256);
max_flips_ok!(max_flips_160bit, 20 => 320);
max_flips_ok!(max_flips_192bit, 24 => 384);
max_flips_ok!(max_flips_224bit, 28 => 448);
max_flips_ok!(max_flips_256bit, 32 => 512);

flips_unsupported!(flips_unsupported_18, 18);
flips_unsupported!(flips_unsupported_1, 1);

// SHA-256("0" * 128)[:16]
coin_vector!(all_zeros_matches_sha256, 16, [0u8; 128] => &[
    0x45, 0x72, 0x57, 0x91, 0xc4, 0x7b, 0x32, 0x61,
    0x8c, 0xc5, 0x7b, 0x88, 0x34, 0x3e, 0x2b, 0xce,
]);

// SHA-256("1" * 128)[:16]
coin_vector!(all_ones_matches_sha256, 16, [1u8; 128] => &[
    0x4f, 0xf5, 0xac, 0x52, 0xaa, 0x16, 0xdb, 0xe3,
    0xdb, 0x44, 0x7e, 0xa1, 0x2d, 0x09, 0x0c, 0x5b,
]);

coin_rejects!(too_few_flips, 16, [0u8; 127] => EntropyError::TooFewRolls(128, 127));
coin_rejects!(too_many_flips, 16, [0u8; 257] => EntropyError::TooManyRolls(256, 257));
coin_rejects!(unsupported_buffer_18, 18, [0u8; 144] => EntropyError::UnsupportedConfig(2, 18));

#[test]
fn deterministic_for_same_input() {
    let mut flips = [0u8; 128];
    for (i, f) in flips.iter_mut().enumerate() {
        *f = (i as u8) & 1;
    }
    let a: CoinEntropy<16> = coin_to_entropy(&flips).unwrap();
    let b: CoinEntropy<16> = coin_to_entropy(&flips).unwrap();
    assert_eq!(a.expose_secret(), b.expose_secret());
}

#[test]
fn order_sensitive() {
    let mut a = [0u8; 128];
    a[0] = 1;
    let mut b = [0u8; 128];
    b[127] = 1;
    let ea: CoinEntropy<16> = coin_to_entropy(&a).unwrap();
    let eb: CoinEntropy<16> = coin_to_entropy(&b).unwrap();
    assert_ne!(ea.expose_secret(), eb.expose_secret());
}

#[test]
fn at_max_length_accepted() {
    let flips = [1u8; 256];
    let _: CoinEntropy<16> = coin_to_entropy(&flips).unwrap();
}

#[test]
fn flip_value_two_rejected() {
    let mut flips = [0u8; 128];
    flips[10] = 2;
    let err = coin_to_entropy::<16>(&flips).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(2, 10));
}

#[test]
fn flip_value_255_rejected() {
    let mut flips = [0u8; 128];
    flips[5] = 255;
    let err = coin_to_entropy::<16>(&flips).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(255, 5));
}
