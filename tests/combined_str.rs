extern crate alloc;

use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Bound;

use combined_str::{strs, CombinedStr, CombinedStrIndex};

// --- CombinedStr basic tests ---

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

#[test]
fn partial_eq_string() {
    assert!(strs!["foo", "bar"] == String::from("foobar"));
    assert!(strs!["foo", "bar"] != String::from("foobaz"));
}

#[test]
fn partial_eq_string_symmetric() {
    assert!(String::from("foobar") == strs!["foo", "bar"]);
    assert!(String::from("foobaz") != strs!["foo", "bar"]);
}

#[test]
fn partial_eq_cow() {
    assert!(strs!["foo", "bar"] == Cow::Borrowed("foobar"));
    assert!(strs!["foo", "bar"] != Cow::Borrowed("foobaz"));
}

#[test]
fn partial_eq_cow_symmetric() {
    assert!(Cow::Borrowed("foobar") == strs!["foo", "bar"]);
    assert!(Cow::Borrowed("foobaz") != strs!["foo", "bar"]);
}

#[test]
fn display_concatenates_segments() {
    let s = strs!["hello", ", ", "world"];
    assert_eq!(format!("{}", s), "hello, world");
}

#[test]
fn display_single_segment() {
    let s = strs!["only"];
    assert_eq!(format!("{}", s), "only");
}

#[test]
fn iterator_yields_segments_in_order() {
    let s = strs!["a", "b", "c"];
    let collected: Vec<&str> = s.into_iter().collect();
    assert_eq!(collected, vec!["a", "b", "c"]);
}

#[test]
fn iterator_empty_segments() {
    let s = strs!["", "x", ""];
    let collected: Vec<&str> = s.into_iter().collect();
    assert_eq!(collected, vec!["", "x", ""]);
}

#[test]
fn string_from_combined_str() {
    let s: String = strs!["foo", "bar"].into();
    assert_eq!(s, "foobar");
}

#[test]
fn cow_from_combined_str() {
    let c: Cow<str> = strs!["foo", "bar"].into();
    assert_eq!(c, "foobar");
}

#[test]
fn string_add_assign() {
    let mut s = String::from("hello ");
    s += strs!["world", "!"];
    assert_eq!(s, "hello world!");
}

#[test]
fn string_add() {
    let s = String::from("hi ") + strs!["there", "!"];
    assert_eq!(s, "hi there!");
}

#[test]
fn string_add_assign_empty_rhs() {
    let mut s = String::from("unchanged");
    s += strs!["", ""];
    assert_eq!(s, "unchanged");
}

#[test]
fn cow_add_assign_to_empty() {
    let mut c: Cow<str> = Cow::Borrowed("");
    c += strs!["hello", " world"];
    assert_eq!(c, "hello world");
}

#[test]
fn cow_add_assign_to_nonempty() {
    let mut c: Cow<str> = Cow::Borrowed("start ");
    c += strs!["end"];
    assert_eq!(c, "start end");
}

#[test]
fn cow_add() {
    let c: Cow<str> = Cow::Borrowed("x");
    let result = c + strs!["y", "z"];
    assert_eq!(result, "xyz");
}

#[cfg(feature = "nightly")]
#[test]
fn add_combined_strs() {
    let a = strs!["hello", " "];
    let b = strs!["world", "!"];
    let c = a + b;
    assert!(c == *"hello world!");
    assert_eq!(c.len(), 12);
}

#[cfg(feature = "nightly")]
#[test]
fn add_combined_strs_empty() {
    let a = strs!["foo"];
    let b = strs!["", ""];
    let c = a + b;
    assert!(c == *"foo");
    assert_eq!(c.len(), 3);
}

#[cfg(feature = "nightly")]
#[test]
fn add_combined_strs_segments() {
    let a = strs!["a", "b"];
    let b = strs!["c"];
    let c = a + b;
    let segs: &[&str] = c.as_ref();
    assert_eq!(segs, &["a", "b", "c"]);
}

#[cfg(feature = "nightly")]
#[test]
fn add_str_slice() {
    let a = strs!["hello", " "];
    let b = a + "world";
    assert!(b == *"hello world");
    assert_eq!(b.len(), 11);
}

#[cfg(feature = "nightly")]
#[test]
fn add_str_slice_to_single() {
    let a = strs!["hi"];
    let b = a + "!";
    let segs: &[&str] = b.as_ref();
    assert_eq!(segs, &["hi", "!"]);
}

#[cfg(feature = "nightly")]
#[test]
fn add_chained() {
    let a = strs!["a"];
    let b = a + "b" + "c";
    assert!(b == *"abc");
    assert_eq!(b.len(), 3);
}

// --- CombinedStrIndex / CombinedStrView tests ---

#[test]
fn get_range_within_single_segment() {
    let s = strs!["hello", " world"];
    let view = CombinedStrIndex::get(&(1..4), &s).unwrap();
    assert!(view == *"ell");
    assert_eq!(view.len(), 3);
}

#[test]
fn get_range_spanning_two_segments() {
    let s = strs!["hello", " world"];
    let view = CombinedStrIndex::get(&(3..8), &s).unwrap();
    assert!(view == *"lo wo");
    assert_eq!(view.len(), 5);
}

#[test]
fn get_range_spanning_all_segments() {
    let s = strs!["ab", "cd", "ef"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(view == *"bcde");
    assert_eq!(view.len(), 4);
}

#[test]
fn get_range_full() {
    let s = strs!["foo", "bar"];
    let view = CombinedStrIndex::get(&(..), &s).unwrap();
    assert!(view == *"foobar");
    assert_eq!(view.len(), 6);
}

#[test]
fn get_range_from() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(2..), &s).unwrap();
    assert!(view == *"cdef");
}

#[test]
fn get_range_to() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(..4), &s).unwrap();
    assert!(view == *"abcd");
}

#[test]
fn get_range_inclusive() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..=4), &s).unwrap();
    assert!(view == *"bcde");
}

#[test]
fn get_range_to_inclusive() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(..=2), &s).unwrap();
    assert!(view == *"abc");
}

#[test]
fn get_empty_range() {
    let s = strs!["hello", "world"];
    let view = CombinedStrIndex::get(&(3..3), &s).unwrap();
    assert!(view.is_empty());
}

#[test]
fn get_out_of_bounds_returns_none() {
    let s = strs!["abc"];
    assert!(CombinedStrIndex::get(&(0..10), &s).is_none());
}

#[test]
fn get_invalid_range_returns_none() {
    let s = strs!["abc"];
    assert!(CombinedStrIndex::get(&(3..1), &s).is_none());
}

#[test]
fn get_at_segment_boundary() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(3..6), &s).unwrap();
    assert!(view == *"def");
}

#[test]
fn get_exact_single_segment() {
    let s = strs!["abc", "def", "ghi"];
    let view = CombinedStrIndex::get(&(3..6), &s).unwrap();
    assert!(view == *"def");
}

#[test]
fn index_range_works() {
    let s = strs!["hello", " ", "world"];
    let view = CombinedStrIndex::index(2..9, &s);
    assert!(view == *"llo wor");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn index_out_of_bounds_panics() {
    let s = strs!["abc"];
    CombinedStrIndex::index(0..10, &s);
}

#[test]
fn view_with_empty_segments_skipped() {
    let s = strs!["", "abc", "", "def", ""];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(view == *"bcde");
}

#[test]
fn view_display() {
    let s = strs!["hello", " ", "world"];
    let view = CombinedStrIndex::get(&(3..9), &s).unwrap();
    assert_eq!(format!("{}", view), "lo wor");
}

#[test]
fn get_bounds_tuple() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(Bound::Included(1), Bound::Excluded(5)), &s).unwrap();
    assert!(view == *"bcde");
}

#[test]
fn get_bounds_tuple_unbounded() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(Bound::Unbounded, Bound::Unbounded), &s).unwrap();
    assert!(view == *"abcdef");
}

#[test]
fn get_bounds_excluded_start() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(Bound::Excluded(1), Bound::Excluded(5)), &s).unwrap();
    assert!(view == *"cde");
}

#[test]
fn get_bounds_included_end() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(Bound::Included(0), Bound::Included(3)), &s).unwrap();
    assert!(view == *"abcd");
}

#[test]
fn get_bounds_excluded_start_overflow_returns_none() {
    let s = strs!["abc"];
    assert!(
        CombinedStrIndex::get(&(Bound::Excluded(usize::MAX), Bound::Excluded(3)), &s,).is_none()
    );
}

#[test]
fn get_bounds_included_end_overflow_returns_none() {
    let s = strs!["abc"];
    assert!(
        CombinedStrIndex::get(&(Bound::Included(0), Bound::Included(usize::MAX)), &s,).is_none()
    );
}

#[test]
fn index_range_to_works() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::index(..4, &s);
    assert!(view == *"abcd");
}

#[test]
fn index_range_from_works() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::index(2.., &s);
    assert!(view == *"cdef");
}

#[test]
fn index_range_full_works() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::index(.., &s);
    assert!(view == *"abcdef");
}

#[test]
fn index_range_inclusive_works() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::index(1..=4, &s);
    assert!(view == *"bcde");
}

#[test]
fn index_range_to_inclusive_works() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::index(..=2, &s);
    assert!(view == *"abc");
}

#[test]
fn view_eq_str_shorter_segment() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(0..6), &s).unwrap();
    assert!(view != *"abc");
    assert!(view != *"abcdefg");
}

#[test]
fn view_eq_str_mismatch() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(0..6), &s).unwrap();
    assert!(view != *"abcxyz");
}

// --- CombinedStrView trait coverage tests ---

#[test]
fn view_eq_view_same() {
    let s = strs!["abc", "def"];
    let a = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let b = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert_eq!(a, b);
}

#[test]
fn view_eq_view_different_segments_same_content() {
    let s1 = strs!["ab", "cd", "ef"];
    let s2 = strs!["abcdef"];
    let a = CombinedStrIndex::get(&(0..6), &s1).unwrap();
    let b = CombinedStrIndex::get(&(0..6), &s2).unwrap();
    assert_eq!(a, b);
}

#[test]
fn view_ne_view() {
    let s = strs!["abc", "def"];
    let a = CombinedStrIndex::get(&(0..3), &s).unwrap();
    let b = CombinedStrIndex::get(&(3..6), &s).unwrap();
    assert_ne!(a, b);
}

#[test]
fn view_ne_view_different_len() {
    let s = strs!["abc", "def"];
    let a = CombinedStrIndex::get(&(0..3), &s).unwrap();
    let b = CombinedStrIndex::get(&(0..5), &s).unwrap();
    assert_ne!(a, b);
}

#[test]
fn view_eq_combined_str() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(0..6), &s).unwrap();
    assert!(view == s);
}

#[test]
fn view_eq_combined_str_symmetric() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(0..6), &s).unwrap();
    assert!(s == view);
}

#[test]
fn view_ne_combined_str() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(view != s);
}

#[test]
fn view_eq_string() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(view == String::from("bcde"));
    assert!(view != String::from("xxxx"));
}

#[test]
fn view_eq_string_symmetric() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(String::from("bcde") == view);
    assert!(String::from("xxxx") != view);
}

#[test]
fn view_eq_cow() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(view == Cow::Borrowed("bcde"));
    assert!(view != Cow::Borrowed("xxxx"));
}

#[test]
fn view_eq_cow_symmetric() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    assert!(Cow::Borrowed("bcde") == view);
    assert!(Cow::Borrowed("xxxx") != view);
}

#[test]
fn string_from_view() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let string: String = view.into();
    assert_eq!(string, "bcde");
}

#[test]
fn cow_from_view() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let cow: Cow<str> = view.into();
    assert_eq!(cow, "bcde");
}

#[test]
fn string_add_view() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let result = String::from("x") + view;
    assert_eq!(result, "xbcde");
}

#[test]
fn string_add_assign_view() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let mut result = String::from("x");
    result += view;
    assert_eq!(result, "xbcde");
}

#[test]
fn cow_add_view() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let result = Cow::Borrowed("x") + view;
    assert_eq!(result, "xbcde");
}

#[test]
fn cow_add_assign_view_to_empty() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let mut cow: Cow<str> = Cow::Borrowed("");
    cow += view;
    assert_eq!(cow, "bcde");
}

#[test]
fn cow_add_assign_view_to_nonempty() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..5), &s).unwrap();
    let mut cow: Cow<str> = Cow::Borrowed("x");
    cow += view;
    assert_eq!(cow, "xbcde");
}

#[test]
fn cow_add_assign_empty_view() {
    let s = strs!["abc"];
    let view = CombinedStrIndex::get(&(0..0), &s).unwrap();
    let mut cow: Cow<str> = Cow::Borrowed("x");
    cow += view;
    assert_eq!(cow, "x");
}

#[test]
fn view_into_iter() {
    let s = strs!["abc", "def", "ghi"];
    let view = CombinedStrIndex::get(&(1..8), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    // first="bc", middle=["def"], last="gh"
    assert_eq!(collected, vec!["bc", "def", "gh"]);
}

#[test]
fn view_into_iter_single_segment() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(1..3), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    // first="bc", middle=[], last=""
    assert_eq!(collected, vec!["bc", ""]);
}

// --- More iterator tests ---

#[test]
fn combined_str_iter_single_segment() {
    let s = strs!["hello"];
    let collected: Vec<&str> = s.into_iter().collect();
    assert_eq!(collected, vec!["hello"]);
}

#[test]
fn combined_str_iter_many_segments() {
    let s = strs!["a", "b", "c", "d", "e"];
    let collected: Vec<&str> = s.into_iter().collect();
    assert_eq!(collected, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn combined_str_iter_all_empty() {
    let s = strs!["", "", ""];
    let collected: Vec<&str> = s.into_iter().collect();
    assert_eq!(collected, vec!["", "", ""]);
}

#[test]
fn combined_str_iter_count() {
    let s = strs!["a", "b", "c"];
    assert_eq!(s.into_iter().count(), 3);
}

#[test]
fn combined_str_iter_fold_concat() {
    let s = strs!["hello", " ", "world"];
    let result: String = s.into_iter().collect();
    assert_eq!(result, "hello world");
}

#[test]
fn combined_str_iter_nth() {
    let s = strs!["a", "b", "c", "d"];
    let mut iter = s.into_iter();
    assert_eq!(iter.nth(2), Some("c"));
    assert_eq!(iter.next(), Some("d"));
    assert_eq!(iter.next(), None);
}

#[test]
fn combined_str_iter_exhausted_returns_none() {
    let s = strs!["a"];
    let mut iter = s.into_iter();
    assert_eq!(iter.next(), Some("a"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn view_iter_full_range() {
    let s = strs!["abc", "def"];
    let view = CombinedStrIndex::get(&(..), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    // first="abc", middle=[], last="def"
    assert_eq!(collected, vec!["abc", "def"]);
}

#[test]
fn view_iter_empty_range() {
    let s = strs!["abc"];
    let view = CombinedStrIndex::get(&(1..1), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    assert_eq!(collected, vec!["", ""]);
}

#[test]
fn view_iter_with_empty_middle_segments() {
    let s = strs!["ab", "", "", "cd"];
    let view = CombinedStrIndex::get(&(0..4), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    assert_eq!(collected, vec!["ab", "", "", "cd"]);
}

#[test]
fn view_iter_fold_concat() {
    let s = strs!["hello", " ", "world"];
    let view = CombinedStrIndex::get(&(2..10), &s).unwrap();
    let result: String = view.into_iter().collect();
    assert_eq!(result, "llo worl");
}

#[test]
fn view_iter_count() {
    let s = strs!["abc", "def", "ghi"];
    let view = CombinedStrIndex::get(&(1..8), &s).unwrap();
    // first="bc", middle=["def"], last="gh" => 3 items
    assert_eq!(view.into_iter().count(), 3);
}

#[test]
fn view_iter_exhausted_returns_none() {
    let s = strs!["abc"];
    let view = CombinedStrIndex::get(&(0..2), &s).unwrap();
    let mut iter = view.into_iter();
    assert_eq!(iter.next(), Some("ab"));
    assert_eq!(iter.next(), Some(""));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn combined_str_collect_into_string() {
    let s = strs!["foo", "bar", "baz"];
    let result: String = s.into_iter().collect();
    assert_eq!(result, "foobarbaz");
}

#[test]
fn view_iter_boundary_at_segment_edge() {
    let s = strs!["abc", "def", "ghi"];
    // range exactly covers middle segment
    let view = CombinedStrIndex::get(&(3..6), &s).unwrap();
    let collected: Vec<&str> = view.into_iter().collect();
    assert_eq!(collected, vec!["def", ""]);
}

#[test]
fn combined_str_iter_find() {
    let s = strs!["apple", "banana", "cherry"];
    let found = s.into_iter().find(|s| s.starts_with('b'));
    assert_eq!(found, Some("banana"));
}

#[test]
fn combined_str_iter_position() {
    let s = strs!["a", "b", "c"];
    let pos = s.into_iter().position(|s| s == "c");
    assert_eq!(pos, Some(2));
}

#[test]
fn combined_str_iter_any_all() {
    let s = strs!["abc", "def"];
    assert!(s.into_iter().all(|s| s.len() == 3));
    let s = strs!["abc", "def"];
    assert!(s.into_iter().any(|s| s == "def"));
    let s = strs!["abc", "def"];
    assert!(!s.into_iter().any(|s| s == "xyz"));
}
