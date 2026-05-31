//! The kdk-zeroize guarantee can't be observed from outside the
//! process.
//!
//! Rust's memory model doesn't expose dropped storage so these tests are an
//! attempt to observe how probably a pin could occurs on through a
//! `expose_secret`, `Debug` redaction, `Drop` runs without panic,
//! `as_slice` agreement, etc., rather than the wipe itself.
//!
//! The wipe correctness is delegated to whoever wants to audit this crate and
//! we be welcomed through a [`SECURITY.md`] report.
use kdk_zeroize::SensitiveBytes;

enum AesKey {}
enum Seed {}

type AesKey256 = SensitiveBytes<32, AesKey>;
type Bip39Seed = SensitiveBytes<64, Seed>;

#[test]
fn new_and_expose_secret_round_trip() {
    let bytes = [0xAA; 32];
    let key = AesKey256::new(bytes);
    assert_eq!(key.expose_secret(), &bytes);
}

#[test]
fn as_slice_matches_expose_secret() {
    let key = AesKey256::new([1u8; 32]);
    assert_eq!(key.as_slice(), &key.expose_secret()[..]);
}

#[test]
fn debug_impl_redacts_bytes() {
    let key = AesKey256::new([0xFF; 32]);
    let dbg = format!("{key:?}");
    assert_eq!(dbg, "SensitiveBytes(REDACTED)");
}

#[test]
fn debug_does_not_contain_any_byte_values() {
    let mut bytes = [0u8; 32];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = i as u8;
    }
    let key = AesKey256::new(bytes);
    let dbg = format!("{key:?}");
    assert!(!dbg.contains("31"), "byte 31 leaked: {dbg}");
    assert!(!dbg.contains("0x"), "hex prefix leaked: {dbg}");
}

#[test]
fn expose_secret_mut_allows_in_place_xor() {
    let mut mixed = AesKey256::new([0xAA; 32]);
    let other = [0x55; 32];
    for (m, &o) in mixed.expose_secret_mut().iter_mut().zip(other.iter()) {
        *m ^= o;
    }
    assert_eq!(mixed.expose_secret(), &[0xFF; 32]);
}

#[test]
fn origin_markers_are_distinct_types() {
    let _key: AesKey256 = SensitiveBytes::new([0; 32]);
    let _seed: Bip39Seed = SensitiveBytes::new([0; 64]);
    // assert_ne!(_key, _seed); // not compile
}

#[test]
fn fits_inside_a_struct_field() {
    struct Wallet {
        seed: Bip39Seed,
    }
    let w = Wallet {
        seed: SensitiveBytes::new([7u8; 64]),
    };
    assert_eq!(w.seed.expose_secret()[0], 7);
}
