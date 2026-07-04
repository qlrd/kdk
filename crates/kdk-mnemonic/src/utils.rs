use bip39::Mnemonic;
use kdk_zeroize::SensitiveBytes;

use crate::error::MnemonicError;
use crate::mnemonic::SensitiveMnemonic;

/// Internal helper used by every `*_mnemonic` source pipeline once the
/// per-`W` match arm has already picked the right byte length.
pub(crate) fn build<O, const W: u8, const N: usize>(
    entropy: &SensitiveBytes<N, O>,
) -> Result<SensitiveMnemonic<O, W>, MnemonicError> {
    Mnemonic::from_entropy(entropy.expose_secret())
        .map(SensitiveMnemonic::new)
        .map_err(MnemonicError::Bip39)
}

/// Convert pre-built entropy bytes into a BIP39 mnemonic. Generic over
/// origin marker `O`, word count `W`, and byte length `N` — validates
/// at runtime that `W` is enabled and `N` matches its BIP39 byte count.
pub fn entropy_to_mnemonic<O, const W: u8, const N: usize>(
    entropy: &SensitiveBytes<N, O>,
) -> Result<SensitiveMnemonic<O, W>, MnemonicError> {
    let expected = match W {
        #[cfg(feature = "words-12")]
        12 => 16,
        #[cfg(feature = "words-15")]
        15 => 20,
        #[cfg(feature = "words-18")]
        18 => 24,
        #[cfg(feature = "words-21")]
        21 => 28,
        #[cfg(feature = "words-24")]
        24 => 32,
        _ => return Err(MnemonicError::InvalidWordCount(W)),
    };
    if N != expected {
        return Err(MnemonicError::InvalidEntropyLength(N));
    }
    build(entropy)
}
