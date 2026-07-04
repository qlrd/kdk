//! Demo: generate 15/18/21-word ("extended") mnemonics via the coin
//! pipeline. `dice` and `deck` entropy tables in `kdk-entropy` cover
//! only the Krux-blessed `(FACES/CARDS, 16/32)` rows today, so the
//! extended word counts dispatch to byte lengths those games don't
//! yet support — only `coin` works end-to-end here.
//!
//! Run: `cargo run --example games_extended -p kdk-mnemonic --features "coin,extended"`

use kdk_mnemonic::coin_mnemonic;

macro_rules! demo {
    ($w:literal, $call:expr) => {{
        let m = $call.expect("valid input");
        // `word_count()` reads the const generic — never touches the
        // inner secret.
        let words = m.word_count();
        assert_eq!(words, $w, "word count mismatch for {}", stringify!($call));
        println!("  {}: {m:?}  (words={words})", stringify!($call));
    }};
}

fn main() {
    println!("== coin extended (15 / 18 / 21 words) ==");
    demo!(15, coin_mnemonic::<15>(&[0u8; 160]));
    demo!(18, coin_mnemonic::<18>(&[0u8; 192]));
    demo!(21, coin_mnemonic::<21>(&[0u8; 224]));

    println!();
    println!("PASS: extended coin pipelines produced 15-, 18-, and 21-word mnemonics.");
}
