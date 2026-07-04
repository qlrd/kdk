use core::fmt;

use kdk_entropy::EntropyError;

/// Errors returned by the conversion functions.
#[derive(Debug)]
pub enum MnemonicError {
    /// `W` is not a BIP-0039 word count (12/15/18/21/24) or the
    /// corresponding `words-<W>` Cargo feature is off.
    InvalidWordCount(u8),

    /// Entropy length is not enabled by the build's feature set.
    /// Reachable via [`crate::entropy_to_mnemonic`] for raw
    /// `SensitiveBytes` paths.
    InvalidEntropyLength(usize),

    /// Underlying `bip39` crate rejected the input. Stored for
    /// pattern-matching but NEVER formatted via `Display`/`Debug` —
    /// upstream impls may include user-supplied bytes.
    Bip39(bip39::Error),

    /// `kdk-entropy` error propagated from a `*_mnemonic` pipeline.
    Entropy(EntropyError),
}

impl fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MnemonicError::InvalidWordCount(w) => {
                write!(f, "invalid or feature-gated BIP39 word count: {w}")
            }
            MnemonicError::InvalidEntropyLength(n) => {
                write!(
                    f,
                    "invalid or feature-gated BIP39 entropy length: {n} bytes"
                )
            }
            MnemonicError::Bip39(_) => write!(f, "bip39 mnemonic conversion failed"),
            MnemonicError::Entropy(e) => write!(f, "{e}"),
        }
    }
}

impl From<EntropyError> for MnemonicError {
    fn from(e: EntropyError) -> Self {
        MnemonicError::Entropy(e)
    }
}

impl core::error::Error for MnemonicError {}
