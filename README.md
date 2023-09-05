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
Issue: I think there is a problem with intersecting Range, if either of the ranges have no ascending start to end. To make it work I have assumed I can reverse if they are descending, but that is causing the other to becaome RangeToInclusive, and that is not yet implemented. So dont have any descending ranges for now.

TODO:
There is still missing RangeToInclusive at the moment.