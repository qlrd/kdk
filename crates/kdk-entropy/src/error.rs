use core::fmt;

/// Errors returned by [`crate::dice_to_entropy`] and friends.
///
/// Manual `Display` (no `thiserror`) so the error messages stay auditable
/// and don't accidentally inherit upstream formatting that could leak
/// secret bytes.
#[derive(Debug, PartialEq, Eq)]
pub enum EntropyError {
    /// Caller supplied fewer rolls than `min_rolls::<FACES, N>()` requires.
    /// Fields: `(required, given)`.
    TooFewRolls(usize, usize),

    /// Caller supplied more rolls than `max_rolls::<FACES, N>()` permits.
    /// Fields: `(allowed, given)`.
    TooManyRolls(usize, usize),

    /// A roll byte was outside the valid range `1..=FACES`.
    /// Fields: `(value, index)`.
    RollOutOfRange(u8, usize),

    /// The `(FACES, N)` generic combination is not one of the supported
    /// dice configurations (d6/d20 × 16/32 bytes). Fields: `(FACES, N)`.
    UnsupportedConfig(u8, usize),

    /// A card byte appeared twice in the input — only relevant to
    /// `deck_to_entropy`. Fields: `(value, second_index)`.
    DuplicateCard(u8, usize),
}

impl fmt::Display for EntropyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntropyError::TooFewRolls(required, given) => {
                write!(f, "too few rolls: required {required}, got {given}")
            }
            EntropyError::TooManyRolls(allowed, given) => {
                write!(
                    f,
                    "too many rolls: at most {allowed} permitted, got {given}"
                )
            }
            EntropyError::RollOutOfRange(value, index) => {
                write!(
                    f,
                    "roll at index {index} has value {value}: out of valid range"
                )
            }
            EntropyError::UnsupportedConfig(faces, n) => {
                write!(
                    f,
                    "unsupported (FACES, N) configuration: ({faces}, {n}) — supported: d6/d20 x 16/32 bytes"
                )
            }
            EntropyError::DuplicateCard(value, index) => {
                write!(f, "card value {value} at index {index} was already drawn")
            }
        }
    }
}

impl core::error::Error for EntropyError {}
