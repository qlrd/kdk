use kdk_entropy::{deck_to_entropy, max_draws, min_draws, DeckEntropy, EntropyError};

macro_rules! min_draws_ok {
    ($name:ident, $cards:literal, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(min_draws::<$cards, $n>(), Ok($expected));
        }
    };
}

macro_rules! max_draws_ok {
    ($name:ident, $cards:literal, $n:literal => $expected:literal) => {
        #[test]
        fn $name() {
            assert_eq!(max_draws::<$cards, $n>(), Ok($expected));
        }
    };
}

macro_rules! draws_unsupported {
    ($name:ident, $cards:literal, $n:literal) => {
        #[test]
        fn $name() {
            assert_eq!(
                min_draws::<$cards, $n>(),
                Err(EntropyError::UnsupportedConfig($cards, $n))
            );
            assert_eq!(
                max_draws::<$cards, $n>(),
                Err(EntropyError::UnsupportedConfig($cards, $n))
            );
        }
    };
}

macro_rules! deck_vector {
    ($name:ident, $cards:literal, $n:literal, $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let e: DeckEntropy<$cards, $n> = deck_to_entropy(&$input).unwrap();
            assert_eq!(e.expose_secret(), $expected);
        }
    };
}

macro_rules! deck_rejects {
    ($name:ident, $cards:literal, $n:literal, $input:expr => $err:expr) => {
        #[test]
        fn $name() {
            let err = deck_to_entropy::<$cards, $n>(&$input).err().unwrap();
            assert_eq!(err, $err);
        }
    };
}

min_draws_ok!(min_draws_40_16,  40, 16 => 28);
min_draws_ok!(min_draws_48_16,  48, 16 => 26);
min_draws_ok!(min_draws_52_16,  52, 16 => 25);
min_draws_ok!(min_draws_58_16,  58, 16 => 24);
min_draws_ok!(min_draws_58_32,  58, 32 => 55);
min_draws_ok!(min_draws_78_16,  78, 16 => 22);
min_draws_ok!(min_draws_78_32,  78, 32 => 45);
min_draws_ok!(min_draws_108_16, 108, 16 => 22);
min_draws_ok!(min_draws_108_32, 108, 32 => 41);
min_draws_ok!(min_draws_112_16, 112, 16 => 22);
min_draws_ok!(min_draws_112_32, 112, 32 => 42);

max_draws_ok!(max_draws_52_16, 52, 16 => 50);
max_draws_ok!(max_draws_58_16, 58, 16 => 48);
max_draws_ok!(max_draws_58_32, 58, 32 => 58); // capped at CARDS
max_draws_ok!(max_draws_108_16, 108, 16 => 44);

draws_unsupported!(draws_unsupported_52_32, 52, 32);
draws_unsupported!(draws_unsupported_32_16, 32, 16);

// SHA-256("0-1-2-...-24")[:16]
deck_vector!(identity_order_52_matches_sha256, 52, 16, [
    0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24,
] => &[
    0x92, 0xe2, 0x92, 0xe3, 0x4c, 0x44, 0x48, 0xc1,
    0x57, 0x73, 0x1a, 0xff, 0x3d, 0x40, 0x10, 0x38,
]);

// SHA-256("0-1-2-...-21")[:16]
deck_vector!(identity_order_108_matches_sha256, 108, 16, [
    0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21,
] => &[
    0xee, 0x89, 0x77, 0x46, 0xc3, 0x0a, 0xd6, 0xd9,
    0x61, 0x04, 0x78, 0xff, 0xe5, 0x1c, 0x41, 0xf9,
]);

deck_rejects!(too_few_cards, 52, 16, [0u8; 24] => EntropyError::TooFewRolls(25, 24));

#[test]
fn deterministic_for_same_input() {
    let cards: [u8; 25] = [
        51, 0, 25, 12, 37, 1, 38, 24, 13, 50, 26, 11, 36, 2, 39, 23, 14, 49, 27, 10, 35, 3, 40, 22,
        15,
    ];
    let a: DeckEntropy<52, 16> = deck_to_entropy(&cards).unwrap();
    let b: DeckEntropy<52, 16> = deck_to_entropy(&cards).unwrap();
    assert_eq!(a.expose_secret(), b.expose_secret());
}

#[test]
fn order_sensitive() {
    let mut a = [0u8; 25];
    for (i, c) in a.iter_mut().enumerate() {
        *c = i as u8;
    }
    let mut b = a;
    b.swap(0, 24);
    let ea: DeckEntropy<52, 16> = deck_to_entropy(&a).unwrap();
    let eb: DeckEntropy<52, 16> = deck_to_entropy(&b).unwrap();
    assert_ne!(ea.expose_secret(), eb.expose_secret());
}

#[test]
fn too_many_cards_rejected() {
    let mut cards = [0u8; 51];
    for (i, c) in cards.iter_mut().enumerate() {
        *c = i as u8;
    }
    let err = deck_to_entropy::<52, 16>(&cards).err().unwrap();
    assert_eq!(err, EntropyError::TooManyRolls(50, 51));
}

#[test]
fn card_out_of_range_rejected() {
    let mut cards = [0u8; 25];
    for (i, c) in cards.iter_mut().enumerate() {
        *c = i as u8;
    }
    cards[10] = 52;
    let err = deck_to_entropy::<52, 16>(&cards).err().unwrap();
    assert_eq!(err, EntropyError::RollOutOfRange(52, 10));
}

#[test]
fn duplicate_card_rejected() {
    let mut cards = [0u8; 25];
    for (i, c) in cards.iter_mut().enumerate() {
        *c = i as u8;
    }
    cards[10] = 3;
    let err = deck_to_entropy::<52, 16>(&cards).err().unwrap();
    assert_eq!(err, EntropyError::DuplicateCard(3, 10));
}

#[test]
fn unsupported_config_rejected_at_call() {
    let mut cards = [0u8; 25];
    for (i, c) in cards.iter_mut().enumerate() {
        *c = i as u8;
    }
    let err = deck_to_entropy::<52, 32>(&cards).err().unwrap();
    assert_eq!(err, EntropyError::UnsupportedConfig(52, 32));
}
