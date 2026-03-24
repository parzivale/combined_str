use core::{fmt::Display, ops};

#[cfg(feature = "alloc")]
use core::ops::AddAssign;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;

use crate::CombinedStr;

mod sealed {
    use core::ops;

    pub trait Sealed {}

    impl Sealed for ops::Range<usize> {}
    impl Sealed for ops::RangeTo<usize> {}
    impl Sealed for ops::RangeFrom<usize> {}
    impl Sealed for ops::RangeFull {}
    impl Sealed for ops::RangeInclusive<usize> {}
    impl Sealed for ops::RangeToInclusive<usize> {}
    impl Sealed for (ops::Bound<usize>, ops::Bound<usize>) {}
}

/// A sealed trait for indexing into a [`CombinedStr`] by range, returning
/// a [`CombinedStrView`].
///
/// Implemented for all standard range types (`Range<usize>`, `RangeTo`,
/// `RangeFrom`, `RangeFull`, `RangeInclusive`, `RangeToInclusive`) as well
/// as `(Bound<usize>, Bound<usize>)` tuples and the `core::range` variants.
///
/// When the range falls within a single segment the view contains just that
/// sub-slice; when it spans multiple segments the view borrows the trimmed
/// first and last slices plus any whole segments in between.
///
/// # Examples
///
/// ```
/// use combined_str::{strs, CombinedStrIndex};
///
/// let s = strs!["hello", " ", "world"];
/// let view = CombinedStrIndex::get(&(3..9), &s).unwrap();
/// assert!(view == *"lo wor");
/// ```
pub trait CombinedStrIndex<const N: usize>: sealed::Sealed {
    /// Returns a [`CombinedStrView`] for the given range, or `None` if
    /// the range is out of bounds.
    fn get<'a>(&self, slice: &'a CombinedStr<'a, N>) -> Option<CombinedStrView<'a>>;

    /// Returns a [`CombinedStrView`] for the given range.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    fn index<'a>(self, slice: &'a CombinedStr<'a, N>) -> CombinedStrView<'a>;
}

/// A borrowed view into a [`CombinedStr`], representing a sub-range
/// that may span multiple segments.
///
/// Internally stores the trimmed first slice, any whole middle segments
/// (borrowed from the original segments array), and the trimmed last slice.
/// When the range falls within a single segment, `middle` is empty and
/// `last` is `""`.
#[derive(Debug, Clone, Copy)]
pub struct CombinedStrView<'a> {
    first: &'a str,
    middle: &'a [&'a str],
    last: &'a str,
}

impl<'a> CombinedStrView<'a> {
    /// Returns the total byte length of this view.
    pub fn len(&self) -> usize {
        self.first.len() + self.middle.iter().map(|s| s.len()).sum::<usize>() + self.last.len()
    }

    /// Returns `true` if this view contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the segments of this view.
    fn segments(&self) -> impl Iterator<Item = &'a str> + '_ {
        core::iter::once(self.first)
            .chain(self.middle.iter().copied())
            .chain(core::iter::once(self.last))
    }

    /// Compares segments against a contiguous byte slice.
    fn segments_eq_bytes(segments: impl Iterator<Item = &'a str>, other: &[u8]) -> bool {
        let mut remaining = other;
        for seg in segments {
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

    /// Resolves a `(Bound<usize>, Bound<usize>)` pair into a half-open `start..end`
    /// range, clamped to `len`. Returns `None` if the range is invalid.
    fn resolve_bounds(
        bounds: &(ops::Bound<usize>, ops::Bound<usize>),
        len: usize,
    ) -> Option<(usize, usize)> {
        let start = match bounds.0 {
            ops::Bound::Included(s) => s,
            ops::Bound::Excluded(s) => s.checked_add(1)?,
            ops::Bound::Unbounded => 0,
        };
        let end = match bounds.1 {
            ops::Bound::Included(e) => e.checked_add(1)?,
            ops::Bound::Excluded(e) => e,
            ops::Bound::Unbounded => len,
        };
        if start > end || end > len {
            return None;
        }
        Some((start, end))
    }

    /// Produces a `CombinedStrView` for the byte range `start..end`
    /// by walking the segments of a `CombinedStr`.
    fn from_range<const N: usize>(
        slice: &'a CombinedStr<'a, N>,
        start: usize,
        end: usize,
    ) -> CombinedStrView<'a> {
        if start == end {
            return CombinedStrView {
                first: "",
                middle: &[],
                last: "",
            };
        }

        let mut offset = 0;
        let mut first_seg = None;
        let mut last_seg = None;

        for (i, seg) in slice.strs.iter().enumerate() {
            let seg_end = offset + seg.len();
            if first_seg.is_none() && start < seg_end {
                first_seg = Some(i);
            }
            if end <= seg_end {
                last_seg = Some(i);
                break;
            }
            offset = seg_end;
        }

        let first_idx = first_seg.unwrap();
        let last_idx = last_seg.unwrap();

        // Recompute offset for first_idx
        let first_offset: usize = slice.strs[..first_idx].iter().map(|s| s.len()).sum();
        let local_start = start - first_offset;

        if first_idx == last_idx {
            let local_end = end - first_offset;
            CombinedStrView {
                first: &slice.strs[first_idx][local_start..local_end],
                middle: &[],
                last: "",
            }
        } else {
            let last_offset: usize = slice.strs[..last_idx].iter().map(|s| s.len()).sum();
            let local_end = end - last_offset;
            CombinedStrView {
                first: &slice.strs[first_idx][local_start..],
                middle: &slice.strs[first_idx + 1..last_idx],
                last: &slice.strs[last_idx][..local_end],
            }
        }
    }
}

impl Display for CombinedStrView<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.first)?;
        for s in self.middle {
            f.write_str(s)?;
        }
        f.write_str(self.last)
    }
}

impl PartialEq for CombinedStrView<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        // Lengths match, so compare byte-by-byte across segments.
        // When one side exhausts, all bytes have been compared, so they're equal.
        let mut a = self.segments();
        let mut b = other.segments();
        let mut a_buf: &[u8] = &[];
        let mut b_buf: &[u8] = &[];
        loop {
            if a_buf.is_empty() {
                match a.next() {
                    Some(s) => a_buf = s.as_bytes(),
                    None => return true,
                }
                continue;
            }
            if b_buf.is_empty() {
                match b.next() {
                    Some(s) => b_buf = s.as_bytes(),
                    None => return true,
                }
                continue;
            }
            let n = a_buf.len().min(b_buf.len());
            if a_buf[..n] != b_buf[..n] {
                return false;
            }
            a_buf = &a_buf[n..];
            b_buf = &b_buf[n..];
        }
    }
}

impl Eq for CombinedStrView<'_> {}

impl PartialEq<str> for CombinedStrView<'_> {
    fn eq(&self, other: &str) -> bool {
        CombinedStrView::segments_eq_bytes(self.segments(), other.as_bytes())
    }
}

impl<'a, const N: usize> PartialEq<CombinedStr<'a, N>> for CombinedStrView<'_> {
    fn eq(&self, other: &CombinedStr<'a, N>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut a = self.segments();
        let mut b = other.strs.iter().copied();
        let mut a_buf: &[u8] = &[];
        let mut b_buf: &[u8] = &[];
        loop {
            if a_buf.is_empty() {
                match a.next() {
                    Some(s) => a_buf = s.as_bytes(),
                    None => return true,
                }
                continue;
            }
            if b_buf.is_empty() {
                match b.next() {
                    Some(s) => b_buf = s.as_bytes(),
                    None => return true,
                }
                continue;
            }
            let n = a_buf.len().min(b_buf.len());
            if a_buf[..n] != b_buf[..n] {
                return false;
            }
            a_buf = &a_buf[n..];
            b_buf = &b_buf[n..];
        }
    }
}

impl<'a, const N: usize> PartialEq<CombinedStrView<'a>> for CombinedStr<'a, N> {
    fn eq(&self, other: &CombinedStrView<'a>) -> bool {
        other == self
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<String> for CombinedStrView<'_> {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<CombinedStrView<'_>> for String {
    fn eq(&self, other: &CombinedStrView<'_>) -> bool {
        other == self.as_str()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<Cow<'_, str>> for CombinedStrView<'_> {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self == other.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<CombinedStrView<'_>> for Cow<'_, str> {
    fn eq(&self, other: &CombinedStrView<'_>) -> bool {
        other == self.as_ref()
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<CombinedStrView<'a>> for String {
    fn from(value: CombinedStrView<'a>) -> Self {
        let mut s = String::with_capacity(value.len());
        for seg in value.segments() {
            s.push_str(seg);
        }
        s
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<CombinedStrView<'a>> for Cow<'a, str> {
    fn from(value: CombinedStrView<'a>) -> Self {
        Cow::Owned(String::from(value))
    }
}

#[cfg(feature = "alloc")]
impl<'a> ops::Add<CombinedStrView<'a>> for String {
    type Output = String;

    fn add(mut self, rhs: CombinedStrView<'a>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a> AddAssign<CombinedStrView<'a>> for String {
    #[inline]
    fn add_assign(&mut self, rhs: CombinedStrView<'a>) {
        for seg in rhs.segments() {
            self.push_str(seg);
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> ops::Add<CombinedStrView<'a>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    fn add(mut self, rhs: CombinedStrView<'a>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a> AddAssign<CombinedStrView<'a>> for Cow<'a, str> {
    fn add_assign(&mut self, rhs: CombinedStrView<'a>) {
        if self.is_empty() {
            *self = Cow::Owned(String::from(rhs))
        } else if !rhs.is_empty() {
            if let Cow::Borrowed(lhs) = *self {
                let mut s = String::with_capacity(lhs.len() + rhs.len());
                s.push_str(lhs);
                *self = Cow::Owned(s);
            }
            *self.to_mut() += rhs;
        }
    }
}

/// An iterator over the string segments of a [`CombinedStrView`].
struct CombinedStrViewIter<'a> {
    first: Option<&'a str>,
    middle: core::slice::Iter<'a, &'a str>,
    last: Option<&'a str>,
}

impl<'a> Iterator for CombinedStrViewIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first) = self.first.take() {
            return Some(first);
        }
        if let Some(&seg) = self.middle.next() {
            return Some(seg);
        }
        self.last.take()
    }
}

impl<'a> IntoIterator for CombinedStrView<'a> {
    type Item = &'a str;
    type IntoIter = CombinedStrViewIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CombinedStrViewIter {
            first: Some(self.first),
            middle: self.middle.iter(),
            last: Some(self.last),
        }
    }
}

macro_rules! impl_combined_str_index {
    ($ty:ty, |$self:ident, $len:ident| -> ($start:expr, $end:expr)) => {
        impl<const N: usize> CombinedStrIndex<N> for $ty {
            fn get<'a>(&self, slice: &'a CombinedStr<'a, N>) -> Option<CombinedStrView<'a>> {
                let $self = self;
                let $len = slice.len();
                let (start, end) = ($start, $end);
                if start > end || end > $len {
                    return None;
                }
                Some(CombinedStrView::from_range(slice, start, end))
            }

            fn index<'a>(self, slice: &'a CombinedStr<'a, N>) -> CombinedStrView<'a> {
                CombinedStrIndex::get(&self, slice).expect("index out of bounds")
            }
        }
    };
}

impl<const N: usize> CombinedStrIndex<N> for (ops::Bound<usize>, ops::Bound<usize>) {
    fn get<'a>(&self, slice: &'a CombinedStr<'a, N>) -> Option<CombinedStrView<'a>> {
        let (start, end) = CombinedStrView::resolve_bounds(self, slice.len())?;
        Some(CombinedStrView::from_range(slice, start, end))
    }

    fn index<'a>(self, slice: &'a CombinedStr<'a, N>) -> CombinedStrView<'a> {
        CombinedStrIndex::get(&self, slice).expect("index out of bounds")
    }
}

impl_combined_str_index!(ops::Range<usize>, |s, len| -> (s.start, s.end));
impl_combined_str_index!(ops::RangeTo<usize>, |s, len| -> (0, s.end));
impl_combined_str_index!(ops::RangeFrom<usize>, |s, len| -> (s.start, len));
impl_combined_str_index!(ops::RangeFull, |_s, len| -> (0, len));
impl_combined_str_index!(ops::RangeInclusive<usize>, |s, _len| -> (*s.start(), s.end().saturating_add(1)));
impl_combined_str_index!(ops::RangeToInclusive<usize>, |s, _len| -> (0, s.end.saturating_add(1)));
