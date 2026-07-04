# `.github/agents/` — KDK review agents (GitHub Copilot)

De-personalized, **read-only advisory** custom agents for the **KDK Development
Kit** rust workspace. Each `*.agent.md` file is one agent: YAML frontmatter
(`name` / `description` / `tools`) plus a markdown operating manual, capped at
30,000 characters. Reference: [How to write a great agents.md][blog].

## Philosophy — assisted AI coding, not vibecoding

These agents **review, check, and guide**. They do not scaffold, generate, or
edit code. The developer writes the code; the agents run read-only checks
(`git diff`, `cargo check/test/clippy/fmt --check`) and report findings for the
developer to apply. There are deliberately **no implementation agents** (no
builder, tester, or workflow-editing agent) and **no persona/maintainer lenses**
(no named reviewers, no runtime-specific personas) — just neutral reviewers.

## What KDK is

A Rust workspace for **provable amnesic firmware** — Bitcoin signing primitives
for microcontrollers based on [krux](https://github.com/selfcustody/krux) and
[kern](https://github.com/odudex/Kern);

Every secret byte MUST be wiped after use, enforced at the type-system level
(`SensitiveBytes`). Active crates: `kdk-zeroize`, `kdk-entropy`, `kdk-mnemonic`.

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Agents

| Agent | Role |
|-------|------|
| [`code-reviewer`](code-reviewer.agent.md) | Reviews a diff against KDK architectural invariants + Rust conventions. |
| [`security-reviewer`](security-reviewer.agent.md) | Reviews for secret-leak / amnesia-wipe / crypto regressions; authority on secret paths. |
| [`completion-checker`](completion-checker.agent.md) | Walks the pre-merge checklist (test / clippy / fmt / doctest / changelog); structured pass/fail. |
| [`formatter`](formatter.agent.md) | Check-only `fmt` + `clippy` drift reporter — reports, never applies. |

All four are read-only (`tools: read, search, shell`): they run commands and
report, and **never edit, apply, or commit**.

[blog]: https://github.blog/ai-and-ml/github-copilot/how-to-write-a-great-agents-md-lessons-from-over-2500-repositories/
