use core::fmt;
use core::marker::PhantomData;

use bip39::Mnemonic;
use kdk_entropy::EntropyError;
use kdk_zeroize::{prelude::*, SensitiveBytes};

use crate::error::MnemonicError;

/// Sensitive BIP39 mnemonic wrapper. `O` is a zero-sized origin marker
/// (e.g. [`kdk_entropy::Coin`], [`kdk_entropy::Dice<F>`],
/// [`kdk_entropy::Deck<C>`]) and `W` is the word count
/// (`12 / 15 / 18 / 21 / 24`).
pub struct GameMnemonic<O, const W: u8> {
    inner: Mnemonic,
    _origin: PhantomData<O>,
}

impl<O, const W: u8> GameMnemonic<O, W> {
    /// per-game pipelines (`coin_mnemonic`, `dice_mnemonic`, `deck_mnemonic`)
    /// or the generic [`entropy_to_mnemonic`].
    pub(crate) fn new(inner: Mnemonic) -> Self {
        Self {
            inner,
            _origin: PhantomData,
        }
    }

    /// Word count of this mnemonic — returned from the const generic
    /// `W` without touching the inner secret. Use this instead of
    /// `expose_secret().word_count()` whenever you just need the
    /// metadata.
    pub const fn word_count(&self) -> usize {
        W as usize
    }
}

impl<O, const W: u8> fmt::Debug for GameMnemonic<O, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GameMnemonic(REDACTED)")
    }
}

impl<O, const W: u8> fmt::Display for GameMnemonic<O, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GameMnemonic(REDACTED)")
    }
}

impl<O, const W: u8> Sensitive for GameMnemonic<O, W> {
    type Inner = Mnemonic;

    fn expose_secret(&self) -> &Mnemonic {
        &self.inner
    }
}

/// Contract for any entropy-source ("game") that can produce a BIP39
/// mnemonic. Implementers supply only [`to_entropy`]. Per-`W`.
pub trait SensitiveGame: Sized {
    /// Game-specific entropy function: raw input bytes → `N`-byte
    /// origin-tagged [`SensitiveBytes`].
    fn to_entropy<const N: usize>(input: &[u8]) -> Result<SensitiveBytes<N, Self>, EntropyError>;

    /// Feature-gated per-`W`, then build the mnemonic.
    /// Each game shares this code path; only the byte size per `W` differs.
    fn mnemonic<const W: u8>(input: &[u8]) -> Result<GameMnemonic<Self, W>, MnemonicError> {
        match W {
            #[cfg(feature = "words-12")]
            12 => Self::build::<W, 16>(input),
            #[cfg(feature = "words-15")]
            15 => Self::build::<W, 20>(input),
            #[cfg(feature = "words-18")]
            18 => Self::build::<W, 24>(input),
            #[cfg(feature = "words-21")]
            21 => Self::build::<W, 28>(input),
            #[cfg(feature = "words-24")]
            24 => Self::build::<W, 32>(input),
            _ => Err(MnemonicError::InvalidWordCount(W)),
        }
    }

    /// Entropy with explicit `N` to [GameMnemonic<Self, W>].
    fn build<const W: u8, const N: usize>(
        input: &[u8],
    ) -> Result<GameMnemonic<Self, W>, MnemonicError> {
        let entropy = Self::to_entropy::<N>(input)?;
        Mnemonic::from_entropy(entropy.expose_secret())
            .map(GameMnemonic::new)
            .map_err(MnemonicError::Bip39)
    }
}

/// Pre-built entropy bytes sets to a BIP39 [GameMnemonic<O,W>]. Validates that
/// `W` is enabled and that `N` matches its BIP39 byte count, then wraps the
/// produced [bip39::Mnemonic].
pub fn entropy_to_mnemonic<O, const W: u8, const N: usize>(
    entropy: &SensitiveBytes<N, O>,
) -> Result<GameMnemonic<O, W>, MnemonicError> {
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
    Mnemonic::from_entropy(entropy.expose_secret())
        .map(GameMnemonic::new)
        .map_err(MnemonicError::Bip39)
}
