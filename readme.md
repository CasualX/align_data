Align Data
==========

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/align-data.svg)](https://crates.io/crates/align-data)
[![docs.rs](https://docs.rs/align-data/badge.svg)](https://docs.rs/align-data)

Simply increase the alignment of any statics or `include_bytes!`.

Examples
--------

When including raw data through `include_bytes!` it is often used to directly interpret as structured data however it does not align the included bytes according to the structured data's alignment.

This crate fixes this oversight:

```rust
use align_data::{include_aligned, Align4K};

static ALIGNED: &[u8] = include_aligned!(Align4K, "lib.rs");
assert_eq!(ALIGNED.as_ptr() as usize % 0x1000, 0);
```

The value of any constant expression can be forced to the given alignment:

```rust
use align_data::{aligned, Align16};

let five = aligned!(Align16, i32, 5);
assert_eq!(five as *const _ as usize % 0x10, 0);
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
