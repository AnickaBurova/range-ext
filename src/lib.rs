//! Binary operations on ranges which are defined in std::ops::
//!
//! There are these types of operations:
//! Union A + B
//! Intersection A * B
//! Symmetric difference A / B
//! Subtraction A - B
//!
//! As these operations can yield disjoint ranges, the result has a special enum type, to represent the result.
//!
//! There are also relations between ranges:
//! A is less than B
//! A is greater than B
//! A is equal to B
//! A is within B
//! A is overlapping B
//! A is touching B
//! A is overlapping and less than B
//! A is overlapping and greater than B
//!
//! For this ordering as there is a possible overlap between ranges, the simple std::cmp::Ordering is not sufficient.
//! So new RangeOrdering is defined.
macro_rules! try_unwrap {
    ($a: expr, $or: expr) => {
        match $a {
            Some(a) => a,
            None => return $or,
        }
    }
}
#[deprecated]
pub mod intersect;

// pub mod split;

pub mod range_interval;
pub use range_interval::*;

pub mod binary_result;
pub use binary_result::*;

pub mod successor;


pub mod subtraction;


