use core::sync::atomic;

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
