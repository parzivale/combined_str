# combined_str

[![docs.rs](https://docs.rs/combined_str/badge.svg)](https://docs.rs/combined_str)

A `no_std` Rust library providing `CombinedStr`, a zero-copy, const-generic string combinator that holds N string slices and presents them as a single logical string — without allocating.

## Features

- **`no_std` compatible** — works in embedded and bare-metal environments
- **Zero-copy** — holds `&str` references, no heap allocation required
- **`alloc` feature** — enables conversion to `String` and `Cow<str>`, plus `+=` / `+` operators and equality comparisons
- **`Display`** — prints all segments as one contiguous string
- **Iterator** — iterate over the individual `&str` segments

## Usage

```toml
[dependencies]
combined_str = "0.1"
```

### Basic example

```rust
use combined_str::strs;

let s = strs!["hello", ", ", "world"];
println!("{}", s);          // hello, world
assert_eq!(s.len(), 12);
assert!(!s.is_empty());
```

### Construction from an array or `&str`

```rust
use combined_str::CombinedStr;

let s = CombinedStr::from(["foo", "bar"]);
let s = CombinedStr::from("hello"); // single segment
```

### Zero-allocation equality comparison

```rust
use combined_str::strs;

assert!(strs!["foo", "bar"] == *"foobar");
```

### Appending to a `String` or `Cow<str>` (requires `alloc` feature)

```rust
use combined_str::strs;

let mut owned = String::from("prefix: ");
owned += strs!["foo", "bar"];
assert_eq!(owned, "prefix: foobar");
```

### Equality with `String` and `Cow<str>` (requires `alloc` feature)

```rust
use std::borrow::Cow;
use combined_str::strs;

assert!(strs!["foo", "bar"] == String::from("foobar"));
assert!(strs!["foo", "bar"] == Cow::Borrowed("foobar"));

// symmetric
assert!(String::from("foobar") == strs!["foo", "bar"]);
assert!(Cow::Borrowed("foobar") == strs!["foo", "bar"]);
```

### Converting to `String` or `Cow<str>` (requires `alloc` feature)

```rust
use std::borrow::Cow;
use combined_str::strs;

let s: String = strs!["a", "b", "c"].into();
assert_eq!(s, "abc");

let c: Cow<str> = strs!["x", "y"].into();
assert_eq!(c, "xy");
```

## API

### `CombinedStr<'a, N>`

| Method | Description |
|---|---|
| `len() -> usize` | Total byte length across all segments |
| `is_empty() -> bool` | True if total length is zero |
| `as_bytes() -> [&[u8]; N]` | Byte slices for each segment |
| `as_pointer() -> [*const u8; N]` | Raw pointers for each segment |
| `as_ref() -> &[&str]` | Underlying segments as a slice |

Implements: `Display`, `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `IntoIterator`, `Default`, `From<[&str; N]>`, `From<&str>` (N=1), `PartialEq<str>`.

With `alloc`: `From<CombinedStr> for String`, `From<CombinedStr> for Cow<str>`, `AddAssign`/`Add` for `String` and `Cow<str>`, `PartialEq<String>`, `PartialEq<Cow<str>>` (all symmetric).

### `strs!` macro

Constructs a `CombinedStr` from a comma-separated list of string expressions:

```rust
use combined_str::strs;

let s = strs!["part1", "part2", "part3"];
```

## Feature flags

| Flag | Default | Description |
|---|---|---|
| `alloc` | enabled | Enables `String`/`Cow` support via the `alloc` crate |

## License

MIT OR Apache-2.0
