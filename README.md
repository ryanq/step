`step`
============

[![Build Status][travis-badge]][travis-ci]
[![Crate on crates.io][crates.io-badge]][crates.io]

`step` is a crate that provides the trait `Step`, which allows for unit
steps and arbitrary steps on numeric types.

Documentation can be found on [docs.rs].

Using `step`
------------------

Add the crate to the dependencies section of Cargo.toml:

```toml
[dependencies]
step = { git = "https://github.com/ryanq/step" }
```

Then import the crate and type in your source:

```rust
extern crate step;

use step::Step;
```

Then you can use the functions for incrementing and decrementing numbers
or implement it on your own types:

```rust
let number = 42;

assert_eq!(number.next(), 43);
assert_eq!(number.next_by(&3), 45);
assert_eq!(number.prev(), 41);
assert_eq!(number.prev_by(&3), 39);
```

```rust
struct Foo {
    bar: i32,
}

impl Step for Foo {
    fn next(&self) -> Self { Foo { bar: self.bar + 1 } }
    fn next_by(&self, by: &Self) -> Self { Foo { bar: self.bar + *by } }
    fn prev(&self) -> Self { Foo { bar: self.bar - 1 } }
    fn prev_by(&self, by: &Self) -> Self { Foo { bar: self.bar - *by } }
}
```

[travis-badge]: https://travis-ci.org/ryanq/step.svg?branch=master
[travis-ci]: https://travis-ci.org/ryanq/step
[crates.io-badge]: https://img.shields.io/crates/v/step.svg
[crates.io]: https://crates.io/crates/step
[docs.rs]: https://docs.rs/step/0.1.0/step/