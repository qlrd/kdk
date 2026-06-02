//! Amnesia verification — proves `wipe_in_place_mut` zeroes the bytes
//! of whatever `&mut T` you hand it.
//!
//! `SensitiveBytes::drop` is a one-line delegation
//! (`unsafe { wipe_in_place_mut(self) }`), so the Drop-time wipe is
//! guaranteed by *reading the source*, not by a test. These tests
//! prove the primitive itself.

use kdk_zeroize::{wipe_in_place_mut, SensitiveBytes};

enum AesKey {}

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
