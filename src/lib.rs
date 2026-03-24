//! Zero-copy, const-generic string combinator for `no_std` environments.
//!
//! `CombinedStr` holds N string slices and presents them as a single logical
//! string without allocating. Use the [`strs!`] macro to construct one.
//!
//! # Features
//!
//! - **`alloc`** *(default)* — enables conversions to/from `String` and
//!   `Cow<str>`, plus `Add`/`AddAssign` impls for those types.
//! - **`nightly`** — enables the `generic_const_exprs` unstable feature,
//!   which allows concatenating `CombinedStr`s with `+`:
//!   - `CombinedStr<N> + &str` → `CombinedStr<{N + 1}>`
//!   - `CombinedStr<N> + CombinedStr<M>` → `CombinedStr<{N + M}>`
#![no_std]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod combined_str;
mod iter;
mod view;

pub use combined_str::CombinedStr;
pub use view::{CombinedStrIndex, CombinedStrView};

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
