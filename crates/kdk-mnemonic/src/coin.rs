use kdk_entropy::{coin_to_entropy, Coin, EntropyError};
use kdk_zeroize::SensitiveBytes;

use crate::error::MnemonicError;
use crate::game::{GameMnemonic, SensitiveGame};

/// Coin-flip BIP39 mnemonic.
pub type CoinMnemonic<const W: u8> = GameMnemonic<Coin, W>;

impl SensitiveGame for Coin {
    fn to_entropy<const N: usize>(input: &[u8]) -> Result<SensitiveBytes<N, Self>, EntropyError> {
        coin_to_entropy::<N>(input)
    }
}

/// Coin flips to BIP39 mnemonic. `W` selects the word count and must
/// have a matching `words-<W>` Cargo feature enabled.
///
/// Each `W` count maps to a coin-entropy byte length `N` (`N = W * 4 / 3`):
///
/// - `coin_mnemonic::<12>(&flips) -> coin_to_entropy::<16>(&flips)`: 128 flips
/// - `coin_mnemonic::<15>(&flips) -> coin_to_entropy::<20>(&flips)`: 160 flips
/// - `coin_mnemonic::<18>(&flips) -> coin_to_entropy::<24>(&flips)`: 192 flips
/// - `coin_mnemonic::<21>(&flips) -> coin_to_entropy::<28>(&flips)`: 224 flips
/// - `coin_mnemonic::<24>(&flips) -> coin_to_entropy::<32>(&flips)`: 256 flips
///
/// # Example
///
/// ```
/// use kdk_mnemonic::{coin_mnemonic, CoinMnemonic};
///
/// let flips = [0u8; 128];
/// let _: CoinMnemonic<12> = coin_mnemonic::<12>(&flips).unwrap();
/// ```
pub fn coin_mnemonic<const W: u8>(flips: &[u8]) -> Result<CoinMnemonic<W>, MnemonicError> {
    Coin::mnemonic::<W>(flips)
}
