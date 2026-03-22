//! Zero-copy, const-generic string combinator for `no_std` environments.
//!
//! `CombinedStr` holds N string slices and presents them as a single logical
//! string without allocating. Use the [`strs!`] macro to construct one.
#![no_std]
use core::fmt::Display;
use core::ops::Add;
#[cfg(feature = "alloc")]
use core::ops::AddAssign;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;

/// A combined view of N string slices, treated as a single contiguous string.
///
/// `CombinedStr` borrows its segments and performs no heap allocation. The
/// const parameter `N` encodes the number of segments at compile time.
///
/// Construct one with the [`strs!`] macro:
///
/// ```
/// use combined_str::strs;
///
/// let s = strs!["hello", ", ", "world"];
/// assert_eq!(s.len(), 12);
/// println!("{}", s); // hello, world
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinedStr<'a, const N: usize> {
    #[doc(hidden)]
    pub strs: [&'a str; N],
}

impl<'a, const N: usize> Default for CombinedStr<'a, N> {
    /// Returns a `CombinedStr` where every segment is an empty string slice.
    fn default() -> Self {
        Self { strs: [""; N] }
    }
}

impl<'a, const N: usize> CombinedStr<'a, N> {
    /// Returns each segment as a byte slice.
    pub fn as_bytes(&self) -> [&[u8]; N] {
        self.strs.map(|item| item.as_bytes())
    }

    /// Returns a raw pointer to the start of each segment's data.
    pub fn as_pointer(&self) -> [*const u8; N] {
        self.strs.map(|item| item as *const str as *const u8)
    }

    /// Returns the total byte length across all segments.
    ///
    /// ```
    /// use combined_str::strs;
    ///
    /// assert_eq!(strs!["foo", "bar"].len(), 6);
    /// ```
    pub fn len(&self) -> usize {
        self.strs.iter().map(|str| str.len()).sum()
    }

    /// Returns `true` if the total length across all segments is zero.
    ///
    /// ```
    /// use combined_str::strs;
    ///
    /// assert!(strs!["", ""].is_empty());
    /// assert!(!strs!["a", ""].is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for CombinedStr<'a, N> {
    /// Constructs a `CombinedStr` from a fixed-size array of string slices.
    fn from(strs: [&'a str; N]) -> Self {
        Self { strs }
    }
}

impl<'a> From<&'a str> for CombinedStr<'a, 1> {
    /// Constructs a single-segment `CombinedStr` from a `&str`.
    fn from(s: &'a str) -> Self {
        Self { strs: [s] }
    }
}

impl<'a, const N: usize> PartialEq<str> for CombinedStr<'a, N> {
    /// Returns `true` if the concatenation of all segments equals `other`,
    /// comparing without any allocation.
    ///
    /// ```
    /// use combined_str::strs;
    ///
    /// assert!(strs!["foo", "bar"] == *"foobar");
    /// assert!(strs!["hello", " ", "world"] == *"hello world");
    /// ```
    fn eq(&self, other: &str) -> bool {
        let mut remaining = other.as_bytes();
        for seg in &self.strs {
            let seg_bytes = seg.as_bytes();
            if remaining.len() < seg_bytes.len() {
                return false;
            }
            if &remaining[..seg_bytes.len()] != seg_bytes {
                return false;
            }
            remaining = &remaining[seg_bytes.len()..];
        }
        remaining.is_empty()
    }
}

impl<'a, const N: usize> AsRef<[&'a str]> for CombinedStr<'a, N> {
    /// Returns the underlying segments as a slice.
    fn as_ref(&self) -> &[&'a str] {
        &self.strs
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> PartialEq<String> for CombinedStr<'a, N> {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> PartialEq<CombinedStr<'a, N>> for String {
    fn eq(&self, other: &CombinedStr<'a, N>) -> bool {
        other == self.as_str()
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> PartialEq<Cow<'_, str>> for CombinedStr<'a, N> {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self == other.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> PartialEq<CombinedStr<'a, N>> for Cow<'_, str> {
    fn eq(&self, other: &CombinedStr<'a, N>) -> bool {
        other == self.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> Add<CombinedStr<'a, N>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    /// Appends all segments of `rhs` to this `Cow<str>`, returning the result.
    fn add(mut self, rhs: CombinedStr<'a, N>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> Add<CombinedStr<'a, N>> for String {
    type Output = String;

    /// Appends all segments of `rhs` to this `String`, returning the result.
    fn add(mut self, rhs: CombinedStr<'a, N>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> AddAssign<CombinedStr<'a, N>> for Cow<'a, str> {
    /// Appends all segments of `rhs` to this `Cow<str>` in place.
    fn add_assign(&mut self, rhs: CombinedStr<'a, N>) {
        if self.is_empty() {
            *self = FromIterator::from_iter(rhs)
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                use alloc::string::String;

                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            *self.to_mut() += rhs;
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> From<CombinedStr<'a, N>> for Cow<'a, str> {
    /// Collects all segments into an owned `Cow<str>`.
    fn from(value: CombinedStr<'a, N>) -> Self {
        FromIterator::from_iter(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> From<CombinedStr<'a, N>> for String {
    /// Collects all segments into a `String`.
    fn from(value: CombinedStr<'a, N>) -> Self {
        FromIterator::from_iter(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> AddAssign<CombinedStr<'a, N>> for String {
    /// Appends all segments of `rhs` to this `String` in place.
    #[inline]
    fn add_assign(&mut self, other: CombinedStr<'a, N>) {
        for i in other.strs {
            self.push_str(i);
        }
    }
}

/// An iterator over the string segments of a [`CombinedStr`].
///
/// Yields each `&str` segment in order. Created by calling
/// [`into_iter`](IntoIterator::into_iter) on a `CombinedStr`.
pub struct CombinedStrIter<'a, const N: usize> {
    strs: [&'a str; N],
    current: usize,
}

impl<'a, const N: usize> IntoIterator for CombinedStr<'a, N> {
    type Item = &'a str;

    type IntoIter = CombinedStrIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        CombinedStrIter {
            strs: self.strs,
            current: 0,
        }
    }
}

impl<'a, const N: usize> Iterator for CombinedStrIter<'a, N> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let str = self.strs.get(self.current);
        self.current += 1;
        str.map(|v| &**v)
    }
}

impl<'a, const N: usize> Display for CombinedStr<'a, N> {
    /// Writes all segments consecutively with no separator.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for s in &self.strs {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::borrow::Cow;
    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(feature = "alloc")]
    use alloc::string::String;
    #[cfg(feature = "alloc")]
    use alloc::vec;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    #[test]
    fn len_sums_segments() {
        let s = strs!["foo", "bar", "baz"];
        assert_eq!(s.len(), 9);
    }

    #[test]
    fn len_with_empty_segments() {
        let s = strs!["", "hello", ""];
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn is_empty_all_empty() {
        let s = strs!["", ""];
        assert!(s.is_empty());
    }

    #[test]
    fn is_empty_false_when_nonempty() {
        let s = strs!["a", ""];
        assert!(!s.is_empty());
    }

    #[test]
    fn default_is_empty() {
        let s = CombinedStr::<'_, 3>::default();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn as_bytes_matches_str_bytes() {
        let s = strs!["hi", "!"];
        let bytes = s.as_bytes();
        assert_eq!(bytes[0], b"hi");
        assert_eq!(bytes[1], b"!");
    }

    #[test]
    fn as_pointer_points_to_str_data() {
        let a = "hello";
        let b = "world";
        let s = strs![a, b];
        let ptrs = s.as_pointer();
        assert_eq!(ptrs[0], a.as_ptr());
        assert_eq!(ptrs[1], b.as_ptr());
    }

    #[test]
    fn equality_same_content() {
        assert_eq!(strs!["x", "y"], strs!["x", "y"]);
    }

    #[test]
    fn equality_different_content() {
        assert_ne!(strs!["x", "y"], strs!["x", "z"]);
    }

    #[test]
    fn clone_and_copy() {
        let s = strs!["a", "b"];
        let cloned = s.clone();
        let copied = s;
        assert_eq!(cloned, copied);
    }

    #[test]
    fn from_array() {
        let s = CombinedStr::from(["hello", " ", "world"]);
        assert_eq!(s.len(), 11);
    }

    #[test]
    fn from_str() {
        let s = CombinedStr::from("hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn partial_eq_str_equal() {
        assert!(strs!["foo", "bar"] == *"foobar");
    }

    #[test]
    fn partial_eq_str_not_equal() {
        assert!(strs!["foo", "bar"] != *"foobaz");
    }

    #[test]
    fn partial_eq_str_prefix_only() {
        assert!(strs!["foo", "bar"] != *"foo");
    }

    #[test]
    fn partial_eq_str_empty() {
        assert!(strs!["", ""] == *"");
    }

    #[test]
    fn as_ref_segments() {
        let s = strs!["a", "b", "c"];
        let segs: &[&str] = s.as_ref();
        assert_eq!(segs, &["a", "b", "c"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn partial_eq_string() {
        assert!(strs!["foo", "bar"] == String::from("foobar"));
        assert!(strs!["foo", "bar"] != String::from("foobaz"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn partial_eq_string_symmetric() {
        assert!(String::from("foobar") == strs!["foo", "bar"]);
        assert!(String::from("foobaz") != strs!["foo", "bar"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn partial_eq_cow() {
        assert!(strs!["foo", "bar"] == Cow::Borrowed("foobar"));
        assert!(strs!["foo", "bar"] != Cow::Borrowed("foobaz"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn partial_eq_cow_symmetric() {
        assert!(Cow::Borrowed("foobar") == strs!["foo", "bar"]);
        assert!(Cow::Borrowed("foobaz") != strs!["foo", "bar"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn display_concatenates_segments() {
        let s = strs!["hello", ", ", "world"];
        assert_eq!(format!("{}", s), "hello, world");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn display_single_segment() {
        let s = strs!["only"];
        assert_eq!(format!("{}", s), "only");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn iterator_yields_segments_in_order() {
        let s = strs!["a", "b", "c"];
        let collected: Vec<&str> = s.into_iter().collect();
        assert_eq!(collected, vec!["a", "b", "c"]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn iterator_empty_segments() {
        let s = strs!["", "x", ""];
        let collected: Vec<&str> = s.into_iter().collect();
        assert_eq!(collected, vec!["", "x", ""]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn string_from_combined_str() {
        let s: String = strs!["foo", "bar"].into();
        assert_eq!(s, "foobar");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cow_from_combined_str() {
        let c: Cow<str> = strs!["foo", "bar"].into();
        assert_eq!(c, "foobar");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn string_add_assign() {
        let mut s = String::from("hello ");
        s += strs!["world", "!"];
        assert_eq!(s, "hello world!");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn string_add() {
        let s = String::from("hi ") + strs!["there", "!"];
        assert_eq!(s, "hi there!");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn string_add_assign_empty_rhs() {
        let mut s = String::from("unchanged");
        s += strs!["", ""];
        assert_eq!(s, "unchanged");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cow_add_assign_to_empty() {
        let mut c: Cow<str> = Cow::Borrowed("");
        c += strs!["hello", " world"];
        assert_eq!(c, "hello world");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cow_add_assign_to_nonempty() {
        let mut c: Cow<str> = Cow::Borrowed("start ");
        c += strs!["end"];
        assert_eq!(c, "start end");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn cow_add() {
        let c: Cow<str> = Cow::Borrowed("x");
        let result = c + strs!["y", "z"];
        assert_eq!(result, "xyz");
    }
}

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
        $crate::CombinedStr {
            strs: [$($s),*],
        }
    };
}
