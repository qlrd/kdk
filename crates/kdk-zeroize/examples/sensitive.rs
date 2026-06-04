//! Demonstrates a probably [`SensitiveBytes`] lifecycle: construction,
//! redacted Debug output, drop-time wipe, and origin tagging.
//!
//! Run with:
//! ```text
//! cargo run --example sensitive_lifecycle -p kdk-zeroize
//! ```

use kdk_zeroize::prelude::*;

pub enum AesKey {}
pub enum SeedBytes {}

type AesKey256 = SensitiveBytes<32, AesKey>;
type Bip39Seed = SensitiveBytes<64, SeedBytes>;

use kdk_zeroize::wipe_in_place_mut;
use zeroize::Zeroize;

fn case<const N: usize>(label: &str, mut a: [u8; N], mut b: [u8; N]) {
    assert_eq!(a, b, "{label}: inputs differ pre-wipe");
    unsafe { wipe_in_place_mut(&mut a) };
    b.zeroize();
    assert_eq!(a, [0u8; N], "{label}: KDK didn't zero");
    assert_eq!(b, [0u8; N], "{label}: zeroize didn't zero");
    assert_eq!(a, b, "{label}: KDK and zeroize disagree");
    println!("  {label}: OK");
}

fn zeroize_wipe() {
    println!("kdk_zeroize::wipe_in_place_mut vs zeroize::Zeroize::zeroize");

    case("[u8; 32] all 0xAA", [0xAAu8; 32], [0xAAu8; 32]);
    case("[u8; 64] all 0xFF", [0xFFu8; 64], [0xFFu8; 64]);

    let mut non_uniform = [0u8; 64];
    for (i, x) in non_uniform.iter_mut().enumerate() {
        *x = (i as u8).wrapping_add(0xA5);
    }
    case("[u8; 64] non-uniform", non_uniform, non_uniform);

    // From here we will start a lot the `#[repr(C)]` attribute
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug, Zeroize)]
    struct Foo {
        a: u32,
        b: u64,
        c: [u8; 8],
    }
    let init = Foo {
        a: 0xDEADBEEF,
        b: 0xCAFEBABE_F00DBABE,
        c: [0xFFu8; 8],
    };
    let zero = Foo {
        a: 0,
        b: 0,
        c: [0u8; 8],
    };

    let mut kdk_foo = init;
    unsafe { wipe_in_place_mut(&mut kdk_foo) };
    assert_eq!(kdk_foo, zero, "KDK struct wipe");

    let mut zeroize_foo = init;
    zeroize_foo.zeroize();
    assert_eq!(zeroize_foo, zero, "zeroize struct wipe");

    assert_eq!(kdk_foo, zeroize_foo, "struct: KDK and zeroize disagree");
    println!("  Foo (struct): OK");

    println!("PASS: KDK and zeroize produce byte-identical output across all cases.");
}

fn detach() {
    let key = AesKey256::new([0xAA; 32]);
    let seed = Bip39Seed::new([0x42; 64]);

    println!("== Debug output is redacted ==");
    println!("AES key:    {key:?}");
    println!("BIP39 seed: {seed:?}");
    println!();

    println!("== Explicit access ==");
    println!("AES key first byte:    0x{:02X}", key.expose_secret()[0]);
    println!("BIP39 seed first byte: 0x{:02X}", seed.expose_secret()[0]);
    println!();

    let mut mixed = AesKey256::new([0xAA; 32]);
    let other = [0x55; 32];
    for (m, &o) in mixed.expose_secret_mut().iter_mut().zip(other.iter()) {
        *m ^= o;
    }
    println!("== After XOR mix (all bytes should be 0xFF) ==");
    println!(
        "First three: 0x{:02X} 0x{:02X} 0x{:02X}",
        mixed.expose_secret()[0],
        mixed.expose_secret()[1],
        mixed.expose_secret()[2]
    );
}

fn main() {
    zeroize_wipe();
    detach();
}
