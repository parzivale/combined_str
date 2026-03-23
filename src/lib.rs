//! Zero-copy, const-generic string combinator for `no_std` environments.
//!
//! `CombinedStr` holds N string slices and presents them as a single logical
//! string without allocating. Use the [`strs!`] macro to construct one.
#![no_std]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]

mod combined_str;
mod iter;
mod view;

pub use combined_str::CombinedStr;
pub use iter::CombinedStrIter;
pub use view::{CombinedStrIndex, CombinedStrView, CombinedStrViewIter};

/// Constructs a [`CombinedStr`] from a comma-separated list of string expressions.
///
/// ```
/// use combined_str::strs;
///
/// let s = strs!["hello", ", ", "world"];
/// assert_eq!(format!("{}", s), "hello, world");
/// ```
#[macro_export]
macro_rules! strs {
    ($($s:expr),* $(,)?) => {
        $crate::CombinedStr::new([$($s),*])
    };
}
