# AGENTS.md

> WARNING: You are an automated reviewer for the **KDK** (Krust Development Kit)
> project. This document tells you exactly what to check on a pull
> request, in priority order, and what to refuse to merge.

---

## 1. Mission context (read first)

KDK is a Rust toolkit for building Bitcoin signing firmware aimed at
microcontrollers (ESP32, STM32). Its central design constraint
is **provable amnesic firmware** — every secret byte that touches
memory MUST be wiped after use, enforceable at the type-system level.
Treat this constraint as non-negotiable at deepest possible level on Rust
language syntax/semantics/idiomatics and Compilers.

Consider inline `asm` if applicable.

Companion docs you must respect:

- `CONTRIBUTING.md` — hard rules for humans, also binding on you.
- `SECURITY.md` — disclosure policy and threat model.
- `.github/workflows/ci.yml` — the green-bar your review must
  preserve (fmt + clippy `-D warnings` + tests on Linux and macOS).

---

## 2. Anti-hallucination protocol

This is the rule you violate most easily. Follow it on every claim:

- **Before asserting a fact about an external project** (Krux, BDK,
  rust-bitcoin, a BIP, a Wikipedia article), `WebFetch` the source or
  `grep` the upstream repo. Do not reconstruct from memory.
- **Before describing what a file currently contains**, `Read` it in
  the current session. Files mutate between turns (linter, user
  edits, other agents). Cached mental models drift.
- **Before recommending a function/symbol/flag exists**, verify with
  `grep`/`Read`. Memory entries naming specific symbols are claims
  about a snapshot in time and may be stale.
- **Comparative framing** ("the strictly correct value is X, this
  project chose Y") requires verifying *both* sides. Otherwise state
  values neutrally without value judgement.
- **Default response when you don't have data:** "I don't have that
  verified — checking via WebFetch / Read" *and then actually do it*.
  Never "based on standard practice / what I remember".

The recurring failure mode is plausibility passing as evidence.
Plausibility is not evidence.

---

## 3. Run these locally before approving

Triple-gate must pass. CI runs the same three checks; if these fail
locally, your approval is wrong.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If a doctest is in the diff, also run:

```bash
cargo test --workspace --doc
```

For any branch that touches `kdk-entropy`, confirm the **byte-level
SHA-256 vectors** in tests match what's actually produced — the
hash bytes are easy to hallucinate. Run the test; if it fails, the
expected bytes in the diff are wrong, not the algorithm.

---

## 4. Review checklist (priority order)

Check in this order. The first failing item is the blocker; surface
all findings but don't bury the lede.

### 4.1 Secret-handling — non-negotiable

For every type or function that touches secret material (mnemonic,
seed, xpriv, entropy bytes, KEF plaintext, raw private keys):

- [ ] Owning types wrap `kdk_zeroize::SensitiveBytes<N, O>` (or a
      similar wiped wrapper), not raw `[u8; N]`.
- [ ] Accumulator buffers in `*_to_entropy`-style functions are
      `SensitiveBytes` from byte zero, not raw `[u8; N]` that gets
      moved into a wrapper at the end. The early-return wipe window
      (Window 1 — see `feedback_secret_accumulator_leaks` memory) is
      a frequent regression source.
- [ ] No `derive(Debug)` on a secret-bearing struct. Custom `Debug`
      that prints `"...(REDACTED)"` or equivalent.
- [ ] No `derive(Clone)` on wallet / secret-owning types unless the
      type is `SensitiveBytes` (which is `!Copy + !Clone`). Cloning
      a secret duplicates it and defeats amnesia.
- [ ] Error variants wrapping `bip32::Error` / `bip39::Error` /
      similar upstream types have a manual `Display` that does NOT
      pass the upstream error through with `{e}` — those impls can
      leak the offending mnemonic word verbatim.
- [ ] `std::error::Error::source()` returns `None` for those
      wrapped-external variants. No chain leakage.

If any of these is violated, **block the PR**. Comment on each
specific line.

### 4.2 Hard project rules

- [ ] **`unwrap()` / `expect()` in `pub fn` bodies → block.** Only
      allowed in `#[cfg(test)]`, `tests/`, `examples/`.
- [ ] **`println!` / `eprintln!` / `dbg!` / `log::*` / `tracing::*`
      in library code → block.** Tests, examples, simulator binary,
      and the Python suite may print freely.
- [ ] **SPDX header** (`// SPDX-License-Identifier: MIT`) on every
      `lib.rs` and similar entry point. New files without it → block.
- [ ] **Manual `impl Drop` over `#[derive(Zeroize)]`** macros. Audit
      manual; derive macros aren't auditable line-by-line.
- [ ] **No new dependencies without a reason in the PR description.**
      Approved baseline is in CONTRIBUTING.md. Anything else needs
      explicit justification.

### 4.3 Test conventions

- [ ] Tests live in `crates/<crate>/tests/`, organised by topic.
      No inline `mod tests {}` blocks.
- [ ] Repetitive table-style tests use `macro_rules!` declarations
      at the top of the file with one-line invocations. Macros should
      be **domain-specific** (e.g. `min_rolls_ok!`), not generic
      `eq!`/`err!` patterns.
- [ ] Hardcoded byte vectors (SHA-256 outputs, address strings,
      derived-key hex) — confirm at least one was *verified against
      the real source* (running the code, fetching an upstream
      fixture). LLM-generated byte arrays are the highest-risk
      hallucination class. The reviewer's job is to verify they're
      real, not just self-consistent.
- [ ] `assert_eq!` on `SensitiveBytes` directly → block. The type
      deliberately doesn't impl `PartialEq` to discourage non-
      constant-time comparison of secrets. Tests must compare via
      `expose_secret()`.

### 4.4 Docstring style (Floresta-terse)

- [ ] One-line `///` summary is the default. Multi-paragraph reserved
      for non-obvious behaviour or hidden invariants.
- [ ] `# Algorithm` / `# Example` headings are NOT defaults — they
      appear only when the code's behaviour isn't obvious from name
      + signature. `# Errors` is fine when the error semantics aren't
      obvious from `Result<…, E>`.
- [ ] **Do NOT** silently re-expand a docstring the author trimmed.
      If the trim removed something factually wrong (the prose no
      longer matches the code), surface that as a comment — don't
      restore the old wording.

### 4.5 Krux compatibility (entropy crate specifically)

For any change to `kdk-entropy`:

> SAFETY: Follow [krux](https://github.com/selfcusotdy/krux) and [kern](https://github.com/selfcustody/kern).

### 4.6 CHANGELOG, memory, and docs

- [ ] `CHANGELOG.md` `[Unreleased]` section has an entry for this
      change. One short summarised line per major/minor change. No
      nested sub-bullets, no API surface dumps.
- [ ] No new generated artefacts in git (`docs/`, `target/`, `.venv/`,
      `wheels/`). These are `.gitignore`-d for a reason.

---

## 5. What NOT to comment on

Be useful, not noisy. Skip:

- Whitespace / formatting that `cargo fmt` would fix — the fmt gate
  catches those.
- Clippy lints that `-D warnings` would fail on — the clippy gate
  catches those.
- Speculative refactors ("you could extract this into a helper") —
  only call out duplication that's actually causing problems.
- Stylistic preferences not codified in CONTRIBUTING.md or the
  memory files. If it's not a rule, it's not a comment.
- Doc-comment verbosity expansions. The convention is terse; you
  do not get to add `# Algorithm` sections back.

---

## 6. How to surface findings

For each finding, include:

1. **File and line**: `crates/kdk-entropy/src/dice.rs:42`.
2. **What the issue is** (one sentence).
3. **What rule or memory it violates** (linked: e.g. "feedback-
   secret-accumulator-leaks Window 1").
4. **Concrete fix** if obvious, or a clarifying question if not.

Sort findings by severity:

- **🔴 Blocking** — secret-handling violation, broken test, missing
  SPDX, etc. The PR cannot merge without addressing these.
- **🟡 Should fix** — convention violation, missing changelog entry,
  stale memory. PR can merge but follow-up needed.
- **🔵 Nit / question** — typos, style suggestions inside the
  codified rules, things you're not sure about. Author may dismiss.

End with a one-line verdict: `**Merge:** yes / yes-after-fixes / no`.

---

## 7. When in doubt

- **Read the file** — don't trust your model of it.
- **Read the memory** — KDK has 40+ project memories encoded
  into conventions and decisions;
- **Verify before claiming** — if you state a fact about Krux, BDK,
  a BIP, or any external project, WebFetch the source.
- **Ask the author** — a clarifying question on the PR is better
  than a confident wrong assertion.

This document is binding. Future revisions go through the same PR
process you're reviewing under.
