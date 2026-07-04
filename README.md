# kdk

A WIP development kit in Rust for microcontroller bitcoin wallet signers
(mainly aimed to ESP32 or STM32) based on [selfcustody/krux](https://github.com/selfcustody/krux)
and [odudex/kern](https://github.com/odudex/kern) for support microcontroller
hardware signers with rust.

## Motivation

The motivation came from a past discussion that maybe could not be accesible
based on past proposition to include `nostr` features on firmware ([maybe could not be available for public](https://github.com/selfcustody/krux/discussions/189) and summarized here) as well
personal shares with bitcoin/rust developer friends:

> Should a hardware device support signing Nostr events? Definitely.
>
> Should Krux? I think it could make sense (if you view Krux as firmware for
hardware signing _in general_), but not in its current form due to all the
Bitcoin-specific assumptions being made about wallets, receive addresses, PSBTs,
etc.
>
> The way I see it, there are 3 options:
>
> 1) Add support for Nostr in the existing UI, but wrap a bunch of places in
> `if is_nostr()` style checks to hide or show certain UI elements (e.g.,
> `Scan Address`, `Wallet`, `Sign PSBT`, etc. don't make sense to show if using
> a Nostr key, and `Show Nostr public key`, `Sign Nostr event` don't make sense
> if using a Bitcoin key)
> 2) Fork Krux, rip out the Bitcoin stuff entirely, and focus the fork on being
> exclusively a Nostr signer
> 3) Don't fork Krux, but redesign it such that it can support different use cases
> or cryptocurrencies / elliptic curves.
> 
> What do others think?
>
> *Jeff Sun*, Krux's creator

### Disclaimer

In any way KDK wants to compete with [Kern](https://github.com/odudex/kern) or
[Krux](https://github.com/selfcustody/krux), neither to redesign some basic
aspects of krux, almost following a mix of `Option 1` and `Option 3`:

- keep based-bitcoin firwamre
- `master` branch could accept `bitcoin-related` PRs as **features**, except
altcoins (i.e., `nostr`, `liquid`, `pgp`, `crypto-primitives` could be accepted
depending o context and review) -- they could share `traits`, `enums`, `structs`
and `methods` with different parameters (e.g: `nostr` and `schnoor`)
 
> **⚠️ Experimental.** This software has not been audited. Do not use it
> with real funds. See [`SECURITY.md`](SECURITY.md).

## Quick start

## Download

```bash
git clone https://github.com/qlrd/kdk.git
```

### Rust unit tests

```bash
cargo test
```

### Examples

To see sample examples:

```bash
cargo run --example <example_name> # on crates/kdk-*/examples
```

## TODO

It's even have crates for microcontrollers, for now just some primitives to
build with (`rust-bitcoin`/`bdk` if applicable) in future while waiting for
microcontrollers come.

- wallet crate
- sign crate
- ...
- STM-32 crate
- ESP32-* crate
- integration tests (see [bornal](https://github,com/qlrd/bornal))


## License

[MIT](LICENSE) — see `LICENSE`.
