//! Range types that are not included in the standard library.
//!
//! Specifically, these are ranges which are bounded exclusively below.
//!
//! These ranges currently do not function as [`Iterator`]s and cannot be used in indexing.
//!
//! # Example
//! While each range type in the standard library is either bounded inclusively below or unbounded
//! below, each range type provided in this crate is bounded exclusively below. Compare, for
//! example, [`RangeFrom`] with [`RangeFromExclusive`].
//!
//! ```
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
//! ```
//!
//! [`Iterator`]: core::iter::Iterator
//! [`RangeFrom`]: core::ops::RangeFrom

#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod range_from_exclusive;
mod range_from_exclusive_to_exclusive;
mod range_from_exclusive_to_inclusive;
#[cfg(feature = "serde")]
mod string;

pub use range_from_exclusive::{IterRangeFromExclusive, RangeFromExclusive};
pub use range_from_exclusive_to_exclusive::{
    IterRangeFromExclusiveToExclusive, RangeFromExclusiveToExclusive,
};
pub use range_from_exclusive_to_inclusive::{
    IterRangeFromExclusiveToInclusive, RangeFromExclusiveToInclusive,
};
