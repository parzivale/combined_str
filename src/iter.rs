use crate::CombinedStr;

/// An iterator over the string segments of a [`CombinedStr`].
///
/// Yields each `&str` segment in order. Created by calling
/// [`into_iter`](IntoIterator::into_iter) on a `CombinedStr`.
struct CombinedStrIter<'a, const N: usize> {
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
