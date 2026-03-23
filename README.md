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
combined_str = "0.4"
```

```rust
use combined_str::strs;

let s = strs!["hello", ", ", "world"];
println!("{}", s);          // hello, world
assert_eq!(s.len(), 12);
```

## Feature flags

| Flag | Default | Description |
|---|---|---|
| `alloc` | enabled | Enables `String`/`Cow` support via the `alloc` crate |
| `nightly` | disabled | Enables `generic_const_exprs` for `CombinedStr<N> + CombinedStr<M>` and `CombinedStr<N> + &str` |

## License

MIT
