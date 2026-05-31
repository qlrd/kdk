# KDK

A WIP development kit in Rust for microcontroller bitcoin wallet signers
(mainly aimed to ESP32 or STM32) in Krux-style hardware signers
based on past [proposition to redesign krux](https://github.com/selfcustody/krux/discussions/189).

In any way i want to compete with [kern](https://github.com/odudex/kern) or
abandon [krux](https://github.com/selfcustody/krux), neither to redesign some
basic aspect of krux (this will not be a product or other techinical aspects)
or add another cryptocurrencies.

> **⚠️ Experimental.** This software has not been audited. Do not use it
> with real funds. See [`SECURITY.md`](SECURITY.md).

## Quick start

### Rust unit tests

```bash
cargo test
```

### Examples

To see sample examples:

```bash
cargo run --example <example_name> # on crates/kdk-*/examples
```

## License

[MIT](LICENSE) — see `LICENSE`.
