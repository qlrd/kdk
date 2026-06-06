# INSTRUCTIONS.md

This document defines the execution contract for an implementation agent working on a Nostr-only firmware fork derived from Krux.

## 1) North-star objective

The most important outcome is:

- given an unsigned Nostr event, the device returns a correctly signed Nostr event.

Everything else is secondary until this works reliably.

## 2) Product boundaries

The firmware scope is Nostr signing only.

- Remove Bitcoin-specific UX and flows (wallet, receive address, PSBT signing).
- Keep user-critical transport and interaction paths:
  - QR scan/import and QR export
  - SD card import/export
  - key onboarding
  - event review and explicit sign confirmation

Do not reintroduce Bitcoin abstractions as compatibility layers.

## 3) Mandatory signer-first module layout

Implement and keep responsibilities strict in the following modules:

- `src/krux/nostr/event_parser.py`
- `src/krux/nostr/canonicalizer.py`
- `src/krux/nostr/schnorr_backend.py`
- `src/krux/nostr/approval_ui.py`
- `src/krux/nostr/exporter.py`

Rules:

- parsing/validation only in `event_parser`
- canonical NIP-01 serialization and ID calculation only in `canonicalizer`
- cryptographic sign/verify only in `schnorr_backend`
- user confirmation rendering/logic only in `approval_ui`
- output/transport formatting only in `exporter`

## 4) Crypto dependency policy

If `embit` is required for Schnorr during migration:

- isolate all `embit` usage to `schnorr_backend.py`
- do not import `embit` in UI, parser, exporter, or app orchestration code
- keep adapter surface minimal (`sign_hash`, `verify_hash`, `derive_pubkey`)

Prefer eventual replacement with a narrower backend when feasible.

## 5) Nostr protocol rules (must enforce)

- NIP-01 event ID must be computed from canonical serialized event fields.
- Signature must be Schnorr over secp256k1 for the computed event hash.
- Verify signatures before export; fail closed on mismatch.
- NIP-19 bech32 forms (`nsec`, `npub`, etc.) are UI I/O formats; convert to internal form for protocol operations.
- NIP-06 mnemonic derivation is optional and should be treated as advanced flow, not default onboarding.

## 6) Test taxonomy and placement

Use this exact structure:

- `tests/unit/` for deterministic logic checks
- `tests/functional/` for end-to-end app behavior checks
- `tests/integration/` for external-system tests (nostr regtest-like node + JSON-RPC client)
- `tests/common/` for shared fixtures/helpers

Each test belongs to one tier only.

## 7) Documentation requirements

The implementation must include documentation updates:

- update `CONTRIBUTING.md` with:
  - Nostr-only scope and architecture policy
  - module ownership boundaries
  - dependency and testing policy
  - pre-merge verification commands
- create or update `SECURITY.md` with:
  - threat model
  - secret handling policy
  - disclosure policy
  - signing safety guarantees and explicit non-goals

## 8) Tooling migration requirement

Replace Poetry-based workflows with uv-based workflows:

- remove Poetry-first instructions from docs and scripts
- add uv setup/install/run/test/lint commands
- update CI or local automation references accordingly
- include old-command to new-command mapping in migration notes

## 9) Execution checklist

Before declaring completion, verify all items:

1. signer path works end-to-end: parse -> canonicalize -> review -> sign -> verify -> export
2. no direct `embit` imports outside `schnorr_backend.py`
3. tests exist in the correct tiered structure
4. `CONTRIBUTING.md` and `SECURITY.md` updated
5. Poetry to uv migration applied and documented
6. no secret leakage in logs, errors, or artifacts

## 10) Reporting format

Final implementation report must include:

- summary of changed files and rationale
- test evidence grouped by unit/functional/integration
- blockers and exact remediation steps (if any)
- explicit "verified" vs "assumed" statements
