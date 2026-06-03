use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic;

/// Disciplines this trait does NOT enforce — see AGENTS.md §4.1 / §4.4:
/// - `Drop` wipes the storage on out-of-scope.
/// - Redacting `fmt::Debug` impl (never derive).
/// - No `Clone` / no `Copy`.
pub trait Sensitive {
    /// The type whose bytes are the secret material.
    type Inner: ?Sized;

    /// Explicit-secret-access getter. Mirrors the inherent
    /// `expose_secret` every sensitive type provides.
    fn expose_secret(&self) -> &Self::Inner;
}

/// Read-only sensitive types (computed/derived values like a wrapped
/// `bip39::Mnemonic`) impl only [`Sensitive`], not this trait —
/// generic code that requires mutation takes `<S: SensitiveMut>` and
/// will refuse those at compile time.
pub trait SensitiveMut: Sensitive {
    /// Mutable explicit-secret-access getter.
    fn expose_secret_mut(&mut self) -> &mut Self::Inner;
}

/// Caller must ensure that after this function returns, **no code
/// reads `*value` as a `T`**.
///
/// # Safety
///
/// Zeroing the bytes may produce an invalid `T` (niches, validity invariants
/// on inner fields, etc.). Typically used inside `Drop`, where Rust's drop
/// semantics already guarantee no further reads.
///
/// WARN: authorship is from [@luisschwab](https://github.com/luisschwab)
/// while reading a go discord's topic. It's improvised and need auction.
pub unsafe fn wipe_in_place_mut<T>(value: &mut T) {
    let ptr = (value as *mut T).cast::<u8>();
    let len = core::mem::size_of::<T>();
    for i in 0..len {
        unsafe {
            core::ptr::write_volatile(ptr.add(i), 0);
        }
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

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

    /// Same bytes as `Sensitive::expose_secret`; the looser `&[u8]`
    /// return is fine for non-cryptographic reads (e.g. equality
    /// checks in tests).
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

impl<const N: usize, O> Sensitive for SensitiveBytes<N, O> {
    type Inner = [u8; N];

    fn expose_secret(&self) -> &[u8; N] {
        &self.bytes
    }
}

impl<const N: usize, O> SensitiveMut for SensitiveBytes<N, O> {
    fn expose_secret_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }
}
