use sha2::{Digest, Sha256};

use crate::error::EntropyError;
use crate::utils::check_length;
use crate::CoinEntropy;

/// Minimum flips required to fill an `N`-byte entropy buffer with a
/// fair coin.
pub const fn min_flips<const N: usize>() -> Result<usize, EntropyError> {
    match N {
        16 => Ok(128),
        32 => Ok(256),
        _ => Err(EntropyError::UnsupportedConfig(2, N)),
    }
}

/// Practical upper bound on flip count — 2× [`min_flips`].
pub const fn max_flips<const N: usize>() -> Result<usize, EntropyError> {
    match min_flips::<N>() {
        Ok(m) => Ok(m * 2),
        Err(e) => Err(e),
    }
}

/// Fold a sequence of coin flips into an `N`-byte entropy buffer.
///
/// Flips are formatted as a concatenated string of `"0"` / `"1"` ASCII
/// digits, SHA-256 hashed, then the first `N` bytes of the digest are
/// taken. Mirrors Krux's dice-string approach for `FACES < 10`.
///
/// # Example
///
/// ```
/// use kdk_entropy::{coin_to_entropy, CoinEntropy};
///
/// // SHA-256("0" * 128)[:16].
/// let flips = [0u8; 128];
/// let e: CoinEntropy<16> = coin_to_entropy(&flips).unwrap();
/// assert_eq!(
///     e.expose_secret(),
///     &[
///         0x45, 0x72, 0x57, 0x91, 0xc4, 0x7b, 0x32, 0x61,
///         0x8c, 0xc5, 0x7b, 0x88, 0x34, 0x3e, 0x2b, 0xce,
///     ]
/// );
/// ```
pub fn coin_to_entropy<const N: usize>(flips: &[u8]) -> Result<CoinEntropy<N>, EntropyError> {
    let required = min_flips::<N>()?;
    let allowed = max_flips::<N>()?;
    check_length(flips.len(), required, allowed)?;

    for (i, &flip) in flips.iter().enumerate() {
        if flip > 1 {
            return Err(EntropyError::RollOutOfRange(flip, i));
        }
    }

    let mut hasher = Sha256::new();
    for &flip in flips {
        hasher.update([b'0' + flip]);
    }
    let digest = hasher.finalize();

    let mut acc: CoinEntropy<N> = CoinEntropy::new([0u8; N]);
    acc.expose_secret_mut().copy_from_slice(&digest[..N]);
    Ok(acc)
}
