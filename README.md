# Range-Ext

This version introduces extensive changes from the previous version 0.4.0. The functionality 'Intersection' is currently deprecated and is undergoing a refactor into a redefined feature.

A new structure, `RangeInterval`, has been introduced. It can handle all the std::ops ranges and additional range cases which were not previously possible in Rust, like the exclusion at the start. This structure allows for direct conversion from std::ops ranges and can convert back to std::ops ranges using `to_range_xxx` if the conversion is possible.

The `RangeType` can be used to ascertain the type of std range. `RangeInterval` supports reversed ranges and will support an Iterator for all possible ranges that can be iterated in the future.

Please note, using `to_range` when the conversion is not possible will result in the loss of range as the range is moved. To prevent this, use `try_to_range_xxx`, which will return itself in 'Err' if the conversion is not possible.

At present, a new binary operation 'subtraction' between ranges is available. This operation can yield an empty range, a single range, or two ranges. To manage this, a new enum 'BinaryResult' has been introduced which has `None`, `One`, or `Two` ranges variants. Instead of using Rust's (0..0) empty range, the result uses the variant 'None'.

Subtraction is implemented for all possible ranges and it operates on reversed ranges as well, yielding reversed range(s) as the result.

Future updates will introduce `Union`, `Intersection`, and `SymmetricDifference` which will be implemented for all possible ranges.

The trait `Sub` (-) for Subtraction on the ranges has been implemented.

A new macro `r` has been introduced which can be used to create `RangeInterval` from literals. It supports all possible range cases and can be used to create reversed ranges. An Exclamation mark '!' is used to indicate the excluded start.
```rust
use range_ext::{r, RangeInterval, RangeType, BinaryResult};

let a = r!(!3..); // (3..)
let b = r!(5..=8); // <5..8>
let c = a - b; // BinaryResult::Two((3..5), (8..))

let d = r!(..);
let e = a - d; // BinaryResult::None
```