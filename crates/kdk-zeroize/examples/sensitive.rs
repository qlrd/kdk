//! Demonstrates a probably [`SensitiveBytes`] lifecycle: construction,
//! redacted Debug output, drop-time wipe, and origin tagging.
//!
//! Run with:
//! ```text
//! cargo run --example sensitive_lifecycle -p kdk-zeroize
//! ```

use kdk_zeroize::SensitiveBytes;

pub enum AesKey {}
pub enum SeedBytes {}

type AesKey256 = SensitiveBytes<32, AesKey>;
type Bip39Seed = SensitiveBytes<64, SeedBytes>;

fn main() {
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
