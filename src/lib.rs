//! Range types that are not included in the standard library.
//!
//! Specifically, these are ranges which are bounded exclusively below.
//!
//! These ranges operate nearly the identical to those in
//! [`std::ops`](https://doc.rust-lang.org/std/ops/index.html). However, some functionality is only
//! available on the `nightly` channel. See individual items for details on what functionality
//! requires `nightly`.
//!
//! # Example
//! While each range type in the standard library is either bounded inclusively below or unbounded
//! below, each range type provided in this crate is bounded exclusively below. Compare, for
//! example, [`RangeFrom`] with [`RangeFromExclusive`].
//!
//! ```
//! # #[cfg(impl_range_bounds)]
//! fn test() {
//! use std::ops::{Bound, RangeBounds, RangeFrom};
//! use more_ranges::RangeFromExclusive;
//!
//! let range_inclusive = RangeFrom {
//!     start: 1,
//! };
//! let range_exclusive = RangeFromExclusive {
//!     start: 1,
//! };
//!
//! // The inclusive range is inclusively bound.
//! assert_eq!(range_inclusive.start_bound(), Bound::Included(&1));
//!
//! // Contrastingly, the exclusive range is exclusively bound.
//! assert_eq!(range_exclusive.start_bound(), Bound::Excluded(&1));
//! # }
//! # #[cfg(not(impl_range_bounds))]
//! # fn test() {}
//! # test();
//! ```
//!
//! [`Iterator`]: core::iter::Iterator
//! [`RangeFrom`]: core::ops::RangeFrom
#![cfg_attr(feature_re_rebalance_coherence, feature(re_rebalance_coherence))]
#![cfg_attr(feature_collections_range, feature(collections_range))]
#![cfg_attr(feature_step, feature(step_trait, step_trait_ext, unchecked_math))]
#![cfg_attr(feature_trusted_len, feature(trusted_len))]
#![cfg_attr(feature_alloc, feature(alloc))]
#![cfg_attr(feature_doc_cfg, feature(doc_cfg))]
#![no_std]

#[cfg(all(impl_index, alloc))]
extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate claim;
#[cfg(feature = "doc_item")]
extern crate doc_item;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(all(feature = "serde", test))]
extern crate serde_test;
#[cfg(all(impl_index, std))]
extern crate std;

#[cfg(impl_index)]
mod impl_index;
#[cfg(impl_iterator)]
mod impl_iterator;
#[cfg(impl_range_bounds)]
mod impl_range_bounds;
#[cfg(feature = "serde")]
mod impl_serde;

/// A range only bounded exclusively below.
///
/// The `RangeFromExclusive` contains all values with `x > start`.
///
/// *Note*: Overflow in the [`Iterator`] implementation (when the contained data type reaches its
/// numerical limit) is allowed to panic, wrap, or saturate. This behavior is defined by the
/// implementation of the [`Step`] trait. For primitive integers, this follows the normal rules, and
/// respects the overflow checks profile (panic in debug, wrap in release).
///
/// # Example
/// `RangeFromExclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusive;
///
/// let range = RangeFromExclusive {
///     start: 1,
/// };
/// ```
///
/// [`Iterator`]: core::iter::Iterator
/// [`Step`]: core::iter::Step
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
}

/// A range bounded exclusively below and inclusively above.
///
/// The `RangeFromExclusiveToInclusive` contains all values with `x > start` and `x <= end`. It is
/// empty unless `start < end`.
///
/// # Example
/// `RangeFromExclusiveToInclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusiveToInclusive;
///
/// let range = RangeFromExclusiveToInclusive {
///     start: 1,
///     end: 4,
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToInclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
    /// The upper bound of the range (inclusive).
    pub end: Idx,
}

/// A range bounded exclusively below and above.
///
/// The `RangeFromExclusiveToExclusive` contains all values with `x > start` and `x < end`. It is
/// empty unless `start < end + 1`.
///
/// # Example
/// `RangeFromExclusiveToExclusive`s can be created directly, as follows:
///
/// ```
/// use more_ranges::RangeFromExclusiveToExclusive;
///
/// let range = RangeFromExclusiveToExclusive {
///     start: 1,
///     end: 4,
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeFromExclusiveToExclusive<Idx> {
    /// The lower bound of the range (exclusive).
    pub start: Idx,
    /// The upper bound of the range (exclusive).
    pub end: Idx,
}
