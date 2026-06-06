# Initial Prompt (copy/paste)

You are implementing a Nostr-only hardware signer firmware fork with one non-negotiable objective:

## Primary Goal

Given an unsigned Nostr event, the device returns a correctly signed Nostr event.

Use `INSTRUCTIONS.md` as binding implementation guidance and execute all required sections.

## Hard constraints

- Firmware scope is Nostr signing only.
- Remove Bitcoin-specific flows and assumptions.
- Keep QR and SD card import/export workflows.
- Keep explicit user approval before signing.

## Required architecture

Build and wire the signer-first modules:

- `src/krux/nostr/event_parser.py`
- `src/krux/nostr/canonicalizer.py`
- `src/krux/nostr/schnorr_backend.py`
- `src/krux/nostr/approval_ui.py`
- `src/krux/nostr/exporter.py`

Responsibilities must remain separated as defined in `INSTRUCTIONS.md`.

## Crypto backend rule

If `embit` is needed for Schnorr signatures, isolate all `embit` imports to `schnorr_backend.py` only. Do not import `embit` elsewhere.

## Protocol requirements

- Implement canonical NIP-01 event serialization and event ID hashing.
- Sign and verify with Schnorr/secp256k1 before export.
- Treat NIP-19 entities as UI boundary formats; convert internally.
- Treat NIP-06 mnemonic derivation as advanced/optional flow.

## Test structure (mandatory)

Use:

- `tests/unit/`
- `tests/functional/`
- `tests/integration/`
- `tests/common/`

Integration tests must use a nostr regtest-like node plus a JSON-RPC client.

## Documentation and tooling tasks (mandatory)

1. Update `CONTRIBUTING.md` for Nostr-only scope, architecture boundaries, and test policy.
2. Create/update `SECURITY.md` with threat model and secret-handling policy.
3. Replace Poetry workflow with uv workflow in docs/scripts/automation references.

## Deliverables

1. Proposed file tree before edits.
2. Implemented code and docs updates.
3. Poetry-to-uv migration map (old command -> new command).
4. Test evidence grouped by unit/functional/integration.
5. Completion checklist proving signer path works end-to-end and no forbidden `embit` coupling exists.

## Reporting rules

- Distinguish verified facts from assumptions.
- Include exact commands executed and outcomes.
- If blocked, provide exact unblock steps, not general advice.
