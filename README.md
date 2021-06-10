# serial-io: A serial port IO library

[![crates.io](http://meritbadge.herokuapp.com/serial-io)](https://crates.io/crates/serial-io)
[![docs.rs](https://docs.rs/serial-io/badge.svg)](https://docs.rs/serial-io)

serial-io is a fork of [mio-serial](https://github.com/berkowski/mio-serial) with
support for the [tokio runtime](https://tokio.rs/).
It combines therefore [tokio-serial](https://github.com/berkowski/tokio-serial) and adds support
for tokio version 1.

serial-io provides a serial port implementation using [mio](https://github.com/carllerche/mio).

**Windows support has been dropped until Tokio provides a serial interface**

## Usage

Add `serial-io` to you `Cargo.toml`:

```toml
[dependencies]
serial-io = "0.1"
```

Optionally enable `tokio` feature:

```toml
[dependencies]
serial-io = { version = "0.1", features = ["tokio] }
```

## Features

The "libudev" dependency of `serialport-rs` is enabled by default. For x86 linux systems this enables the `available_ports` function for port enumeration.
Not all targets support udev, especially when cross-compiling. To disable this feature, compile with the `--no-default-features` option. For example:

```
cargo build --no-default-features
```

## Examples

A few examples can be found [here](https://github.com/tarnadas/serial-io/tree/master/examples).

## License

This software is licensed under [MIT](https://opensource.org/licenses/MIT).

This software builds upon the [MPL-2.0](https://opensource.org/licenses/MPL-2.0) licensed [serialport-rs](https://gitlab.com/susurrus/serialport-rs) and
constitutes a "Larger Work" by that license. The source for [serialport-rs](https://gitlab.com/susurrus/serialport-rs) can be found at https://gitlab.com/susurrus/serialport-rs.
