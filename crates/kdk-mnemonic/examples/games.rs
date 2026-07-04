//! Demo: generate a BIP39 mnemonic from each kdk-mnemonic game
//! pipeline (coin, dice, deck) at the default word counts (12, 24).
//!
//! Mnemonic contents stay redacted via `GameMnemonic`'s
//! `Debug`/`Display` impls (see `examples/secret_leak.rs` for the
//! leak-surface analysis).
//!
//! For 15/18/21-word ("extended") word counts, see
//! `examples/games_extended.rs` — `coin` is the only game whose
//! entropy table covers those byte sizes today.
//!
//! Run: `cargo run --example games -p kdk-mnemonic --features "coin,dice,deck,words-12,words-24"`

use kdk_mnemonic::{coin_mnemonic, deck_mnemonic, dice_mnemonic};

macro_rules! demo {
    ($w:literal, $call:expr) => {{
        let m = $call.expect("valid input");
        // `word_count()` reads the const generic — never touches the
        // inner secret. Reserve `expose_secret()` for code that
        // actually needs the words.
        let words = m.word_count();
        assert_eq!(words, $w, "word count mismatch for {}", stringify!($call));
        println!("  {}: {m:?}  (words={words})", stringify!($call));
    }};
}

const fn ascending<const K: usize>() -> [u8; K] {
    let mut arr = [0u8; K];
    let mut i = 0;
    while i < K {
        arr[i] = i as u8;
        i += 1;
    }
    arr
}

fn main() {
    println!("== coin (FACES=2: each flip = 1 bit) ==");
    demo!(12, coin_mnemonic::<12>(&[0u8; 128]));
    demo!(24, coin_mnemonic::<24>(&[1u8; 256]));

    println!();
    println!("== dice D6 ==");
    demo!(12, dice_mnemonic::<6, 12>(&[1u8; 50]));
    demo!(24, dice_mnemonic::<6, 24>(&[1u8; 99]));

    println!();
    println!("== dice D20 ==");
    demo!(12, dice_mnemonic::<20, 12>(&[1u8; 30]));
    demo!(24, dice_mnemonic::<20, 24>(&[1u8; 60]));

    println!();
    println!("== deck 52-card (standard) ==");
    demo!(12, deck_mnemonic::<52, 12>(&ascending::<25>()));

    println!();
    println!("== deck 78-card (tarot) ==");
    demo!(24, deck_mnemonic::<78, 24>(&ascending::<45>()));

    println!();
    println!("PASS: all six game pipelines produced mnemonics with the expected word count.");
}
