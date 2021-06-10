# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.3.0] 2021-06-10

### Added

- Reexport `SerialPortBuilder` from serialport-rs

### Changed

- (BREAKING) Removed platform specific `TTYPort`
- (BREAKING) Replaced `Serial::from_serial` with `Serial::from_builder`

### Fixed

## [0.2.0] 2021-06-10

### Added

- Added tokio example
- Reexport `new` as `build` and `TTYPort` from serialport-rs

### Changed

- (BREAKING) remove `unix` and `windows` export. They are implementation details and `Serial` is reexported already
- Hint about temporarily dropped Windows support

## [0.1.1] 2021-06-09

### Fixed

- Fixed links in documentation

## [0.1.0] 2021-06-09

### Added

- Added [tokio](https://github.com/tokio-rs/tokio/) 1.0 support with feature `tokio`

### Changed

- Forked and renamed crate to `serial-io`
- Bumped [mio](https://github.com/tokio-rs/mio) to 0.7
- Bumped [nix](https://github.com/nix-rust/nix) to 0.21
- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 4.0

---

**prior changelog of mio-serial**

## [3.3.1] 2020-03-15

### Added

- @flosse added #derive Debug support for the Serial struct in [#20](https://github.com/berkowski/mio-serial/pull/20)
- @vleesvlieg added automatic retrying for EINTR returns to file descriptors in [#21](https://github.com/berkowski/mio-serial/pull/21)

### Changed

- Bumped [nix](https://github.com/nix-rust/nix) to 0.17

## [3.3.0] 2019-08-23

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 3.3

## [3.2.14] 2019-06-01

### Changed

- Bumped [nix](https://github.com/nix-rust/nix) to 0.14 to address [#17](https://github.com/berkowski/mio-serial/issues/17)

## [3.2] 2019-01-12

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 3.2

## [3.1.1] 2019-01-12

### Changed

- Merged [#16](https://github/berkowski/mio-serial/pull/16) @yuja fixed feature flags

## [3.1] 2018-11-10

### Added

- Added "libudev" feature. Enabled by default, can be disabled for targets without udev support.

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 3.1
- Merged [#13](https://github.com/berkowski/mio-serial/pull/13) @dvtomas added some clarity to the example.

## [3.0.1] - 2018-11-06

### Changed

- Restricted [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 3.0
  serialport-rs 3.1 contains API breaking changes.

## [3.0.0] - 2018-10-06

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 3.0
  serialport-rs 3.0 contains breaking changes.
- Bumped [nix](https://github.com/nix-rust/nix) to 0.11
- `mio-serial` version number will now track upstream serialport-rs. mio-serial
  is mostly feature complete at this point (at least for \*nix) and this should
  help reduce confusion.

### Fixed

- Merged [#10](https://github.com/berkowski/mio-serial/pull/10) (thanks @yuja!). Addresses some
  windows timeout settings.

## [0.8.0] - 2018-03-31

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 2.3

### Added

- Merged[#5](https://github.com/berkowski/mio-serial/pull/5) @ndusart added `try_clone` implementations as requred
  by the serialport trait as of 2.3
- Closed[#6](https://github.com/berkowski/mio-serial/pull/6) @snorp also drew attention to the `try_clone` addition

## 0.7.0] - 2018-02-25

### Changed

- Bumped [serialport-rs](https://gitlab.com/susurrus/serialport-rs) to 2.1

### Added

- Merged[#4](https://github.com/berkowski/mio-serial/pull/4) @ndusart added windows support!
- Added appveyor config to support new windows impl.

## [0.6.0] - 2017-11-28

### Added

- Closed [#3](https://github.com/berkowski/mio-serial/pull/3) Reexport serialport::Error for error handling without importing serialport crate.
  Thanks @Idanko

## [0.5.0] - 2017-04-15

### Added

- Added [trust](https://github.com/japaric/trust) based ci

### Changed

- Changed license back to MIT now that `serialport-rs` is MPL-2.0
- Bumped `serialport-rs` dependency to 1.0

## [0.4.0] - 2017-02-13

### Changed

- Changed to LGPL-3 for compliance with `serialport` dependency.

## [0.3.0] - 2017-02-13 [YANKED]

### Added

- Bumped `serialport` dependency to 0.9
