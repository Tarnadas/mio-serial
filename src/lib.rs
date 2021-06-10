//! # serial-io: A serial port IO library
//!
//! serial-io is a fork of [mio-serial](https://github.com/berkowski/mio-serial) with
//! support for the [tokio runtime](https://tokio.rs/).
//! It combines therefore [tokio-serial](https://github.com/berkowski/tokio-serial) and adds support
//! for tokio version 1.
//!
//! serial-io provides a serial port implementation using [mio](https://github.com/carllerche/mio).
//!
//! **Windows support is present but largely untested by the author**
//!
//! ## Links
//!   - repo:  <https://github.com/tarnadas/serial-io>
//!   - docs:  <https://docs.rs/serial-io>

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "tokio")]
extern crate tokio_crate as tokio;

// Enums, Structs, and Traits from the serialport crate
pub use serialport::{
    available_ports, new as build, ClearBuffer, DataBits, Error, ErrorKind, FlowControl, Parity,
    SerialPort, SerialPortBuilder, SerialPortInfo, StopBits,
};

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::Serial;

#[cfg(all(unix, feature = "tokio"))]
pub use unix::tokio::AsyncSerial;

#[cfg(windows)]
pub use windows::Serial;

/// A type for results generated by interacting with serial ports.
pub type Result<T> = serialport::Result<T>;
