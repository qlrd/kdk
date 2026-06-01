use crate::error::EntropyError;

/// Validate `actual ∈ [required, allowed]`. Returns the appropriate
/// `EntropyError` on out-of-range.
#[inline]
pub(crate) fn check_length(
    actual: usize,
    required: usize,
    allowed: usize,
) -> Result<(), EntropyError> {
    if actual < required {
        return Err(EntropyError::TooFewRolls(required, actual));
    }
    if actual > allowed {
        return Err(EntropyError::TooManyRolls(allowed, actual));
    }
    Ok(())
}
