//! Boolean operations on ranges have multiple options of result. From nothing up to 2 ranges.


use crate::range_interval::RangeInterval;

#[derive(Debug, PartialEq)]
/// Binary operation on Ranges can yield nothing, one or two ranges.
pub enum BinaryResult<T> {
    /// The result is an empty set (0..0)
    None,
    /// The result is a single range
    One(RangeInterval<T>),
    /// The result is two ranges
    Two(RangeInterval<T>, RangeInterval<T>),
}