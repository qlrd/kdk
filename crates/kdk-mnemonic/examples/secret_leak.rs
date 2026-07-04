//! Check of the secret-leak boundary between `bip39`'s plain types and
//! `kdk-mnemonic`'s wrappers.
//!
//! `kdk-mnemonic` does not eliminate copies; it try to guarantees the sensitive
//! copy is wiped on drop and removes the legal path to read the source
//! copy after move.
//!
//! Run: `cargo run --example secret_leak -p kdk-mnemonic`
//!
//! # Security notice
//!
//! `bip39::Mnemonic::from_entropy(&entropy)` borrows the entropy: the
//! caller keeps their copy alongside bip39's internal one.
//! `SensitiveBytes::new(entropy_bytes)` *moves* the entropy in: the
//! compiler bans further reads of the source. This does not eliminate the
//! physical bytes in the source frame until LLVM reuses that slot.

use bip39::Mnemonic;
use kdk_mnemonic::{entropy_to_mnemonic, GameMnemonic};
use kdk_zeroize::{prelude::*, SensitiveBytes};

enum Vector {}

fn main() {
    let entropy = [0u8; 16]; // canonical "abandon × 11 + about" 12-word vector

    println!("== bip39 (reference: Debug redacts, Display leaks) ==");
    let bip_mnemonic = Mnemonic::from_entropy(&entropy).expect("valid entropy");
    let dbg = format!("{:?}", bip_mnemonic);
    let disp = format!("{}", bip_mnemonic);
    println!("  Debug:   {dbg}");
    println!("  Display: {disp}");
    assert!(
        !dbg.contains("abandon"),
        "bip39 Debug unexpectedly leaked the word list"
    );
    assert!(
        disp.contains("about"),
        "expected bip39 Display to leak the word list"
    );

    println!();
    println!("== kdk-mnemonic (Debug AND Display both redact) ==");
    let kdk_entropy = SensitiveBytes::<16, Vector>::new(entropy);
    let kdk_mnemonic: GameMnemonic<Vector, 12> =
        entropy_to_mnemonic(&kdk_entropy).expect("entropy length matches W=12");
    let dbg = format!("{:?}", kdk_mnemonic);
    let disp = format!("{}", kdk_mnemonic);
    println!("  Debug:   {dbg}");
    println!("  Display: {disp}");
    // Real evidence: no word from the canonical 12-word vector leaks
    // on either channel. The literal "GameMnemonic(REDACTED)" string
    // is our own impl's output — asserting it equals itself adds no
    // information.
    assert!(!dbg.contains("abandon") && !dbg.contains("about"));
    assert!(!disp.contains("abandon") && !disp.contains("about"));

    println!();
    println!("== bip39-style raw [u8; 16] ==");
    let raw = [0xAAu8; 16];
    let raw_dbg = format!("{:?}", raw);
    println!("  Debug:   {raw_dbg}");
    assert!(
        raw_dbg.contains("170"), // 0xAA = 170 decimal
        "raw array Debug should print byte values"
    );
    // Display is not implemented for `[u8; N]`; the line below would
    // fail with E0277 ("the trait `Display` is not implemented for `[u8; 16]`"):
    // let _ = format!("{}", raw);
    println!("  Display: <not implemented on [u8; N] — would not compile>");

    println!();
    println!("== kdk SensitiveBytes<16> ==");
    let wrapped = SensitiveBytes::<16, Vector>::new(raw);
    let wrapped_dbg = format!("{:?}", wrapped);
    let wrapped_disp = format!("{}", wrapped);
    println!("  Debug:   {wrapped_dbg}");
    println!("  Display: {wrapped_disp}");
    // Real evidence: the original byte value (170 decimal = 0xAA)
    // does not appear in either channel.
    assert!(!wrapped_dbg.contains("170") && !wrapped_disp.contains("170"));

    println!();
    println!("== bip39::Mnemonic Drop ==");
    let bip_ptr: *mut Mnemonic;
    {
        // <internal code>
        let boxed = Box::new(Mnemonic::from_entropy(&[0xAAu8; 16]).expect("valid entropy"));
        bip_ptr = Box::into_raw(boxed);
    }

    unsafe {
        let size = core::mem::size_of::<Mnemonic>();
        let pre: Vec<u8> = (0..size)
            .map(|i| core::ptr::read_volatile((bip_ptr as *const u8).add(i)))
            .collect();
        core::ptr::drop_in_place(bip_ptr);
        let post: Vec<u8> = (0..size)
            .map(|i| core::ptr::read_volatile((bip_ptr as *const u8).add(i)))
            .collect();
        assert!(
            !post.iter().all(|&b| b == 0),
            "bip39 Mnemonic unexpectedly zeroed on Drop"
        );
        println!(
            "  bytes pre-drop  (first 16):  {:02x?}",
            &pre[..16.min(size)]
        );
        println!(
            "  bytes post-drop (first 16):  {:02x?}",
            &post[..16.min(size)]
        );
        std::alloc::dealloc(bip_ptr as *mut u8, std::alloc::Layout::new::<Mnemonic>());
    }

    println!();
    println!("== SensitiveBytes Drop ==");
    let raw_ptr: *mut SensitiveBytes<16, Vector>;
    {
        // <internal code>
        // Heap-alloc so the address survives the inner scope and stays
        // valid for post-drop inspection. `Box::into_raw` consumes the
        // Box without running its destructor — the SensitiveBytes is
        // still alive in memory, ownership transferred to `raw_ptr`.
        let boxed = Box::new(SensitiveBytes::<16, Vector>::new([0xAAu8; 16]));
        raw_ptr = Box::into_raw(boxed);
    }

    unsafe {
        let mut pre = [0u8; 16];
        for (i, slot) in pre.iter_mut().enumerate() {
            *slot = core::ptr::read_volatile((raw_ptr as *const u8).add(i));
        }
        core::ptr::drop_in_place(raw_ptr);
        let mut post = [0u8; 16];
        for (i, slot) in post.iter_mut().enumerate() {
            *slot = core::ptr::read_volatile((raw_ptr as *const u8).add(i));
        }
        assert_eq!(post, [0u8; 16], "Drop did NOT wipe the bytes");
        println!("  bytes pre-drop:  {:02x?}", pre);
        println!("  bytes post-drop: {:02x?}", post);
        std::alloc::dealloc(
            raw_ptr as *mut u8,
            std::alloc::Layout::new::<SensitiveBytes<16, Vector>>(),
        );
    }
    let leaked = format!("{}", kdk_mnemonic.expose_secret());
    let seed = kdk_mnemonic.expose_secret().to_seed("");
    let seed_dbg = format!("{:?}", seed);
    assert!(leaked.contains("abandon"));
    assert!(seed_dbg.starts_with("["), "bare seed prints its bytes");
    println!();
    println!("      > You can deliberately expose secret `.expose_secret()` and/or");
    println!("      > use `Display` to leak it:");
    println!("      >");
    println!("      >");
    println!("      >   {leaked}");
    println!("      >");
    println!("      >");
    println!("      > Or use `to_seed()` and slice it:");
    println!("      >");
    println!("      >");
    println!("      >   {:?}", &seed[..8]);
}
