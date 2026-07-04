use kdk_entropy::{deck_to_entropy, Deck, EntropyError};
use kdk_zeroize::SensitiveBytes;

use crate::error::MnemonicError;
use crate::game::{GameMnemonic, SensitiveGame};

/// Card-draw BIP39 mnemonic. `CARDS` is the deck size (e.g. 52);
/// `W` is the word count `in [12, 15, 18, 21, 24]`.
pub type DeckMnemonic<const CARDS: u8, const W: u8> = GameMnemonic<Deck<CARDS>, W>;

impl<const CARDS: u8> SensitiveGame for Deck<CARDS> {
    fn to_entropy<const N: usize>(input: &[u8]) -> Result<SensitiveBytes<N, Self>, EntropyError> {
        deck_to_entropy::<CARDS, N>(input)
    }
}

/// Card draws to BIP39 mnemonic. `CARDS` is the deck size and `W` the
/// word count; the matching `words-<W>` Cargo feature must be enabled.
///
/// Requires the `deck` and `words-<W>` features.
///
/// # Example
///
/// ```
/// use kdk_mnemonic::{deck_mnemonic, DeckMnemonic};
///
/// let cards: [u8; 25] = [
///     0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
///     10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
///     20, 21, 22, 23, 24,
/// ];
/// let _: DeckMnemonic<52, 12> = deck_mnemonic::<52, 12>(&cards).unwrap();
/// ```
pub fn deck_mnemonic<const CARDS: u8, const W: u8>(
    cards: &[u8],
) -> Result<DeckMnemonic<CARDS, W>, MnemonicError> {
    <Deck<CARDS> as SensitiveGame>::mnemonic::<W>(cards)
}
