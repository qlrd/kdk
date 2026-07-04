use core::fmt;
use core::marker::PhantomData;

use bip39::Mnemonic;
use kdk_zeroize::Sensitive;

/// Sensitive BIP39 mnemonic, parameterised by origin marker `O` and
/// word count `W` (12 / 15 / 18 / 21 / 24).
///
/// The word count is a type-level marker only; the runtime value is
/// validated by the constructor in `*_mnemonic` pipelines.
pub struct SensitiveMnemonic<O, const W: u8> {
    inner: Mnemonic,
    _origin: PhantomData<O>,
}

impl<O, const W: u8> SensitiveMnemonic<O, W> {
    /// `pub(crate)` so callers go through the source-specific pipelines
    /// (`coin_mnemonic`, `dice_mnemonic`, `deck_mnemonic`) or the
    /// generic `entropy_to_mnemonic`, not through this constructor.
    pub(crate) fn new(inner: Mnemonic) -> Self {
        Self {
            inner,
            _origin: PhantomData,
        }
    }

    /// Inherent mirror of [`Sensitive::expose_secret`] so callers don't
    /// need to import the trait just to read the inner mnemonic.
    pub fn expose_secret(&self) -> &Mnemonic {
        &self.inner
    }
}

impl<O, const W: u8> fmt::Debug for SensitiveMnemonic<O, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SensitiveMnemonic(REDACTED)")
    }
}

impl<O, const W: u8> Sensitive for SensitiveMnemonic<O, W> {
    type Inner = Mnemonic;

    fn expose_secret(&self) -> &Mnemonic {
        &self.inner
    }
}
