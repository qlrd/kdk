//! Functional test: kdk-mnemonic produces seeds matching the canonical
//! Trezor BIP39 reference vectors byte-for-byte.
//!
//! Vectors below pinned from
//! `github.com/trezor/python-mnemonic/blob/master/vectors.json`
//! (English section, passphrase `"TREZOR"`). That file is the
//! de-facto BIP39 reference; matching it proves end-to-end correctness
//! of the kdk pipeline (entropy → mnemonic → PBKDF2 → 64-byte seed)
//! against an independent implementation.

use kdk_mnemonic::{entropy_to_mnemonic, GameMnemonic};
use kdk_zeroize::{prelude::*, SensitiveBytes};

enum Vector {}

const fn hex_nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("invalid hex char"),
    }
}

const fn from_hex<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    assert!(bytes.len() == 2 * N, "hex length mismatch");
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        out[i] = (hex_nibble(bytes[2 * i]) << 4) | hex_nibble(bytes[2 * i + 1]);
        i += 1;
    }
    out
}

fn kdk_seed<const W: u8, const N: usize>(entropy_bytes: [u8; N]) -> [u8; 64] {
    let entropy = SensitiveBytes::<N, Vector>::new(entropy_bytes);
    let mnemonic: GameMnemonic<Vector, W> =
        entropy_to_mnemonic(&entropy).expect("entropy length matches W");
    mnemonic.expose_secret().to_seed("TREZOR")
}

macro_rules! trezor_vector {
    ($name:ident, $w:literal, $n:literal, $entropy_hex:expr, $seed_hex:expr) => {
        #[test]
        #[cfg(all(feature = "words-12", feature = "words-24"))]
        fn $name() {
            let entropy: [u8; $n] = from_hex($entropy_hex);
            let expected_seed: [u8; 64] = from_hex($seed_hex);
            let got = kdk_seed::<$w, $n>(entropy);
            assert_eq!(got, expected_seed, "seed mismatch vs Trezor reference");
        }
    };
}

// 12-word vectors (16-byte entropy)
trezor_vector!(
    zeros_12_word,
    12,
    16,
    "00000000000000000000000000000000",
    "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04"
);
trezor_vector!(
    ones_12_word,
    12,
    16,
    "ffffffffffffffffffffffffffffffff",
    "ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069"
);

// 24-word vectors (32-byte entropy)
trezor_vector!(
    zeros_24_word,
    24,
    32,
    "0000000000000000000000000000000000000000000000000000000000000000",
    "bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8"
);
trezor_vector!(
    ones_24_word,
    24,
    32,
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    "dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad"
);

#[test]
#[cfg(feature = "words-12")]
fn distinct_inputs_produce_distinct_seeds() {
    let zeros = kdk_seed::<12, 16>([0u8; 16]);
    let ones = kdk_seed::<12, 16>([0xFFu8; 16]);
    assert_ne!(zeros, ones, "zeros and ones produced the same seed");
}

#[test]
#[cfg(feature = "words-12")]
fn deterministic_for_same_input() {
    let a = kdk_seed::<12, 16>([0u8; 16]);
    let b = kdk_seed::<12, 16>([0u8; 16]);
    assert_eq!(a, b, "kdk seed pipeline is non-deterministic");
}
