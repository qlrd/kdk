use kdk_zeroize::prelude::*;
use sha2::{Digest, Sha256};

use crate::error::EntropyError;
use crate::utils::check_length;
use crate::DeckEntropy;

/// Minimum card draws required to fill an `N`-byte entropy buffer
/// from a `CARDS`-deck. Each entry is the smallest `k` such that
/// `log₂(CARDS! / (CARDS-k)!) ≥ 8·N`.
pub const fn min_draws<const CARDS: u8, const N: usize>() -> Result<usize, EntropyError> {
    match (CARDS, N) {
        (40, 16) => Ok(28),
        (48, 16) => Ok(26),
        (52, 16) => Ok(25),
        (58, 16) => Ok(24),
        (58, 32) => Ok(55),
        (78, 16) => Ok(22),
        (78, 32) => Ok(45),
        (108, 16) => Ok(22),
        (108, 32) => Ok(41),
        (112, 16) => Ok(22),
        (112, 32) => Ok(42),
        _ => Err(EntropyError::UnsupportedConfig(CARDS, N)),
    }
}

/// Practical upper bound on draw count — `2 * min_draws`, capped at `CARDS`.
pub const fn max_draws<const CARDS: u8, const N: usize>() -> Result<usize, EntropyError> {
    match min_draws::<CARDS, N>() {
        Ok(m) => {
            let doubled = m * 2;
            let cap = CARDS as usize;
            Ok(if doubled > cap { cap } else { doubled })
        }
        Err(e) => Err(e),
    }
}

/// Fold a sequence of drawn cards into an `N`-byte entropy buffer.
///
/// Cards are formatted as decimal indices, hyphen-separated, SHA-256
/// hashed, then truncated to `N` bytes. Mirrors Krux's d20 string
/// shape (`"-".join`) for multi-digit values.
///
/// # Example
///
/// ```
/// use kdk_entropy::{deck_to_entropy, DeckEntropy};
/// use kdk_zeroize::prelude::*;
///
/// // SHA-256("0-1-2-3-...-24")[:16].
/// let cards: [u8; 25] = [
///     0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
///     10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
///     20, 21, 22, 23, 24,
/// ];
/// let e: DeckEntropy<52, 16> = deck_to_entropy(&cards).unwrap();
/// assert_eq!(
///     e.expose_secret(),
///     &[
///         0x92, 0xe2, 0x92, 0xe3, 0x4c, 0x44, 0x48, 0xc1,
///         0x57, 0x73, 0x1a, 0xff, 0x3d, 0x40, 0x10, 0x38,
///     ]
/// );
/// ```
pub fn deck_to_entropy<const CARDS: u8, const N: usize>(
    cards: &[u8],
) -> Result<DeckEntropy<CARDS, N>, EntropyError> {
    let required = min_draws::<CARDS, N>()?;
    let allowed = max_draws::<CARDS, N>()?;
    check_length(cards.len(), required, allowed)?;

    // Validation pass: range + uniqueness. `present[i]` is non-secret
    // bookkeeping for which indices remain undrawn.
    let mut present = [true; 256];
    for (i, &card) in cards.iter().enumerate() {
        if card >= CARDS {
            return Err(EntropyError::RollOutOfRange(card, i));
        }
        if !present[card as usize] {
            return Err(EntropyError::DuplicateCard(card, i));
        }
        present[card as usize] = false;
    }

    let mut hasher = Sha256::new();
    for (i, &card) in cards.iter().enumerate() {
        if i > 0 {
            hasher.update(b"-");
        }
        if card >= 100 {
            hasher.update([b'0' + card / 100]);
        }
        if card >= 10 {
            hasher.update([b'0' + (card / 10) % 10]);
        }
        hasher.update([b'0' + card % 10]);
    }
    let digest = hasher.finalize();

    let mut acc: DeckEntropy<CARDS, N> = DeckEntropy::new([0u8; N]);
    acc.expose_secret_mut().copy_from_slice(&digest[..N]);
    Ok(acc)
}
