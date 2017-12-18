Trimmer Derive
==============

[Trimmer](https://crates.io/crates/trimmer/) |
[Docs](https://docs.rs/trimmer_derive/) |
[Github](https://github.com/tailhook/trimmer-derive) |
[Crate](https://crates.io/crates/trimmer_derive)


This crate allows to derive `trimmer::Variable` trait.

Examples:

```rust
// Derives `.x` and `.y` attributes
#[derive(Variable)]
struct Point {
    x: u32,
    y: u32,
}

// Forwards all methods to the internal type
#[derive(Variable)]
struct NewType(SomeVarType);
```


License
=======

Licensed under either of

* Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
