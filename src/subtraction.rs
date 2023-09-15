//! Boolean operation subtraction between two ranges, where the possible result is nothing, one or two ranges.


use std::ops::{Bound, Sub};
use crate::binary_result::BinaryResult;
use crate::range_interval::RangeInterval;

impl<T: PartialOrd> RangeInterval<T> {
    pub fn subtract(self, other: Self) -> BinaryResult<T> {
        // self is a .. b
        // other is x .. y
        // this code could be simpler if I use partial ord for all possible combinations between ax, ay, bx and by, in one huge match,
        // but readability would suffer a lot
        match (other.start, other.end) {
            (Bound::Unbounded, Bound::Unbounded) => {
                // subtracting everything will yield None
                BinaryResult::None
            }
            (Bound::Unbounded, Bound::Included(y)) => {
                // subtracting everything up to a point will yield a single range excluded at the start
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y < b { // .. y < b
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Included(b)))
                        } else {
                            BinaryResult::None
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b { // .. y < b
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Excluded(b)))
                        } else {
                            BinaryResult::None
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if y < a { // .. y < a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        } else { // a <= y ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if y < a { // .. y < a ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        } else { // a <= y ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                }
            }
            (Bound::Unbounded, Bound::Excluded(y)) => {
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y < b { // .. y < b
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Included(b)))
                        } else {
                            BinaryResult::None
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b { // .. y < b
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Excluded(b)))
                        } else {
                            BinaryResult::None
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if y < a { // .. y < a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        } else { // a <= y ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b)})
                        } else if y <= b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if y < a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if y <= a { // .. y < a ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        } else { // a <= y ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if y <= a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b)})
                        } else if y <= b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if y <= a { // .. y < a .. b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b)})
                        } else if y < b { // a <= y < b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                        } else { //  b<= y , all is gone
                            BinaryResult::None
                        }
                    }
                }
            }
            (Bound::Included(x), Bound::Unbounded) => {
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if x <= a {
                            BinaryResult::None
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Excluded(x)))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if x <= a {
                            BinaryResult::None
                        } else if x <= b {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x)})
                        } else {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b)})
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if x <= a {
                            BinaryResult::None
                        } else if x <= b {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x)})
                        } else {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b)})
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if x <= a {
                            BinaryResult::None
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Excluded(x)))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if x <= a {
                            BinaryResult::None
                        } else if x <= b {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x)})
                        } else {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b)})
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if x <= a {
                            BinaryResult::None
                        } else if x <= b {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x)})
                        } else {
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b)})
                        }
                    }
                }
            }
            (Bound::Included(x), Bound::Included(y)) => {
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        // we are sure that the x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                            )
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y < b {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Included(b)),
                            )
                        } else if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Excluded(b)),
                            )
                        } else if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if a < x {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Included(a), Bound::Excluded(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                            )
                        } else if a <= y {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                            }
                        }  else if a > y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                        } else if a == y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                            }
                        }  else if a > y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                        } else if a == y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if a < x {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Excluded(a), Bound::Excluded(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                            )
                        } else if a <= y {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                            }
                        }  else if a >= y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                            }
                        }  else if a >= y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                }
            }
            (Bound::Included(x), Bound::Excluded(y)) => {
                if x == y {
                    return BinaryResult::One(self);
                }
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        // we are sure that the x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                            )
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y <= b {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Included(b)),
                            )
                        } else if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Excluded(b)),
                            )
                        } else if x <= b {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if a < x {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Included(a), Bound::Excluded(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                            )
                        } else if a <= y {
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if a < x {
                            if y <= b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                            }
                        }  else if a > y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                        } else if a == y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                        } else {
                            if y <= b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                            }
                        }  else if a > y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                        } else if a == y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if a < x {
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Excluded(a), Bound::Excluded(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                            )
                        } else if a < y {
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if a < x {
                            if y <= b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                            }
                        }  else if a >= y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                        } else {
                            if y <= b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if a < x {
                            if y < b {
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b) },
                                )
                            } else if x <= b {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(x) })
                            } else {
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                            }
                        }  else if a >= y {
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                        } else {
                            if y < b {
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                            } else {
                                BinaryResult::None
                            }
                        }
                    }
                }
            }
            (Bound::Excluded(x), Bound::Unbounded) => {
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if x <= b { // .. x <= b
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if x < b { // .. x <= b
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if a > x { // .. a < x ..
                            BinaryResult::None
                        } else { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Included(x)))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if a > x { // .. a < x .. b
                            BinaryResult::None
                        } else if x <= b { // x <= a <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x)})
                        } else { //  a < x <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b)})
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if a > x { // .. a < x .. b
                            BinaryResult::None
                        } else if x < b { // x <= a <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x)})
                        } else { //  a < x <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b)})
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if a >= x { // .. a < x ..
                            BinaryResult::None
                        } else { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Included(x)))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if a >= x { // .. a < x .. b
                            BinaryResult::None
                        } else if x <= b { // x <= a <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x)})
                        } else { //  a < x <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b)})
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if a >= x { // .. a < x .. b
                            BinaryResult::None
                        } else if x < b { // x <= a <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x)})
                        } else { //  a < x <= b
                            BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b)})
                        }
                    }
                }
            }
            (Bound::Excluded(x), Bound::Included(y)) => {
                if x == y {
                    return BinaryResult::One(self);
                }
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::Two(
                            RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                            RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                        )
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y < b { // .. x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Included(b)),
                            )
                        } else if x <= b { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b { // .. x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Excluded(b)),
                            )
                        } else if x < b { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if a <= x { // .. a < x ..
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Included(a), Bound::Included(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                            )
                        } else if a > y { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if a <= x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b) },
                                )
                            } else if x <= b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                            }
                        }  else if a > y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if a <= x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b) },
                                )
                            } else if x < b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                            }
                        }  else if a > y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if a < x { // .. a < x ..
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Excluded(a), Bound::Included(x)),
                                RangeInterval::new(Bound::Excluded(y), Bound::Unbounded),
                            )
                        } else if a <= y { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if a < x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b) },
                                )
                            } else if x <= b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                            }
                        }  else if a >= y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Included(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if a < x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b) },
                                )
                            } else if x < b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                            }
                        }  else if a >= y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Excluded(y), end: Bound::Excluded(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                }

            }
            (Bound::Excluded(x), Bound::Excluded(y)) => {
                if x == y {
                    return BinaryResult::One(self);
                }
                match (self.start, self.end) {
                    (Bound::Unbounded, Bound::Unbounded) => {
                        BinaryResult::Two(
                            RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                            RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                        )
                    }
                    (Bound::Unbounded, Bound::Included(b)) => {
                        if y <= b { // .. x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Included(b)), )
                        } else if x <= b { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(b)))
                        }
                    }
                    (Bound::Unbounded, Bound::Excluded(b)) => {
                        if y < b { // .. x <= b
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Unbounded, Bound::Included(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Excluded(b)),
                            )
                        } else if x < b { // b < x
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Included(x)))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Unbounded, Bound::Excluded(b)))
                        }
                    }
                    (Bound::Included(a), Bound::Unbounded) => {
                        if a <= x { // .. a < x ..
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Included(a), Bound::Included(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                            )
                        } else if a > y { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(a), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        }
                    }
                    (Bound::Included(a), Bound::Included(b)) => {
                        if a <= x { // .. a < x .. b
                            if y <= b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b) },
                                )
                            } else if x < b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                            }
                        }  else if a > y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(b) })
                        } else { // .. a < y .. b
                            if y <= b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Included(a), Bound::Excluded(b)) => {
                        if a <= x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b) },
                                )
                            } else if x < b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                            }
                        }  else if a > y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Included(a), end: Bound::Excluded(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Unbounded) => {
                        if a < x { // .. a < x ..
                            BinaryResult::Two(
                                RangeInterval::new(Bound::Excluded(a), Bound::Included(x)),
                                RangeInterval::new(Bound::Included(y), Bound::Unbounded),
                            )
                        } else if a < y { // x <= a ..
                            BinaryResult::One(RangeInterval::new(Bound::Included(y), Bound::Unbounded))
                        } else {
                            BinaryResult::One(RangeInterval::new(Bound::Excluded(a), Bound::Unbounded))
                        }
                    }
                    (Bound::Excluded(a), Bound::Included(b)) => {
                        if a < x { // .. a < x .. b
                            if y <= b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b) },
                                )
                            } else if x <= b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                            }
                        }  else if a >= y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(b) })
                        } else { // .. a < y .. b
                            if y <= b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Included(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                    (Bound::Excluded(a), Bound::Excluded(b)) => {
                        if a < x { // .. a < x .. b
                            if y < b { // .. a < x .. y < b
                                BinaryResult::Two(
                                    RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) },
                                    RangeInterval { reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b) },
                                )
                            } else if x < b { // .. a < x <= b
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Included(x) })
                            } else { // .. a < x .. b < x
                                BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                            }
                        }  else if a >= y { // .. y < a .. b
                            BinaryResult::One(RangeInterval { reverse: self.reverse, start: Bound::Excluded(a), end: Bound::Excluded(b) })
                        } else { // .. a < y .. b
                            if y < b { // .. a < y .. b
                                BinaryResult::One(RangeInterval{ reverse: self.reverse, start: Bound::Included(y), end: Bound::Excluded(b)})
                            } else { // .. a < y .. b < x
                                BinaryResult::None
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<T: PartialOrd> Sub<RangeInterval<T>> for RangeInterval<T> {
    type Output = BinaryResult<T>;

    fn sub(self, other: RangeInterval<T>) -> Self::Output {
        self.subtract(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::r;

    #[test]
    fn test_subtraction() {
        assert_eq!(r!(..) - r!(..), BinaryResult::<i32>::None);
        assert_eq!(r!(1..10) - r!(..), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(..), BinaryResult::None);

        assert_eq!(r!(..)    - r!(..=10), BinaryResult::One(r!(!10..))    );

        assert_eq!(r!(..=11) - r!(..=10), BinaryResult::One(r!(!10..=11)) );
        assert_eq!(r!(..=11) - r!(..=11), BinaryResult::None              );

        assert_eq!(r!(..11)  - r!(..=10), BinaryResult::One(r!(!10..11))  );
        assert_eq!(r!(..11)  - r!(..=11), BinaryResult::None              );

        assert_eq!(r!(1..)   - r!(..=1), BinaryResult::One(r!(!1..))    );
        assert_eq!(r!(1..)   - r!(..=10), BinaryResult::One(r!(!10..))    );
        assert_eq!(r!(1..)   - r!(..=0), BinaryResult::One(r!(1..))    );

        assert_eq!(r!(1..=1) - r!(..=1), BinaryResult::None              );
        assert_eq!(r!(1..=10) - r!(..=10), BinaryResult::None              );
        assert_eq!(r!(10..=1) - r!(..=10), BinaryResult::None              );
        assert_eq!(r!(10..=1) - r!(..=5), BinaryResult::One(r!(10..5)));
        assert_eq!(r!(1..=10) - r!(..=5), BinaryResult::One(r!(!5..=10)));
        assert_eq!(r!(1..=10) - r!(..=1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(1..=10) - r!(..=0), BinaryResult::One(r!(1..=10)));

        assert_eq!(r!(1..10) - r!(..=10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(..=12), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(..=5), BinaryResult::One(r!(!5..10)));
        assert_eq!(r!(1..10) - r!(..=1), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(1..10) - r!(..=0), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(..=5), BinaryResult::One(r!(!10..5)));
        assert_eq!(r!(!10..=1) - r!(..=1), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..=1) - r!(..=0), BinaryResult::One(r!(!10..=1)));

        assert_eq!(r!(!1..) - r!(..=10), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!1..) - r!(..=1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(..=0), BinaryResult::One(r!(!1..)));

        assert_eq!(r!(!1..=10) - r!(..=12), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(..=10), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(..=12), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(..=10), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(..=5), BinaryResult::One(r!(!5..=10)));
        assert_eq!(r!(!1..=10) - r!(..=1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(..=0), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(10..1) - r!(..=5), BinaryResult::One(r!(10..5)));
        assert_eq!(r!(10..1) - r!(..=1), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(..=0), BinaryResult::One(r!(10..1)));

        assert_eq!(r!(!1..10) - r!(..=12), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(..=10), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(..=12), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(..=10), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(..=5), BinaryResult::One(r!(!5..10)));
        assert_eq!(r!(!1..10) - r!(..=1), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(..=0), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!10..1) - r!(..=5), BinaryResult::One(r!(!10..5)));
        assert_eq!(r!(!10..1) - r!(..=1), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(..=0), BinaryResult::One(r!(!10..1)));

        assert_eq!(r!(..) - r!(..10), BinaryResult::One(r!(10..)));

        assert_eq!(r!(..=9) - r!(..10), BinaryResult::None);
        assert_eq!(r!(..=10) - r!(..10), BinaryResult::None);
        assert_eq!(r!(..=11) - r!(..10), BinaryResult::One(r!(10..=11)));

        assert_eq!(r!(..9) - r!(..10), BinaryResult::None);
        assert_eq!(r!(..10) - r!(..10), BinaryResult::None);
        assert_eq!(r!(..11) - r!(..10), BinaryResult::One(r!(10..11)));

        assert_eq!(r!(1..) - r!(..0), BinaryResult::One(r!(1..)));
        assert_eq!(r!(1..) - r!(..1), BinaryResult::One(r!(1..)));
        assert_eq!(r!(1..) - r!(..10), BinaryResult::One(r!(10..)));


        assert_eq!(r!(1..=10) - r!(..11), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(..11), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(..10), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(1..=10) - r!(..5), BinaryResult::One(r!(5..=10)));
        assert_eq!(r!(1..=10) - r!(..1), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(..0), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(10..=1) - r!(..5), BinaryResult::One(r!(10..=5)));
        assert_eq!(r!(10..=1) - r!(..1), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(..0), BinaryResult::One(r!(10..=1)));

        assert_eq!(r!(1..10) - r!(..11), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(..10), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(..11), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(..10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(..5), BinaryResult::One(r!(5..10)));
        assert_eq!(r!(1..10) - r!(..1), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..10) - r!(..0), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(..5), BinaryResult::One(r!(!10..=5)));
        assert_eq!(r!(!10..=1) - r!(..1), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(!10..=1) - r!(..0), BinaryResult::One(r!(!10..=1)));

        assert_eq!(r!(!1..) - r!(..0), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(..1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(..2), BinaryResult::One(r!(2..)));

        assert_eq!(r!(!1..=10) - r!(..11), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(..11), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(..10), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(!1..=10) - r!(..5), BinaryResult::One(r!(5..=10)));
        assert_eq!(r!(!1..=10) - r!(..1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(..0), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(10..1) - r!(..10), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(10..1) - r!(..5), BinaryResult::One(r!(10..=5)));
        assert_eq!(r!(10..1) - r!(..1), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(..0), BinaryResult::One(r!(10..1)));


        assert_eq!(r!(!1..10) - r!(..11), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(..10), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(..5), BinaryResult::One(r!(5..10)));
        assert_eq!(r!(!1..10) - r!(..1), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(..0), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!10..1) - r!(..5), BinaryResult::One(r!(!10..=5)));
        assert_eq!(r!(!10..1) - r!(..1), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(..0), BinaryResult::One(r!(!10..1)));


        assert_eq!(r!(..) - r!(1..), BinaryResult::One(r!(..1)));

        assert_eq!(r!(..=1) - r!(0..), BinaryResult::One(r!(..0)));
        assert_eq!(r!(..=1) - r!(1..), BinaryResult::One( r!(..1)));
        assert_eq!(r!(..=1) - r!(2..), BinaryResult::One(r!(..=1)));

        assert_eq!(r!(..1) - r!(0..), BinaryResult::One(r!(..0)));
        assert_eq!(r!(..1) - r!(1..), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..1) - r!(2..), BinaryResult::One(r!(..1)));


        assert_eq!(r!(1..) - r!(0..), BinaryResult::None);
        assert_eq!(r!(1..) - r!(1..), BinaryResult::None);
        assert_eq!(r!(1..) - r!(2..), BinaryResult::One(r!(1..2)));

        assert_eq!(r!(1..=10) - r!(0..), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(1..), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(5..), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..=10) - r!(10..), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..=10) - r!(11..), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(10..=1) - r!(5..), BinaryResult::One(r!(!5..=1)));
        assert_eq!(r!(10..=1) - r!(10..), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(10..=1) - r!(11..), BinaryResult::One(r!(10..=1)));

        assert_eq!(r!(1..10) - r!(0..), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(0..), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(1..), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(5..), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..10) - r!(5..), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..10) - r!(10..), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..10) - r!(11..), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(5..), BinaryResult::One(r!(!5..=1)));
        assert_eq!(r!(!10..=1) - r!(5..), BinaryResult::One(r!(!5..=1)));
        assert_eq!(r!(!10..=1) - r!(10..), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(!10..=1) - r!(11..), BinaryResult::One(r!(!10..=1)));

        assert_eq!(r!(!1..) - r!(0..), BinaryResult::None);
        assert_eq!(r!(!1..) - r!(1..), BinaryResult::None);
        assert_eq!(r!(!1..) - r!(2..), BinaryResult::One(r!(!1..2)));

        assert_eq!(r!(!1..=10) - r!(0..), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(1..), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(0..), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(1..), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(2..), BinaryResult::One(r!(!1..2)));
        assert_eq!(r!(!1..=10) - r!(10..), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..=10) - r!(11..), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(10..1) - r!(2..), BinaryResult::One(r!(!2..1)));
        assert_eq!(r!(10..1) - r!(10..), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(10..1) - r!(11..), BinaryResult::One(r!(10..1)));

        assert_eq!(r!(!1..10) - r!(0..), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(1..), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(0..), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(1..), BinaryResult::None);

        assert_eq!(r!(!1..10) - r!(2..), BinaryResult::One(r!(!1..2)));
        assert_eq!(r!(!1..10) - r!(10..), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(11..), BinaryResult::One(r!(!1..10)));

        assert_eq!(r!(!10..1) - r!(2..), BinaryResult::One(r!(!2..1)));
        assert_eq!(r!(!10..1) - r!(10..), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(11..), BinaryResult::One(r!(!10..1)));


        // splitting ranges
        assert_eq!(r!(..) - r!(1..=10), BinaryResult::Two(r!(..1), r!(!10..)));
        assert_eq!(r!(..) - r!(1..=1), BinaryResult::Two(r!(..1), r!(!1..)));
        assert_eq!(r!(..) - r!(10..=1), BinaryResult::Two(r!(..1), r!(!10..)));

        assert_eq!(r!(..=5) - r!(5..=10), BinaryResult::One(r!(..5)));
        assert_eq!(r!(..=5) - r!(1..=10), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..=5) - r!(1..=5), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..=5) - r!(1..=4), BinaryResult::Two(r!(..1), r!(!4..=5)));

        assert_eq!(r!(..5) - r!(5..=10), BinaryResult::One(r!(..5)));
        assert_eq!(r!(..5) - r!(1..=10), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..5) - r!(1..=5), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..5) - r!(1..=4), BinaryResult::Two(r!(..1), r!(!4..5)));

        assert_eq!(r!(1..) - r!(5..=10), BinaryResult::Two(r!(1..5), r!(!10..)));
        assert_eq!(r!(1..) - r!(1..=10), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(1..) - r!(0..=1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(1..) - r!(-1..=0), BinaryResult::One(r!(1..)));

        assert_eq!(r!(1..=10) - r!(-1..=0), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(-0..=0), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(-1..=1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(1..=10) - r!(-1..=2), BinaryResult::One(r!(!2..=10)));

        assert_eq!(r!(10..=1) - r!(-1..=0), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(-0..=0), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(-1..=1), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..=1) - r!(-1..=2), BinaryResult::One(r!(10..2)));

        assert_eq!(r!(1..=10) - r!(-1..=10), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(-1..=11), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(1..=11), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(1..=10), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(-1..=10), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(-1..=11), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(1..=11), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(1..=10), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(2..=2), BinaryResult::Two(r!(1..2), r!(!2..=10)));
        assert_eq!(r!(1..=10) - r!(2..=8), BinaryResult::Two(r!(1..2), r!(!8..=10)));
        assert_eq!(r!(1..=10) - r!(5..=10), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..=10) - r!(5..=12), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..=10) - r!(10..=10), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..=10) - r!(10..=12), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..=10) - r!(11..=12), BinaryResult::One(r!(1..=10)));

        assert_eq!(r!(10..=1) - r!(5..=10), BinaryResult::One(r!(!5..=1)));
        assert_eq!(r!(10..=1) - r!(5..=12), BinaryResult::One(r!(!5..=1)));
        assert_eq!(r!(10..=1) - r!(10..=10), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(10..=1) - r!(10..=12), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(10..=1) - r!(11..=12), BinaryResult::One(r!(10..=1)));

        assert_eq!(r!(1..10) - r!(-1..=0), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(-1..=0), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(1..10) - r!(-1..=1), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!10..=1) - r!(-1..=1), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(1..10) - r!(-1..=2), BinaryResult::One(r!(!2..10)));
        assert_eq!(r!(!10..=1) - r!(-1..=2), BinaryResult::One(r!(!10..2)));
        assert_eq!(r!(1..10) - r!(-1..=10), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(-1..=10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(-1..=11), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(-1..=11), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..=10), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(1..=10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..=11), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(1..=11), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..=5), BinaryResult::One(r!(!5..10)));
        assert_eq!(r!(!10..=1) - r!(1..=5), BinaryResult::One(r!(!10..5)));
        assert_eq!(r!(1..10) - r!(3..=5), BinaryResult::Two(r!(1..3), r!(!5..10)));
        assert_eq!(r!(!10..=1) - r!(3..=5), BinaryResult::Two(r!(!3..=1), r!(!10..5)));
        assert_eq!(r!(1..10) - r!(3..=5), BinaryResult::Two(r!(1..3), r!(!5..10)));
        assert_eq!(r!(!10..=1) - r!(3..=5), BinaryResult::Two(r!(!3..=1), r!(!10..5)));
        assert_eq!(r!(1..10) - r!(5..=5), BinaryResult::Two(r!(1..5), r!(!5..10)));
        assert_eq!(r!(!10..=1) - r!(5..=5), BinaryResult::Two(r!(!5..=1), r!(!10..5)));
        assert_eq!(r!(1..10) - r!(10..=12), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(10..=12), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(1..10) - r!(11..=12), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(11..=12), BinaryResult::One(r!(!10..=1)));

        assert_eq!(r!(!1..) - r!(-1..=0), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(-1..=1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(-1..=2), BinaryResult::One(r!(!2..)));
        assert_eq!(r!(!1..) - r!(1..=2), BinaryResult::One(r!(!2..)));
        assert_eq!(r!(!1..) - r!(1..=5), BinaryResult::One(r!(!5..)));
        assert_eq!(r!(!1..) - r!(2..=5), BinaryResult::Two(r!(!1..2), r!(!5..)));
        assert_eq!(r!(!1..) - r!(4..=5), BinaryResult::Two(r!(!1..4), r!(!5..)));


        assert_eq!(r!(!1..=10) - r!(-2..=0), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(-2..=1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(-2..=5), BinaryResult::One(r!(!5..=10)));
        assert_eq!(r!(!1..=10) - r!(-2..=10), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(-2..=13), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(1..=5), BinaryResult::One(r!(!5..=10)));
        assert_eq!(r!(!1..=10) - r!(1..=10), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(1..=15), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(3..=5), BinaryResult::Two(r!(!1..3), r!(!5..=10)));
        assert_eq!(r!(!1..=10) - r!(5..=5), BinaryResult::Two(r!(!1..5), r!(!5..=10)));
        assert_eq!(r!(!1..=10) - r!(5..=10), BinaryResult::One(r!(!1..5)));
        assert_eq!(r!(!1..=10) - r!(5..=15), BinaryResult::One(r!(!1..5)));
        assert_eq!(r!(!1..=10) - r!(10..=15), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..=10) - r!(13..=15), BinaryResult::One(r!(!1..=10)));

        assert_eq!(r!(10..1) - r!(0..=-2), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(1..=-2), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(5..=-2), BinaryResult::One(r!(10..5)));
        assert_eq!(r!(10..1) - r!(10..=-2), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(13..=-2), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(5..=1), BinaryResult::One(r!(10..5)));
        assert_eq!(r!(10..1) - r!(10..=1), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(15..=1), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(5..=3), BinaryResult::Two(r!(!3..1), r!(10..5)));
        assert_eq!(r!(10..1) - r!(5..=5), BinaryResult::Two(r!(!5..1), r!(10..5)));
        assert_eq!(r!(10..1) - r!(10..=5), BinaryResult::One(r!(!5..1)));
        assert_eq!(r!(10..1) - r!(15..=5), BinaryResult::One(r!(!5..1)));
        assert_eq!(r!(10..1) - r!(15..=10), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(10..1) - r!(15..=13), BinaryResult::One(r!(10..1)));

        // - Included .. Excluded
        assert_eq!(r!(..) - r!(1..1), BinaryResult::One(r!(..)));
        assert_eq!(r!(1..) - r!(1..1), BinaryResult::One(r!(1..)));
        assert_eq!(r!(!1..) - r!(1..1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(..1) - r!(1..1), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..=1) - r!(1..1), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(1..5) - r!(1..1), BinaryResult::One(r!(1..5)));
        assert_eq!(r!(1..=5) - r!(1..1), BinaryResult::One(r!(1..=5)));
        assert_eq!(r!(!1..5) - r!(1..1), BinaryResult::One(r!(!1..5)));
        assert_eq!(r!(!1..=5) - r!(1..1), BinaryResult::One(r!(!1..=5)));

        assert_eq!(r!(..) - r!(1..10), BinaryResult::Two(r!(..1), r!(10..)));

        assert_eq!(r!(..=10) - r!(1..5), BinaryResult::Two(r!(..1), r!(5..=10)));
        assert_eq!(r!(..=10) - r!(1..10), BinaryResult::Two(r!(..1), r!(10..=10)));
        assert_eq!(r!(..=10) - r!(1..11), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..=10) - r!(10..11), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..=10) - r!(11..12), BinaryResult::One(r!(..=10)));

        assert_eq!(r!(..10) - r!(1..5), BinaryResult::Two(r!(..1), r!(5..10)));
        assert_eq!(r!(..10) - r!(1..10), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..10) - r!(1..12), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..10) - r!(10..12), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..10) - r!(11..12), BinaryResult::One(r!(..10)));


        assert_eq!(r!(1..) - r!(-2..0), BinaryResult::One(r!(1..)));
        assert_eq!(r!(1..) - r!(-2..1), BinaryResult::One(r!(1..)));
        assert_eq!(r!(1..) - r!(-2..5), BinaryResult::One(r!(5..)));
        assert_eq!(r!(1..) - r!(1..5), BinaryResult::One(r!(5..)));
        assert_eq!(r!(1..) - r!(1..5), BinaryResult::One(r!(5..)));
        assert_eq!(r!(1..) - r!(3..5), BinaryResult::Two(r!(1..3), r!(5..)));


        assert_eq!(r!(1..=10) - r!(-2..0), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(-2..1), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(-2..5), BinaryResult::One(r!(5..=10)));
        assert_eq!(r!(10..=1) - r!(-2..0), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(-2..1), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(-2..5), BinaryResult::One(r!(10..=5)));

        assert_eq!(r!(1..=10) - r!(-2..10), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(1..=10) - r!(-2..11), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(-2..10), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(10..=1) - r!(-2..11), BinaryResult::None);


        assert_eq!(r!(1..=10) - r!(1..11), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(1..11), BinaryResult::None);

        assert_eq!(r!(1..=10) - r!(4..8), BinaryResult::Two(r!(1..4), r!(8..=10)));
        assert_eq!(r!(10..=1) - r!(4..8), BinaryResult::Two(r!(!4..=1), r!(10..=8)));
        assert_eq!(r!(1..=10) - r!(4..10), BinaryResult::Two(r!(1..4), r!(10..=10)));
        assert_eq!(r!(1..=10) - r!(4..15), BinaryResult::One(r!(1..4)));
        assert_eq!(r!(1..=10) - r!(10..15), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..=10) - r!(12..15), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(10..=1) - r!(4..15), BinaryResult::One(r!(!4..=1)));
        assert_eq!(r!(10..=1) - r!(10..15), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(10..=1) - r!(12..15), BinaryResult::One(r!(10..=1)));


        assert_eq!(r!(1..10) - r!(-2..0), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(-2..0), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(1..10) - r!(-2..1), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(-2..1), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(1..10) - r!(-2..5), BinaryResult::One(r!(5..10)));
        assert_eq!(r!(!10..=1) - r!(-2..5), BinaryResult::One(r!(!10..=5)));
        assert_eq!(r!(1..10) - r!(-2..10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(-2..15), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..10), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(1..10), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(1..15), BinaryResult::None);

        assert_eq!(r!(1..10) - r!(1..6), BinaryResult::One(r!(6..10)));
        assert_eq!(r!(!10..=1) - r!(1..6), BinaryResult::One(r!(!10..=6)));
        assert_eq!(r!(1..10) - r!(4..6), BinaryResult::Two(r!(1..4), r!(6..10)));
        assert_eq!(r!(!10..=1) - r!(4..6), BinaryResult::Two(r!(!4..=1), r!(!10..=6)));
        assert_eq!(r!(1..10) - r!(4..10), BinaryResult::One(r!(1..4)));
        assert_eq!(r!(!10..=1) - r!(4..10), BinaryResult::One(r!(!4..=1)));
        assert_eq!(r!(1..10) - r!(4..15), BinaryResult::One(r!(1..4)));
        assert_eq!(r!(!10..=1) - r!(4..15), BinaryResult::One(r!(!4..=1)));
        assert_eq!(r!(1..10) - r!(10..15), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..10) - r!(12..15), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(10..15), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(!10..=1) - r!(12..15), BinaryResult::One(r!(!10..=1)));


        assert_eq!(r!(!1..) - r!(-5..0), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(-5..1), BinaryResult::One(r!(!1..)));
        assert_eq!(r!(!1..) - r!(-5..5), BinaryResult::One(r!(5..)));
        assert_eq!(r!(!1..) - r!(1..5), BinaryResult::One(r!(5..)));
        assert_eq!(r!(!1..) - r!(3..5), BinaryResult::Two(r!(!1..3), r!(5..)));

        assert_eq!(r!(!1..=10) - r!(-5..0), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(-5..1), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(-5..5), BinaryResult::One(r!(5..=10)));
        assert_eq!(r!(!1..=10) - r!(-5..10), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(!1..=10) - r!(-5..15), BinaryResult::None);

        assert_eq!(r!(10..1) - r!(-5..0), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(-5..1), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(-5..5), BinaryResult::One(r!(10..=5)));
        assert_eq!(r!(10..1) - r!(-5..10), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(10..1) - r!(-5..15), BinaryResult::None);

        assert_eq!(r!(!1..=10) - r!(1..5), BinaryResult::One(r!(5..=10)));
        assert_eq!(r!(!1..=10) - r!(1..10), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(!1..=10) - r!(1..15), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(3..6), BinaryResult::Two(r!(!1..3), r!(6..=10)));
        assert_eq!(r!(!1..=10) - r!(3..10), BinaryResult::Two(r!(!1..3), r!(10..=10)));
        assert_eq!(r!(!1..=10) - r!(3..15), BinaryResult::One(r!(!1..3)));

        assert_eq!(r!(10..1) - r!(1..5), BinaryResult::One(r!(10..=5)));
        assert_eq!(r!(10..1) - r!(1..10), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(10..1) - r!(1..15), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(3..6), BinaryResult::Two(r!(!3..1), r!(10..=6)));
        assert_eq!(r!(10..1) - r!(3..10), BinaryResult::Two(r!(!3..1), RangeInterval::rev_point(10)));
        assert_eq!(r!(10..1) - r!(3..15), BinaryResult::One(r!(!3..1)));

        assert_eq!(r!(!1..=10) - r!(10..15), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..=10) - r!(12..15), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(10..1) - r!(10..15), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(10..1) - r!(12..15), BinaryResult::One(r!(10..1)));


        assert_eq!(r!(!1..10) - r!(-5..0), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(-5..1), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(-5..5), BinaryResult::One(r!(5..10)));
        assert_eq!(r!(!1..10) - r!(-5..10), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(-5..15), BinaryResult::None);

        assert_eq!(r!(!1..10) - r!(1..5), BinaryResult::One(r!(5..10)));
        assert_eq!(r!(!1..10) - r!(1..10), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(1..15), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(3..5), BinaryResult::Two(r!(!1..3), r!(5..10)));
        assert_eq!(r!(!1..10) - r!(3..10), BinaryResult::One(r!(!1..3)));
        assert_eq!(r!(!1..10) - r!(3..15), BinaryResult::One(r!(!1..3)));
        assert_eq!(r!(!1..10) - r!(10..15), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(12..15), BinaryResult::One(r!(!1..10)));


        assert_eq!(r!(!10..1) - r!(-5..0), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(-5..1), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(-5..5), BinaryResult::One(r!(!10..=5)));
        assert_eq!(r!(!10..1) - r!(-5..10), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(-5..15), BinaryResult::None);

        assert_eq!(r!(!10..1) - r!(1..5), BinaryResult::One(r!(!10..=5)));
        assert_eq!(r!(!10..1) - r!(1..10), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(1..15), BinaryResult::None);
        assert_eq!(r!(!10..1) - r!(3..5), BinaryResult::Two(r!(!3..1), r!(!10..=5)));
        assert_eq!(r!(!10..1) - r!(3..10), BinaryResult::One(r!(!3..1)));
        assert_eq!(r!(!10..1) - r!(3..15), BinaryResult::One(r!(!3..1)));
        assert_eq!(r!(!10..1) - r!(10..15), BinaryResult::One(r!(!10..1)));
        assert_eq!(r!(!10..1) - r!(12..15), BinaryResult::One(r!(!10..1)));


        assert_eq!(r!(..) - r!(!1..), BinaryResult::One(r!(..=1)));

        assert_eq!(r!(..=1) - r!(!-3..), BinaryResult::One(r!(..=-3)));
        assert_eq!(r!(..=1) - r!(!1..), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..=1) - r!(!5..), BinaryResult::One(r!(..=1)));

        assert_eq!(r!(..1) - r!(!-3..), BinaryResult::One(r!(..=-3)));
        assert_eq!(r!(..1) - r!(!1..), BinaryResult::One(r!(..1)));
        assert_eq!(r!(..1) - r!(!3..), BinaryResult::One(r!(..1)));


        assert_eq!(r!(1..) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(1..) - r!(!1..), BinaryResult::One(r!(1..=1)));
        assert_eq!(r!(1..) - r!(!5..), BinaryResult::One(r!(1..=5)));


        assert_eq!(r!(1..=10) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(1..=10) - r!(!1..), BinaryResult::One(r!(1..=1)));
        assert_eq!(r!(1..=10) - r!(!10..), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(1..=10) - r!(!15..), BinaryResult::One(r!(1..=10)));
        assert_eq!(r!(10..=1) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(10..=1) - r!(!1..), BinaryResult::One(RangeInterval::rev_point(1)));
        assert_eq!(r!(10..=1) - r!(!10..), BinaryResult::One(r!(10..=1)));
        assert_eq!(r!(10..=1) - r!(!15..), BinaryResult::One(r!(10..=1)));


        assert_eq!(r!(1..10) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(1..10) - r!(!1..), BinaryResult::One(r!(1..=1)));
        assert_eq!(r!(1..10) - r!(!5..), BinaryResult::One(r!(1..=5)));
        assert_eq!(r!(1..10) - r!(!10..), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(1..10) - r!(!15..), BinaryResult::One(r!(1..10)));
        assert_eq!(r!(!10..=1) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(!10..=1) - r!(!1..), BinaryResult::One(RangeInterval::rev_point(1)));
        assert_eq!(r!(!10..=1) - r!(!5..), BinaryResult::One(r!(5..=1)));
        assert_eq!(r!(!10..=1) - r!(!10..), BinaryResult::One(r!(!10..=1)));
        assert_eq!(r!(!10..=1) - r!(!15..), BinaryResult::One(r!(!10..=1)));

        assert_eq!(r!(!1..) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(!1..) - r!(!1..), BinaryResult::None);
        assert_eq!(r!(!1..) - r!(!5..), BinaryResult::One(r!(!1..=5)));

        assert_eq!(r!(!1..=10) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(!1..), BinaryResult::None);
        assert_eq!(r!(!1..=10) - r!(!5..), BinaryResult::One(r!(!1..=5)));
        assert_eq!(r!(!1..=10) - r!(!10..), BinaryResult::One(r!(!1..=10)));
        assert_eq!(r!(!1..=10) - r!(!15..), BinaryResult::One(r!(!1..=10)));

        assert_eq!(r!(10..1) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(!1..), BinaryResult::None);
        assert_eq!(r!(10..1) - r!(!5..), BinaryResult::One(r!(5..1)));
        assert_eq!(r!(10..1) - r!(!10..), BinaryResult::One(r!(10..1)));
        assert_eq!(r!(10..1) - r!(!15..), BinaryResult::One(r!(10..1)));

        assert_eq!(r!(!1..10) - r!(!-3..), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(!1..), BinaryResult::None);
        assert_eq!(r!(!1..10) - r!(!5..), BinaryResult::One(r!(!1..=5)));
        assert_eq!(r!(!1..10) - r!(!10..), BinaryResult::One(r!(!1..10)));
        assert_eq!(r!(!1..10) - r!(!15..), BinaryResult::One(r!(!1..10)));


        assert_eq!(r!(..) - r!(!1..=10), BinaryResult::Two(r!(..=1), r!(!10..)));


        assert_eq!(r!(..=10) - r!(!1..=5), BinaryResult::Two(r!(..=1), r!(!5..=10)));
        assert_eq!(r!(..=10) - r!(!1..=10), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..=10) - r!(!1..=15), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..=10) - r!(!10..=10), BinaryResult::One(r!(..=10)));
        assert_eq!(r!(..=10) - r!(!10..=15), BinaryResult::One(r!(..=10)));
        assert_eq!(r!(..=10) - r!(!12..=15), BinaryResult::One(r!(..=10)));


        assert_eq!(r!(..10) - r!(!1..=5), BinaryResult::Two(r!(..=1), r!(!5..10)));
        assert_eq!(r!(..10) - r!(!1..=10), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..10) - r!(!1..=15), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..10) - r!(!10..=10), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..10) - r!(!10..=15), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..10) - r!(!12..=15), BinaryResult::One(r!(..10)));


        assert_eq!(r!(10..) - r!(!1..=5), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!1..=10), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!1..=15), BinaryResult::One(r!(15..)));
        assert_eq!(r!(10..) - r!(!10..=10), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!10..=15), BinaryResult::Two(r!(10..=10), r!(!15..)));
        assert_eq!(r!(10..) - r!(!12..=15), BinaryResult::Two(r!(10..=12), r!(!15..)));


        assert_eq!(r!(10..=20) - r!(!1..=5), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!1..=10), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(10..=20) - r!(!1..=15), BinaryResult::One(r!(!15..=20)));
        assert_eq!(r!(10..=20) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(10..=20) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(10..=20) - r!(!10..=15), BinaryResult::Two(r!(10..=10), r!(!15..=20)));
        assert_eq!(r!(10..=20) - r!(!10..=20), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..=20) - r!(!10..=25), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..=20) - r!(!13..=15), BinaryResult::Two(r!(10..=13), r!(!15..=20)));
        assert_eq!(r!(10..=20) - r!(!13..=20), BinaryResult::One(r!(10..=13)));
        assert_eq!(r!(10..=20) - r!(!13..=25), BinaryResult::One(r!(10..=13)));
        assert_eq!(r!(10..=20) - r!(!20..=25), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!22..=25), BinaryResult::One(r!(10..=20)));

        assert_eq!(r!(20..=10) - r!(!1..=5), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!1..=10), BinaryResult::One(r!(20..10)));
        assert_eq!(r!(20..=10) - r!(!1..=15), BinaryResult::One(r!(20..15)));
        assert_eq!(r!(20..=10) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(20..=10) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(20..=10) - r!(!10..=15), BinaryResult::Two(RangeInterval::rev_point(10), r!(20..15)));
        assert_eq!(r!(20..=10) - r!(!13..=15), BinaryResult::Two(r!(13..=10), r!(20..15)));
        assert_eq!(r!(20..=10) - r!(!13..=20), BinaryResult::One(r!(13..=10)));
        assert_eq!(r!(20..=10) - r!(!13..=25), BinaryResult::One(r!(13..=10)));
        assert_eq!(r!(20..=10) - r!(!20..=25), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!22..=25), BinaryResult::One(r!(20..=10)));


        assert_eq!(r!(10..20) - r!(!1..=5), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!1..=10), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(10..20) - r!(!1..=15), BinaryResult::One(r!(!15..20)));
        assert_eq!(r!(10..20) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(10..20) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(10..20) - r!(!10..=15), BinaryResult::Two(r!(10..=10), r!(!15..20)));
        assert_eq!(r!(10..20) - r!(!10..=20), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..20) - r!(!10..=25), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..20) - r!(!13..=15), BinaryResult::Two(r!(10..=13), r!(!15..20)));
        assert_eq!(r!(10..20) - r!(!13..=20), BinaryResult::One(r!(10..=13)));
        assert_eq!(r!(10..20) - r!(!13..=25), BinaryResult::One(r!(10..=13)));
        assert_eq!(r!(10..20) - r!(!20..=25), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!22..=25), BinaryResult::One(r!(10..20)));

        assert_eq!(r!(!20..=10) - r!(!1..=5), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!1..=10), BinaryResult::One(r!(!20..10)));
        assert_eq!(r!(!20..=10) - r!(!1..=15), BinaryResult::One(r!(!20..15)));
        assert_eq!(r!(!20..=10) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(!20..=10) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(!20..=10) - r!(!10..=15), BinaryResult::Two(RangeInterval::rev_point(10), r!(!20..15)));
        assert_eq!(r!(!20..=10) - r!(!10..=20), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(!20..=10) - r!(!10..=25), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(!20..=10) - r!(!13..=15), BinaryResult::Two(r!(13..=10), r!(!20..15)));
        assert_eq!(r!(!20..=10) - r!(!13..=20), BinaryResult::One(r!(13..=10)));
        assert_eq!(r!(!20..=10) - r!(!13..=25), BinaryResult::One(r!(13..=10)));
        assert_eq!(r!(!20..=10) - r!(!20..=25), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!22..=25), BinaryResult::One(r!(!20..=10)));


        assert_eq!(r!(!10..) - r!(!1..=5), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!10..) - r!(!1..=10), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!10..) - r!(!1..=15), BinaryResult::One(r!(!15..)));
        assert_eq!(r!(!10..) - r!(!10..=15), BinaryResult::One(r!(!15..)));
        assert_eq!(r!(!10..) - r!(!12..=15), BinaryResult::Two(r!(!10..=12), r!(!15..)));
        assert_eq!(r!(!10..) - r!(!15..=15), BinaryResult::One(r!(!10..)));


        assert_eq!(r!(!10..=20) - r!(!1..=5), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..=10), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..=15), BinaryResult::One(r!(!15..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!10..=15), BinaryResult::One(r!(!15..=20)));
        assert_eq!(r!(!10..=20) - r!(!10..=20), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!10..=25), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!13..=15), BinaryResult::Two(r!(!10..=13), r!(!15..=20)));
        assert_eq!(r!(!10..=20) - r!(!13..=20), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..=20) - r!(!13..=25), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..=20) - r!(!20..=25), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!22..=25), BinaryResult::One(r!(!10..=20)));

        assert_eq!(r!(20..10) - r!(!1..=5), BinaryResult::One(r!(20..10)));
        assert_eq!(r!(20..10) - r!(!1..=10), BinaryResult::One(r!(20..10)));
        assert_eq!(r!(20..10) - r!(!1..=15), BinaryResult::One(r!(20..15)));
        assert_eq!(r!(20..10) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(20..10) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(20..10) - r!(!10..=15), BinaryResult::One(r!(20..15)));
        assert_eq!(r!(20..10) - r!(!10..=20), BinaryResult::None);
        assert_eq!(r!(20..10) - r!(!10..=25), BinaryResult::None);
        assert_eq!(r!(20..10) - r!(!13..=15), BinaryResult::Two(r!(13..10), r!(20..15)));
        assert_eq!(r!(20..10) - r!(!13..=20), BinaryResult::One(r!(13..10)));
        assert_eq!(r!(20..10) - r!(!13..=25), BinaryResult::One(r!(13..10)));
        assert_eq!(r!(20..10) - r!(!20..=25), BinaryResult::One(r!(20..10)));
        assert_eq!(r!(20..10) - r!(!22..=25), BinaryResult::One(r!(20..10)));


        assert_eq!(r!(!10..20) - r!(!1..=5), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!1..=10), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!1..=15), BinaryResult::One(r!(!15..20)));
        assert_eq!(r!(!10..20) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!10..=15), BinaryResult::One(r!(!15..20)));
        assert_eq!(r!(!10..20) - r!(!10..=20), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!10..=25), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!13..=15), BinaryResult::Two(r!(!10..=13), r!(!15..20)));
        assert_eq!(r!(!10..20) - r!(!13..=20), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..20) - r!(!13..=25), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..20) - r!(!20..=25), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!22..=25), BinaryResult::One(r!(!10..20)));

        assert_eq!(r!(!20..10) - r!(!1..=5), BinaryResult::One(r!(!20..10)));
        assert_eq!(r!(!20..10) - r!(!1..=10), BinaryResult::One(r!(!20..10)));
        assert_eq!(r!(!20..10) - r!(!1..=15), BinaryResult::One(r!(!20..15)));
        assert_eq!(r!(!20..10) - r!(!1..=20), BinaryResult::None);
        assert_eq!(r!(!20..10) - r!(!1..=25), BinaryResult::None);
        assert_eq!(r!(!20..10) - r!(!10..=15), BinaryResult::One(r!(!20..15)));
        assert_eq!(r!(!20..10) - r!(!10..=20), BinaryResult::None);
        assert_eq!(r!(!20..10) - r!(!10..=25), BinaryResult::None);
        assert_eq!(r!(!20..10) - r!(!13..=15), BinaryResult::Two(r!(13..10), r!(!20..15)));
        assert_eq!(r!(!20..10) - r!(!13..=20), BinaryResult::One(r!(13..10)));
        assert_eq!(r!(!20..10) - r!(!13..=25), BinaryResult::One(r!(13..10)));
        assert_eq!(r!(!20..10) - r!(!20..=25), BinaryResult::One(r!(!20..10)));
        assert_eq!(r!(!20..10) - r!(!22..=25), BinaryResult::One(r!(!20..10)));


        assert_eq!(r!(..) - r!(!13..13), BinaryResult::One(r!(..)));
        assert_eq!(r!(..) - r!(!13..15), BinaryResult::Two(r!(..=13), r!(15..)));

        assert_eq!(r!(..=10) - r!(!13..13), BinaryResult::One(r!(..=10)));
        assert_eq!(r!(..=10) - r!(!1..5), BinaryResult::Two(r!(..=1), r!(5..=10)));
        assert_eq!(r!(..=10) - r!(!1..10), BinaryResult::Two(r!(..=1), r!(10..=10)));
        assert_eq!(r!(..=10) - r!(!1..15), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..=10) - r!(!10..15), BinaryResult::One(r!(..=10)));
        assert_eq!(r!(..=10) - r!(!12..15), BinaryResult::One(r!(..=10)));


        assert_eq!(r!(..10) - r!(!13..13), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..10) - r!(!1..5), BinaryResult::Two(r!(..=1), r!(5..10)));
        assert_eq!(r!(..10) - r!(!1..10), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..10) - r!(!1..15), BinaryResult::One(r!(..=1)));
        assert_eq!(r!(..10) - r!(!10..15), BinaryResult::One(r!(..10)));
        assert_eq!(r!(..10) - r!(!12..15), BinaryResult::One(r!(..10)));

        assert_eq!(r!(10..) - r!(!1..5), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!1..10), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!1..20), BinaryResult::One(r!(20..)));
        assert_eq!(r!(10..) - r!(!10..10), BinaryResult::One(r!(10..)));
        assert_eq!(r!(10..) - r!(!10..20), BinaryResult::Two(r!(10..=10), r!(20..)));
        assert_eq!(r!(10..) - r!(!15..20), BinaryResult::Two(r!(10..=15), r!(20..)));

        assert_eq!(r!(10..=20) - r!(!1..5), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!1..10), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!1..15), BinaryResult::One(r!(15..=20)));
        assert_eq!(r!(10..=20) - r!(!1..20), BinaryResult::One(r!(20..=20)));
        assert_eq!(r!(10..=20) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(10..=20) - r!(!10..10), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!10..15), BinaryResult::Two(r!(10..=10), r!(15..=20)));
        assert_eq!(r!(10..=20) - r!(!10..20), BinaryResult::Two(r!(10..=10), r!(20..=20)));
        assert_eq!(r!(10..=20) - r!(!10..25), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..=20) - r!(!15..20), BinaryResult::Two(r!(10..=15), r!(20..=20)));
        assert_eq!(r!(10..=20) - r!(!15..18), BinaryResult::Two(r!(10..=15), r!(18..=20)));
        assert_eq!(r!(10..=20) - r!(!15..25), BinaryResult::One(r!(10..=15)));
        assert_eq!(r!(10..=20) - r!(!20..25), BinaryResult::One(r!(10..=20)));
        assert_eq!(r!(10..=20) - r!(!22..25), BinaryResult::One(r!(10..=20)));

        assert_eq!(r!(20..=10) - r!(!1..5), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!1..10), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!1..15), BinaryResult::One(r!(20..=15)));
        assert_eq!(r!(20..=10) - r!(!1..20), BinaryResult::One(RangeInterval::rev_point(20)));
        assert_eq!(r!(20..=10) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(20..=10) - r!(!10..10), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!10..15), BinaryResult::Two(RangeInterval::rev_point(10), r!(20..=15)));
        assert_eq!(r!(20..=10) - r!(!10..20), BinaryResult::Two(RangeInterval::rev_point(10), RangeInterval::rev_point(20)));
        assert_eq!(r!(20..=10) - r!(!10..25), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(20..=10) - r!(!15..20), BinaryResult::Two(r!(15..=10), RangeInterval::rev_point(20)));
        assert_eq!(r!(20..=10) - r!(!15..25), BinaryResult::One(r!(15..=10)));
        assert_eq!(r!(20..=10) - r!(!20..25), BinaryResult::One(r!(20..=10)));
        assert_eq!(r!(20..=10) - r!(!22..25), BinaryResult::One(r!(20..=10)));


        assert_eq!(r!(10..20) - r!(!1..5), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!1..10), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!1..15), BinaryResult::One(r!(15..20)));
        assert_eq!(r!(10..20) - r!(!1..20), BinaryResult::None);
        assert_eq!(r!(10..20) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(10..20) - r!(!10..10), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!10..15), BinaryResult::Two(r!(10..=10), r!(15..20)));
        assert_eq!(r!(10..20) - r!(!10..20), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..20) - r!(!10..25), BinaryResult::One(r!(10..=10)));
        assert_eq!(r!(10..20) - r!(!15..18), BinaryResult::Two(r!(10..=15), r!(18..20)));
        assert_eq!(r!(10..20) - r!(!15..20), BinaryResult::One(r!(10..=15)));
        assert_eq!(r!(10..20) - r!(!15..25), BinaryResult::One(r!(10..=15)));
        assert_eq!(r!(10..20) - r!(!20..25), BinaryResult::One(r!(10..20)));
        assert_eq!(r!(10..20) - r!(!22..25), BinaryResult::One(r!(10..20)));

        assert_eq!(r!(!20..=10) - r!(!1..5), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!1..10), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!1..15), BinaryResult::One(r!(!20..=15)));
        assert_eq!(r!(!20..=10) - r!(!1..20), BinaryResult::None);
        assert_eq!(r!(!20..=10) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(!20..=10) - r!(!10..10), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!10..15), BinaryResult::Two(RangeInterval::rev_point(10), r!(!20..=15)));
        assert_eq!(r!(!20..=10) - r!(!10..20), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(!20..=10) - r!(!10..25), BinaryResult::One(RangeInterval::rev_point(10)));
        assert_eq!(r!(!20..=10) - r!(!15..20), BinaryResult::One(r!(15..=10)));
        assert_eq!(r!(!20..=10) - r!(!15..25), BinaryResult::One(r!(15..=10)));
        assert_eq!(r!(!20..=10) - r!(!20..25), BinaryResult::One(r!(!20..=10)));
        assert_eq!(r!(!20..=10) - r!(!22..25), BinaryResult::One(r!(!20..=10)));


        assert_eq!(r!(!10..) - r!(!1..5), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!10..) - r!(!1..10), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!10..) - r!(!1..20), BinaryResult::One(r!(20..)));
        assert_eq!(r!(!10..) - r!(!10..10), BinaryResult::One(r!(!10..)));
        assert_eq!(r!(!10..) - r!(!10..20), BinaryResult::One(r!(20..)));
        assert_eq!(r!(!10..) - r!(!15..20), BinaryResult::Two(r!(!10..=15), r!(20..)));

        assert_eq!(r!(!10..=20) - r!(!1..5), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..10), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..15), BinaryResult::One(r!(15..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..20), BinaryResult::One(r!(20..=20)));
        assert_eq!(r!(!10..=20) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!10..10), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!10..15), BinaryResult::One(r!(15..=20)));
        assert_eq!(r!(!10..=20) - r!(!10..20), BinaryResult::One(r!(20..=20)));
        assert_eq!(r!(!10..=20) - r!(!10..25), BinaryResult::None);
        assert_eq!(r!(!10..=20) - r!(!13..15), BinaryResult::Two(r!(!10..=13), r!(15..=20)));
        assert_eq!(r!(!10..=20) - r!(!13..20), BinaryResult::Two(r!(!10..=13), r!(20..=20)));
        assert_eq!(r!(!10..=20) - r!(!13..25), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..=20) - r!(!20..25), BinaryResult::One(r!(!10..=20)));
        assert_eq!(r!(!10..=20) - r!(!23..25), BinaryResult::One(r!(!10..=20)));


        assert_eq!(r!(!10..20) - r!(!1..5), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!1..10), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!1..15), BinaryResult::One(r!(15..20)));
        assert_eq!(r!(!10..20) - r!(!1..20), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!1..25), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!10..10), BinaryResult::One(r!(!10..20)));
        assert_eq!(r!(!10..20) - r!(!10..15), BinaryResult::One(r!(15..20)));
        assert_eq!(r!(!10..20) - r!(!10..20), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!10..25), BinaryResult::None);
        assert_eq!(r!(!10..20) - r!(!13..15), BinaryResult::Two(r!(!10..=13), r!(15..20)));
        assert_eq!(r!(!10..20) - r!(!13..20), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..20) - r!(!13..25), BinaryResult::One(r!(!10..=13)));
        assert_eq!(r!(!10..20) - r!(!20..25), BinaryResult::One(r!(!10..20)));
    }
}