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

assert_eq!(number.step(), 43);
assert_eq!(number.step_by(&3), 45);
assert_eq!(number.step_back(), 41);
assert_eq!(number.step_back_by(&3), 39);
```

```rust
struct Foo {
    bar: i32,
}

impl Step for Foo {
    fn step(&self) -> Self { Foo { bar: self.bar + 1 } }
    fn step_by(&self, by: &Self) -> Self { Foo { bar: self.bar + *by } }
    fn step_back(&self) -> Self { Foo { bar: self.bar - 1 } }
    fn step_back_by(&self, by: &Self) -> Self { Foo { bar: self.bar - *by } }
}
```

[travis-badge]: https://travis-ci.org/ryanq/step.svg?branch=master
[travis-ci]: https://travis-ci.org/ryanq/step
[crates.io-badge]: https://img.shields.io/crates/v/step.svg
[crates.io]: https://crates.io/crates/step
[docs.rs]: https://docs.rs/step/0.2.0/step/