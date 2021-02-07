# more_ranges

Range types not provided in the standard library.

This crate provides range types that are bounded exclusively below. Specifically, the types provided
are:

- [`RangeFromExclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusive.html)
- [`RangeFromExclusiveToExclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusiveToExclusive.html)
- [`RangeFromExclusiveToInclusive`](https://docs.rs/more_ranges/*/more_ranges/struct.RangeFromExclusiveToInclusive.html)

These ranges operate nearly the same as those in
[`std::ops`](https://doc.rust-lang.org/std/ops/index.html). However, they do not function as
[`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)s, nor can they be used in
indexing.

## Example
The range types provided here can be used by directly specifying their fields:

```rust
use more_ranges::{RangeFromExclusive, RangeFromExclusiveToExclusive, RangeFromExclusiveToInclusive};

let range_from_exclusive = RangeFromExclusive { start: 1 };
let range_from_exclusive_to_exclusive = RangeFromExclusiveToExclusive { start: 1, end: 4 };
let range_from_exclusive_to_inclusive = RangeFromExclusiveToInclusive { start: 1, end: 4 };
```

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.28.0` and up.
