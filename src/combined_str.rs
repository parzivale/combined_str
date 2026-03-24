use core::{fmt::Display, ops::Add};

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
/// Construct one with the [`strs!`](crate::strs) macro:
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
    #[doc(hidden)]
    pub fn new(strs: [&'a str; N]) -> Self {
        Self { strs }
    }

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

#[cfg(feature = "nightly")]
impl<'a, const N: usize> Add<&'a str> for CombinedStr<'a, N>
where
    [(); N + 1]:,
{
    type Output = CombinedStr<'a, { N + 1 }>;

    /// Appends a string slice as a new segment, producing a `CombinedStr`
    /// with one additional segment.
    ///
    /// Requires the `nightly` feature.
    fn add(self, rhs: &'a str) -> Self::Output {
        let mut out = [""; N + 1];
        let mut i = 0;
        while i < N {
            out[i] = self.strs[i];
            i += 1;
        }
        out[N] = rhs;
        CombinedStr::new(out)
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

#[cfg(feature = "nightly")]
impl<'a, const N: usize, const M: usize> Add<CombinedStr<'a, M>> for CombinedStr<'a, N>
where
    [(); N + M]:,
{
    type Output = CombinedStr<'a, { N + M }>;

    /// Concatenates two `CombinedStr`s into one whose segment count is the sum
    /// of the operands' segment counts.
    ///
    /// Requires the `nightly` feature.
    fn add(self, rhs: CombinedStr<'a, M>) -> Self::Output {
        let mut out = [""; N + M];
        let mut i = 0;
        while i < N {
            out[i] = self.strs[i];
            i += 1;
        }
        while i < N + M {
            out[i] = rhs.strs[i - N];
            i += 1;
        }
        CombinedStr::new(out)
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
