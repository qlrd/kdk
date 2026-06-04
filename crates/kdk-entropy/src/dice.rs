use kdk_zeroize::prelude::*;
use sha2::{Digest, Sha256};

use crate::error::EntropyError;
use crate::utils::check_length;
use crate::DiceEntropy;

/// Minimum rolls required to fill an `N`-byte entropy buffer from a
/// `FACES`-die. Values match Krux upstream byte-for-byte.
///
/// For more, see [https://selfcustody.github.io/krux/getting-started/usage/generating-a-mnemonic/]
pub const fn min_rolls<const FACES: u8, const N: usize>() -> Result<usize, EntropyError> {
    match (FACES, N) {
        (6, 16) => Ok(50),
        (6, 32) => Ok(99),
        (20, 16) => Ok(30),
        (20, 32) => Ok(60),
        _ => Err(EntropyError::UnsupportedConfig(FACES, N)),
    }
}

/// Practical upper bound on roll count — 2× [`min_rolls`].
pub const fn max_rolls<const FACES: u8, const N: usize>() -> Result<usize, EntropyError> {
    match min_rolls::<FACES, N>() {
        Ok(m) => Ok(m * 2),
        Err(e) => Err(e),
    }
}

/// Fold a sequence of dice rolls into an `N`-byte entropy buffer,
/// byte-for-byte compatible with Krux upstream.
///
/// based on `src/krux/pages/new_mnemonic/dice_rolls.py`
/// rolls are formatted as a string (concatenated for `FACES < 10`,
/// hyphen-separated otherwise), SHA-256 hashed, then the first `N`
/// bytes of the digest become the entropy.
///
/// # Example
///
/// ```
/// use kdk_entropy::{dice_to_entropy, DiceEntropy};
/// use kdk_zeroize::prelude::*;
///
/// // 50 d6 rolls (all "1") → SHA-256("1"*50) truncated to 16 bytes.
/// // Same bytes Krux produces for the same input.
/// let rolls = [1u8; 50];
/// let e: DiceEntropy<6, 16> = dice_to_entropy(&rolls).unwrap();
/// assert_eq!(
///     e.expose_secret(),
///     &[
///         0x3d, 0xac, 0x51, 0xa6, 0x5e, 0xc9, 0xfc, 0xfc,
///         0x40, 0x9a, 0x1b, 0x5f, 0x1d, 0xef, 0xe9, 0x2b,
///     ]
/// );
/// ```
pub fn dice_to_entropy<const FACES: u8, const N: usize>(
    rolls: &[u8],
) -> Result<DiceEntropy<FACES, N>, EntropyError> {
    let required = min_rolls::<FACES, N>()?;
    let allowed = max_rolls::<FACES, N>()?;
    check_length(rolls.len(), required, allowed)?;

    // Validate before hashing — Krux relies on the UI to constrain
    // input, KDK enforces the contract explicitly.
    for (i, &roll) in rolls.iter().enumerate() {
        if roll == 0 || roll > FACES {
            return Err(EntropyError::RollOutOfRange(roll, i));
        }
    }

    let mut hasher = Sha256::new();
    if FACES < 10 {
        // d6 path: ASCII digits concatenated, no separator.
        for &roll in rolls {
            hasher.update([b'0' + roll]);
        }
    } else {
        // d20 path: variable-width decimals, hyphen-separated.
        for (i, &roll) in rolls.iter().enumerate() {
            if i > 0 {
                hasher.update(b"-");
            }
            if roll >= 10 {
                hasher.update([b'0' + roll / 10]);
            }
            hasher.update([b'0' + roll % 10]);
        }
    }
    let digest = hasher.finalize();

    let mut acc: DiceEntropy<FACES, N> = DiceEntropy::new([0u8; N]);
    acc.expose_secret_mut().copy_from_slice(&digest[..N]);
    Ok(acc)
}
