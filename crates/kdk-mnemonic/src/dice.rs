use kdk_entropy::{dice_to_entropy, Dice, EntropyError};
use kdk_zeroize::SensitiveBytes;

use crate::error::MnemonicError;
use crate::game::{GameMnemonic, SensitiveGame};

/// Dice-roll BIP39 mnemonic. `FACES` is the die size (e.g. 6, 20);
/// `W` is the word count `in [12, 15, 18, 21, 24]`.
pub type DiceMnemonic<const FACES: u8, const W: u8> = GameMnemonic<Dice<FACES>, W>;

impl<const FACES: u8> SensitiveGame for Dice<FACES> {
    fn to_entropy<const N: usize>(input: &[u8]) -> Result<SensitiveBytes<N, Self>, EntropyError> {
        dice_to_entropy::<FACES, N>(input)
    }
}

/// Dice rolls to BIP39 mnemonic. `FACES` is the die size and `W` the
/// word count; the matching `words-<W>` Cargo feature must be enabled.
///
/// Requires the `dice` and `words-<W>` features.
///
/// # Example
///
/// ```
/// use kdk_mnemonic::{dice_mnemonic, DiceMnemonic};
///
/// let rolls = [1u8; 50];
/// let _: DiceMnemonic<6, 12> = dice_mnemonic::<6, 12>(&rolls).unwrap();
/// ```
pub fn dice_mnemonic<const FACES: u8, const W: u8>(
    rolls: &[u8],
) -> Result<DiceMnemonic<FACES, W>, MnemonicError> {
    <Dice<FACES> as SensitiveGame>::mnemonic::<W>(rolls)
}
