use kdk_entropy::{dice_to_entropy, max_rolls, min_rolls, DiceEntropy, EntropyError};

macro_rules! min_rolls_ok {
    ($name:ident, $faces:literal, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(min_rolls::<$faces, $n>(), Ok($expected));
        }
    };
}

macro_rules! max_rolls_ok {
    ($name:ident, $faces:literal, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(max_rolls::<$faces, $n>(), Ok($expected));
        }
    };
}

macro_rules! rolls_unsupported {
    ($name:ident, $faces:literal, $n:literal) => {
        #[test]
        fn $name() {
            assert_eq!(
                min_rolls::<$faces, $n>(),
                Err(EntropyError::UnsupportedConfig($faces, $n))
            );
            assert_eq!(
                max_rolls::<$faces, $n>(),
                Err(EntropyError::UnsupportedConfig($faces, $n))
            );
        }
    };
}

macro_rules! dice_vector {
    ($name:ident, $faces:literal, $n:literal, $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let e: DiceEntropy<$faces, $n> = dice_to_entropy(&$input).unwrap();
            assert_eq!(e.expose_secret(), $expected);
        }
    };
}

macro_rules! dice_rejects {
    ($name:ident, $faces:literal, $n:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = dice_to_entropy::<$faces, $n>(&$input).err().unwrap();
            assert_eq!(err, $err);
        }
    };
}

min_rolls_ok!(min_rolls_d6_128bit,  6, 16 => 50);
min_rolls_ok!(min_rolls_d6_256bit,  6, 32 => 99);
min_rolls_ok!(min_rolls_d20_128bit, 20, 16 => 30);
min_rolls_ok!(min_rolls_d20_256bit, 20, 32 => 60);

max_rolls_ok!(max_rolls_d6_128bit,  6, 16 => 100);
max_rolls_ok!(max_rolls_d6_256bit,  6, 32 => 198);
max_rolls_ok!(max_rolls_d20_128bit, 20, 16 => 60);
max_rolls_ok!(max_rolls_d20_256bit, 20, 32 => 120);

rolls_unsupported!(rolls_unsupported_d7_16, 7, 16);
rolls_unsupported!(rolls_unsupported_d6_24, 6, 24);

// SHA-256("1" * 50)[:16]
dice_vector!(all_ones_d6_matches_krux, 6, 16, [1u8; 50] => &[
    0x3d, 0xac, 0x51, 0xa6, 0x5e, 0xc9, 0xfc, 0xfc,
    0x40, 0x9a, 0x1b, 0x5f, 0x1d, 0xef, 0xe9, 0x2b,
]);

dice_rejects!(too_few_rolls_d6,  6, 16, [1u8; 49]  => EntropyError::TooFewRolls(50, 49));
dice_rejects!(too_many_rolls_d6, 6, 16, [1u8; 101] => EntropyError::TooManyRolls(100, 101));

#[test]
fn deterministic_for_same_input() {
    let rolls = [3u8; 50];
    let a: DiceEntropy<6, 16> = dice_to_entropy(&rolls).unwrap();
    let b: DiceEntropy<6, 16> = dice_to_entropy(&rolls).unwrap();
    assert_eq!(a.expose_secret(), b.expose_secret());
}

#[test]
fn order_sensitive() {
    let mut a = [1u8; 50];
    a[0] = 6;
    let mut b = [1u8; 50];
    b[49] = 6;
    let ea: DiceEntropy<6, 16> = dice_to_entropy(&a).unwrap();
    let eb: DiceEntropy<6, 16> = dice_to_entropy(&b).unwrap();
    assert_ne!(ea.expose_secret(), eb.expose_secret());
}

#[test]
fn d6_at_max_length_accepted() {
    let rolls = [3u8; 100];
    let _: DiceEntropy<6, 16> = dice_to_entropy(&rolls).unwrap();
}

#[test]
fn d20_at_max_length_accepted() {
    let rolls = [10u8; 60];
    let _: DiceEntropy<20, 16> = dice_to_entropy(&rolls).unwrap();
}

#[test]
fn roll_zero_rejected() {
    let mut rolls = [1u8; 50];
    rolls[10] = 0;
    let err = dice_to_entropy::<6, 16>(&rolls).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(0, 10));
}

#[test]
fn roll_overrange_rejected_d6() {
    let mut rolls = [1u8; 50];
    rolls[5] = 7;
    let err = dice_to_entropy::<6, 16>(&rolls).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(7, 5));
}

#[test]
fn roll_overrange_rejected_d20() {
    let mut rolls = [1u8; 30];
    rolls[0] = 21;
    let err = dice_to_entropy::<20, 16>(&rolls).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(21, 0));
}

#[test]
fn distinct_die_geometries_yield_distinct_outputs() {
    let r6 = [1u8; 50];
    let r20 = [1u8; 30];
    let e6: DiceEntropy<6, 16> = dice_to_entropy(&r6).unwrap();
    let e20: DiceEntropy<20, 16> = dice_to_entropy(&r20).unwrap();
    assert_ne!(e6.expose_secret(), e20.expose_secret());
}
