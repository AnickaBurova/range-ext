# Range-Ext

Range-Ext is a Rust library to handle range intersections. It provides a set of types and methods to determine whether and how two numeric ranges intersect.

## Features

- **Intersection Enumeration**: A simple enumeration helps to identify the type of intersection occurring between ranges. Options include: `Empty`, `Overlap`, and `Full`.
- **Extended Intersection Enumeration**: An extended enumeration introduces additional, more specific types of intersection for ranges. It includes `Bellow`, `BellowOverlap`, `Within`, `Same`, `Over`, `AboveOverlap`, `Above`.
- **Intersection Trait**: A trait that can be implemented on ranges to determine the kind of intersection with another range. 

## Usage

To use Range-Ext in your Rust program, add it to your `Cargo.toml` file:

```rust
use range_ext::{IntersectionExt, Intersect};

let range1 = 3..10;
let range2 = 5..8;

let intersection = range1.intersect(&range2);
```


TODO:
There is still missing RangeInclusive and RangeToInclusive at the moment.