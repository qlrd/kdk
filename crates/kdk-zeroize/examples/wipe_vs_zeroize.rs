//! Integration test with `zeroize` crate.
//!
//! Run with:
//! ```text
//! cargo run --example wipe_vs_zeroize -p kdk-zeroize --release
//! ```
//!
//! `--release` is intentional: that's where the optimiser is most
//! aggressive about dead-store elimination.

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

fn main() {
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
