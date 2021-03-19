# more_ranges

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Anders429/more_ranges/Tests)](https://github.com/Anders429/more_ranges/actions)
[![codecov.io](https://img.shields.io/codecov/c/gh/Anders429/more_ranges)](https://codecov.io/gh/Anders429/more_ranges)
[![crates.io](https://img.shields.io/crates/v/more_ranges)](https://crates.io/crates/more_ranges)
[![docs.rs](https://docs.rs/more_ranges/badge.svg)](https://docs.rs/more_ranges)
[![MSRV](https://img.shields.io/badge/rustc-1.13.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/more_ranges)](#license)

Range types not provided in the standard library.

This crate provides range types that are bounded exclusively below. Specifically, the types provided
are:

- [`RangeFromExclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusive.html)
- [`RangeFromExclusiveToExclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusiveToExclusive.html)
- [`RangeFromExclusiveToInclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusiveToInclusive.html)

These ranges operate nearly the identical to those in
[`std::ops`](https://doc.rust-lang.org/std/ops/index.html). However, some functionality is only
available on the `nightly` channel. See the [documentation](https://docs.rs/more_ranges/) for
details.

## Example
The range types provided here can be used by directly specifying their fields:

```rust
use more_ranges::{RangeFromExclusive, RangeFromExclusiveToExclusive, RangeFromExclusiveToInclusive};

let range_from_exclusive = RangeFromExclusive { start: 1 };
let range_from_exclusive_to_exclusive = RangeFromExclusiveToExclusive { start: 1, end: 4 };
let range_from_exclusive_to_inclusive = RangeFromExclusiveToInclusive { start: 1, end: 4 };
```

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.13.0` and up. Some functionality requires
later versions. See the [documentation](https://docs.rs/more_ranges/) for individual requirements.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/more_ranges/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/more_ranges/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
