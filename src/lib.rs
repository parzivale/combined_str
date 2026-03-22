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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombinedStr<'a, const N: usize> {
    strs: [&'a str; N],
}

impl<'a, const N: usize> Default for CombinedStr<'a, N> {
    fn default() -> Self {
        Self { strs: [""; N] }
    }
}

impl<'a, const N: usize> CombinedStr<'a, N> {
    pub fn as_bytes(&self) -> [&[u8]; N] {
        self.strs.map(|item| item.as_bytes())
    }

    pub fn as_pointer(&self) -> [*const u8; N] {
        self.strs.map(|item| item as *const str as *const u8)
    }

    pub fn len(&self) -> usize {
        self.strs.iter().map(|str| str.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> Add<CombinedStr<'a, N>> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    fn add(mut self, rhs: CombinedStr<'a, N>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> Add<CombinedStr<'a, N>> for String {
    type Output = String;

    fn add(mut self, rhs: CombinedStr<'a, N>) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> AddAssign<CombinedStr<'a, N>> for Cow<'a, str> {
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
            *self += rhs;
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> From<CombinedStr<'a, N>> for Cow<'a, str> {
    fn from(value: CombinedStr<'a, N>) -> Self {
        FromIterator::from_iter(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> From<CombinedStr<'a, N>> for String {
    fn from(value: CombinedStr<'a, N>) -> Self {
        FromIterator::from_iter(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> AddAssign<CombinedStr<'a, N>> for String {
    #[inline]
    fn add_assign(&mut self, other: CombinedStr<'a, N>) {
        for i in other.strs {
            self.push_str(i);
        }
    }
}

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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for s in &self.strs {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! strs {
    ($($s:expr),* $(,)?) => {
        CombinedStr {
            strs: [$($s),*],
        }
    };
}
