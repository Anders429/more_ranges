# Changelog

## Unreleased
### Added
- Implemented `Iterator` for `RangeFromExclusive`.
- Implemented `Iterator` for `RangeFromExclusiveToExclusive`.
- Implemented `Iterator` for `RangeFromExclusiveToInclusive`.
- Implemented `FusedIterator` for `RangeFromExclusive`.
- Implemented `FusedIterator` for `RangeFromExclusiveToExclusive`.
- Implemented `FusedIterator` for `RangeFromExclusiveToInclusive`.
- Implemented `DoubleEndedIterator` for `RangeFromExclusiveToExclusive`.
- Implemented `DoubleEndedIterator` for `RangeFromExclusiveToInclusive`.
- Implemented `ExactSizeIterator` for `RangeFromExclusiveToExclusive` for all integers within the pointer size, as well as `char`s.
- Implemented `ExactSizeIterator` for `RangeFromExclusiveToInclusive` for all integers within the pointer size, as well as `char`s.
- Implemented `Index` and `IndexMut` for `[T]`, `Vec<T>`, `str`, and `String` using `RangeFromExclusive<usize>`.
- Implemented `Index` and `IndexMut` for `[T]`, `Vec<T>`, `str`, and `String` using `RangeFromExclusiveToExclusive<usize>`.
- Implemented `Index` and `IndexMut` for `[T]`, `Vec<T>`, `str`, and `String` using `RangeFromExclusiveToInclusive<usize>`.
- Implemented `Index` for `CStr` using `RangeFromExclusive<usize>`.
- Optional `alloc` feature to enable interoperation with types from the `alloc` crate.
### Changed
- Increased MSRV to `1.38.0`.
- Updated dev dependency on `claim` to instead depend on `claims`.

## 0.1.0 - 2021-02-07
### Added
- `RangeFromExclusive` struct, with `RangeBounds` bounded exclusively below and unbounded above.
- `RangeFromExclusiveToExclusive` struct, with `RangeBounds` bounded exclusively below and exclusively above.
- `RangeFromExclusiveToInclusive` struct, with `RangeBounds` bounded exclusively below and inclusively above.
