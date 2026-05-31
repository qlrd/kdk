# Contributing to KDK

Thanks for considering a contribution. KDK aims to be a **provable
amnesic firmware** for Bitcoin signing — secrets must be wiped after
use, enforced at the type-system and test-suite level.

## Hard rules

### Rust code

- **No `unwrap()` or `expect()` in `pub fn` bodies.** Propagate errors
  via `Result`, `?`, `match`, `ok_or(...)`, `map_err(...)`. Tests and
  examples may use `unwrap` freely.
- **No `println!` / `eprintln!` / `dbg!` / `log::*` / `tracing::*` in
  library code.** never logs — errors propagate, the
  caller decides what to display.
- **Tests live under `crates/<crate>/tests/`**, organised by topic
- **Shared fixtures `tests/common/mod.rs`.** Repetitive spec-vector
  tables use `#[macro_export]` macros.
- **No new dependencies without discussion.** Approved baseline:
  - **`kdk-zeroize`** has **zero external deps** — it's the
    foundation; adding a dep here requires an explicit security
    review.
  - `bitcoin`, `bip39`, `secp256k1` (rust-bitcoin org)
  - `bip322`, `psbt-v2` (BIP support)
  - `aes-gcm`, `pbkdf2`, `sha2`, `hmac` (KEF crypto)
  - `hex` (dev-dep only)
 
- **Format + lint clean before pushing:**
  ```bash
  cargo fmt --all
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  ```
  CI runs the same three checks.

### Secret hygiene — provable amnesic firmware

Every type that owns secret material must zeroize on drop. Use
`kdk-zeroize` primitives. We aim to be less dependency possible.

- **`SensitiveBytes<N, O>`** — fixed-size buffer with const-generic
  length and a phantom `O` origin marker.
  `enum Bip39Seed {}`) to distinguish secret kinds at the type level.
- **`wipe_in_place<T>` / `wipe_in_place_mut<T>`** — generic primitives
  for wiping non-`SensitiveBytes` types.
- **Custom `Debug` impl that redacts** for every secret-bearing type.
  Never `derive(Debug)` on a struct with a `SensitiveBytes`.
- **No `derive(Clone)` on wallet types** — cloning a secret duplicates
  it, defeating the amnesic guarantee.
- **`std::error::Error::source()` returns `None`** for variants that
  wrap external errors (`bip32::Error`, `bip39::Error`). Upstream
  `Display`/`Debug` impls can leak user bytes (a bad mnemonic word,
  raw key material) — never chain through them.
- **Validate at every API boundary.** Hardened-index rejection, range
  checks, policy compatibility — typed errors, never panic.

### Error-type style

- Every variant has a `///` doc comment.
- **Manual `Display` + `From` impls.**
- Error enums implement `std::error::Error` with `source()` returning
  `None` for any variant that wraps an upstream error containing user
  bytes.

## Workspace structure

Each crate carries its own `examples/` and `tests/` directories.

## Development loop

```bash
# Format + lint + test (mirrors CI)
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Overall, have fun :)
