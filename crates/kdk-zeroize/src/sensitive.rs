use core::fmt;
use core::marker::PhantomData;

use crate::wipe_in_place_mut;

/// `N` is the byte length and `O` is a chosen zero-sized type that lets the
/// type system track where the bytes came from (BIP39 seed, dice entropy, AES
/// key, decrypted KEF payload, etc...).
///
/// # Example
///
/// ```
/// use kdk_zeroize::SensitiveBytes;
///
/// // A caller-chosen marker type.
/// pub enum AesKey {}
///
/// type AesKey256 = SensitiveBytes<32, AesKey>;
///
///
/// let key = AesKey256::new([0u8; 32]);
/// ```
pub struct SensitiveBytes<const N: usize, O> {
    bytes: [u8; N],
    _origin: PhantomData<O>,
}

impl<const N: usize, O> SensitiveBytes<N, O> {
    /// Wrap a `[u8; N]` as origin-tagged sensitive bytes.
    pub const fn new(bytes: [u8; N]) -> Self {
        Self {
            bytes,
            _origin: PhantomData,
        }
    }

    /// Explicit-secret-access getter. Prefer this over `as_slice`
    /// when the bytes are about to be fed into a cryptographic
    /// primitive — the name makes the secrecy visible in diffs.
    pub fn expose_secret(&self) -> &[u8; N] {
        &self.bytes
    }

    /// Mutable explicit-secret-access getter
    pub fn expose_secret_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }

    /// Same bytes as `expose_secret`; the looser name is fine for
    /// non-cryptographic reads (e.g. equality checks in tests).
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize, O> Drop for SensitiveBytes<N, O> {
    fn drop(&mut self) {
        unsafe { wipe_in_place_mut(self) };
    }
}

impl<const N: usize, O> fmt::Debug for SensitiveBytes<N, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SensitiveBytes(REDACTED)")
    }
}
