//! Behavioural tests: `expose_secret`, `Debug` redaction, origin markers, etc.
//!
//! Drop-time wipe proof lives in `tests/amnesia.rs` (`Box::into_raw` →
//! `drop_in_place` → volatile byte check → `dealloc`).
use kdk_zeroize::prelude::*;
use kdk_zeroize::wipe_in_place_mut;

enum AesKey {}
enum Seed {}

type AesKey256 = SensitiveBytes<32, AesKey>;
type Bip39Seed = SensitiveBytes<64, Seed>;

fn read<S: Sensitive>(s: &S) -> &S::Inner {
    s.expose_secret()
}

fn write<S: SensitiveMut>(s: &mut S) -> &mut S::Inner {
    s.expose_secret_mut()
}

fn read_mut<S: SensitiveMut>(s: &S) -> &S::Inner {
    s.expose_secret() // method from `Sensitive`, available because SensitiveMut: Sensitive
}

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

#[test]
fn wipe_in_place_mut_zeroes_u8_array_32() {
    let mut buf = [0xAAu8; 32];
    unsafe { wipe_in_place_mut(&mut buf) };
    assert_eq!(buf, [0u8; 32]);
}

#[test]
fn wipe_in_place_mut_zeroes_u8_array_64_non_uniform() {
    let mut buf = [0u8; 64];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_add(0xA5);
    }
    unsafe { wipe_in_place_mut(&mut buf) };
    assert_eq!(buf, [0u8; 64]);
}

#[test]
fn wipe_in_place_mut_zeroes_arbitrary_struct() {
    #[repr(C)]
    struct Foo {
        a: u32,
        b: u64,
        c: [u8; 8],
    }
    let mut foo = Foo {
        a: 0xDEADBEEF,
        b: 0xCAFEBABE_F00DBABE,
        c: [0xFFu8; 8],
    };
    assert_ne!(foo.a, 0);
    assert_ne!(foo.b, 0);
    assert_ne!(foo.c, [0u8; 8]);
    unsafe { wipe_in_place_mut(&mut foo) };
    assert_eq!(foo.a, 0);
    assert_eq!(foo.b, 0);
    assert_eq!(foo.c, [0u8; 8]);
}

#[test]
fn wipe_in_place_mut_zeroes_sensitive_bytes() {
    let mut value = SensitiveBytes::<32, AesKey>::new([0xDEu8; 32]);
    unsafe { wipe_in_place_mut(&mut value) };
    assert_eq!(value.expose_secret(), &[0u8; 32]);
}

#[test]
fn same_bytes_as_inherent() {
    let s = SensitiveBytes::<32, AesKey>::new([0x42; 32]);
    let direct = s.expose_secret();
    let via = read(&s);
    assert_eq!(direct, via);
    assert_eq!(direct[0], 0x42);
}

#[test]
fn persists_mutation() {
    let mut s = SensitiveBytes::<32, AesKey>::new([0u8; 32]);
    let buf = write(&mut s);
    buf[0] = 0xFF;
    buf[31] = 0x11;
    assert_eq!(s.expose_secret()[0], 0xFF);
    assert_eq!(s.expose_secret()[31], 0x11);
}

#[test]
fn mut_is_subtrait_of_sensitive() {
    let s = SensitiveBytes::<32, AesKey>::new([0x37; 32]);
    let buf = read_mut(&s);
    assert_eq!(buf[0], 0x37);
}

#[test]
fn resolves_to_byte_array() {
    fn assert_inner_is<S: Sensitive<Inner = [u8; 32]>>(_s: &S) {}
    let s = SensitiveBytes::<32, AesKey>::new([0; 32]);
    assert_inner_is(&s);
}

#[test]
fn markers_compose_with_trait() {
    let aes = SensitiveBytes::<32, AesKey>::new([0xAA; 32]);
    let seed = SensitiveBytes::<64, Seed>::new([0x55; 64]);
    assert_eq!(read(&aes).len(), 32);
    assert_eq!(read(&seed).len(), 64);
}
