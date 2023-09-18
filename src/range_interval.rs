//! Unified structure for representing ranges and intervals.

use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[macro_export]
macro_rules! r {
        (..) => {
            RangeInterval::new(Bound::Unbounded, Bound::Unbounded)
        };
        (.. $b: literal) => {
            RangeInterval::new(Bound::Unbounded, Bound::Excluded($b))
        };
        (..= $b: literal) => {
            RangeInterval::new(Bound::Unbounded, Bound::Included($b))
        };
        ($a: literal .. $b: literal) => {
            RangeInterval::new(Bound::Included($a), Bound::Excluded($b))
        };
        ($a: literal ..= $b: literal) => {
            RangeInterval::new(Bound::Included($a), Bound::Included($b))
        };
        ($a: literal ..) => {
            RangeInterval::new(Bound::Included($a), Bound::Unbounded)
        };
        ($a: literal) => {
            RangeInterval::new(Bound::Included($a), Bound::Included($a))
        };
        (!$a: literal ..) => {
            RangeInterval::new(Bound::Excluded($a), Bound::Unbounded)
        };
        (!$a: literal .. $b: literal) => {
            RangeInterval::new(Bound::Excluded($a), Bound::Excluded($b))
        };
        (!$a: literal ..= $b: literal) => {
            RangeInterval::new(Bound::Excluded($a), Bound::Included($b))
        };
    }
/// A range interval is a range with a start and end bound.
/// This has more case than standard std::ops ranges offer,
/// as here we can have Excluded start, which in std::ops is reversed to Included for example.
/// It returns correctly reverse of the range, unless the range is unbounded, than the reverse is not applicable
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RangeInterval<T> {
    /// if the interval is reverse
    /// for the simplicity of calculations the start will always be less or equal than the end
    pub reverse: bool,
    pub start: Bound<T>,
    pub end: Bound<T>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RangeType {
    /// This represent a range which is not inclusive on the end, and is supported by rust standard library.
    Range,
    /// This represent a range which is inclusive on the end, and is supported by rust standard library.
    RangeInclusive,
    /// This represent a range which is unbounded on the end, and is supported by rust standard library.
    RangeFrom,
    /// Unbounded range on both ends, and is supported by rust standard library.
    RangeFull,
    /// This represent a range which is unbounded on the start, and is supported by rust standard library.
    RangeTo,
    /// This represent a range which is inclusive on the end, and is supported by rust standard library.
    RangeToInclusive,
    /// This represent other range which is not supported by rust standard library.
    Other,
}

impl<T: PartialOrd> RangeInterval<T> {
    /// Test if a value is contained in the range
    pub fn contains(&self, value: T) -> bool {
        match (&self.start, &self.end) {
            (Bound::Unbounded, Bound::Unbounded) => true,
            (Bound::Unbounded, Bound::Included(end)) => value <= *end,
            (Bound::Unbounded, Bound::Excluded(end)) => value < *end,
            (Bound::Included(start), Bound::Unbounded) => value >= *start,
            (Bound::Excluded(start), Bound::Unbounded) => value > *start,
            (Bound::Included(start), Bound::Included(end)) => *start <= value && value <= *end,
            (Bound::Included(start), Bound::Excluded(end)) => *start <= value && value < *end,
            (Bound::Excluded(start), Bound::Included(end)) => *start < value && value <= *end,
            // the problem with excluded on both ends for integral values is that,
            // for example there is nothing between 1 and 2
            (Bound::Excluded(start), Bound::Excluded(end)) => *start < value && value < *end,
        }
    }
}

impl<T: PartialOrd> RangeInterval<T> {
    #[cfg(test)]
    pub(crate) fn rev_point(point: T) -> Self
    where T: Copy
    {
        Self { reverse: true, start: Bound::Included(point), end: Bound::Included(point) }
    }
    pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
        let (reverse, start, end) = match (&start, &end) {
            (Bound::Included(a) | Bound::Excluded(a), Bound::Included(b) | Bound::Excluded(b)) => {
                if a > b {
                    (true, end, start)
                } else {
                    (false, start, end)
                }
            }
            _ => (false, start, end),
        };
        Self { reverse, start, end }
    }

    /// Return the type of range
    pub fn range_type(&self) -> RangeType {
        match (self.reverse, &self.start, &self.end) {
            (_, Bound::Included(_), Bound::Included(_)) => RangeType::RangeInclusive,
            (_, Bound::Included(_), Bound::Excluded(_)) => RangeType::Range,
            (_, Bound::Included(_), Bound::Unbounded) => RangeType::RangeFrom,
            (true, Bound::Excluded(_), Bound::Included(_)) => RangeType::Range, // reverse Range
            // there is no representation in the standard library
            (_, Bound::Excluded(_), _) => RangeType::Other,
            (_, Bound::Unbounded, Bound::Included(_)) => RangeType::RangeToInclusive,
            (_, Bound::Unbounded, Bound::Excluded(_)) => RangeType::RangeTo,
            (_, Bound::Unbounded, Bound::Unbounded) => RangeType::RangeFull,
        }
    }

    /// Convert to a range if possible
    pub fn to_range(self) -> Option<Range<T>> {
        match (self.reverse, self.start, self.end) {
            (true, Bound::Excluded(end), Bound::Included(start)) |
            (false, Bound::Included(start), Bound::Excluded(end)) => Some(start..end),
            _ => None,
        }
    }

    /// Convert to a range inclusive if possible
    pub fn to_range_inclusive(self) -> Option<RangeInclusive<T>> {
        match (self.reverse, self.start, self.end) {
            (true, Bound::Included(end), Bound::Included(start)) |
            (false, Bound::Included(start), Bound::Included(end)) => Some(start..=end),
            _ => None,
        }
    }

    /// Convert to a range from if possible
    pub fn to_range_from(self) -> Option<RangeFrom<T>> {
        match (self.start, self.end) {
            (Bound::Included(start), Bound::Unbounded) => Some(start..),
            _ => None,
        }
    }

    /// Convert to a range full if possible
    pub fn to_range_full(self) -> Option<RangeFull> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Unbounded) => Some(..),
            _ => None,
        }
    }

    /// Convert to a range to if possible
    pub fn to_range_to(self) -> Option<RangeTo<T>> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Excluded(end)) => Some(..end),
            _ => None,
        }
    }

    /// Convert to a range to inclusive if possible
    pub fn to_range_to_inclusive(self) -> Option<RangeToInclusive<T>> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Included(end)) => Some(..=end),
            _ => None,
        }
    }

    pub fn try_to_range(self) -> Result<Range<T>, Self> {
        match (self.reverse, self.start, self.end) {
            (true, Bound::Excluded(end), Bound::Included(start)) |
            (false, Bound::Included(start), Bound::Excluded(end)) => Ok(start..end),
            (reverse, start, end) => Err(RangeInterval { reverse, start, end }),
        }
    }

    pub fn try_to_range_inclusive(self) -> Result<RangeInclusive<T>, Self> {
        match (self.reverse, self.start, self.end) {
            (true, Bound::Included(end), Bound::Included(start)) |
            (false, Bound::Included(start), Bound::Included(end)) => Ok(start..=end),
            (reverse, start, end) => Err(RangeInterval { reverse, start, end }),
        }
    }

    pub fn try_to_range_from(self) -> Result<RangeFrom<T>, Self> {
        match (self.start, self.end) {
            (Bound::Included(start), Bound::Unbounded) => Ok(start..),
            (a, b) => Err(RangeInterval::new(a, b)),
        }
    }

    pub fn try_to_range_full(self) -> Result<RangeFull, Self> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Unbounded) => Ok(..),
            (a, b) => Err(RangeInterval::new(a, b)),
        }
    }

    pub fn try_to_range_to(self) -> Result<RangeTo<T>, Self> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Excluded(end)) => Ok(..end),
            (a, b) => Err(RangeInterval::new(a, b)),
        }
    }

    pub fn try_to_range_to_inclusive(self) -> Result<RangeToInclusive<T>, Self> {
        match (self.start, self.end) {
            (Bound::Unbounded, Bound::Included(end)) => Ok(..=end),
            (a, b) => Err(RangeInterval::new(a, b)),
        }
    }
}

impl<T: PartialOrd> From<Range<T>> for RangeInterval<T> {
    fn from(range: Range<T>) -> Self {
        if range.start > range.end {
            Self {
                reverse: true,
                start: Bound::Excluded(range.end),
                end: Bound::Included(range.start),
            }
        } else {
            Self {
                reverse: false,
                start: Bound::Included(range.start),
                end: Bound::Excluded(range.end),
            }
        }
    }
}

impl<T: PartialOrd> From<RangeInclusive<T>> for RangeInterval<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        let (a, b) = range.into_inner();
        if a > b {
            Self {
                reverse: true,
                start: Bound::Included(b),
                end: Bound::Included(a),
            }
        } else {
            Self {
                reverse: false,
                start: Bound::Included(a),
                end: Bound::Included(b),
            }
        }
    }
}

impl<T> From<RangeFrom<T>> for RangeInterval<T> {
    fn from(range: RangeFrom<T>) -> Self {
        Self {
            reverse: false,
            start: Bound::Included(range.start),
            end: Bound::Unbounded,
        }
    }
}

impl<T> From<RangeFull> for RangeInterval<T> {
    fn from(_: RangeFull) -> Self {
        Self {
            reverse: false,
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl<T> From<RangeTo<T>> for RangeInterval<T> {
    fn from(range: RangeTo<T>) -> Self {
        Self {
            reverse: false,
            start: Bound::Unbounded,
            end: Bound::Excluded(range.end),
        }
    }
}

impl<T> From<RangeToInclusive<T>> for RangeInterval<T> {
    fn from(range: RangeToInclusive<T>) -> Self {
        Self {
            reverse: false,
            start: Bound::Unbounded,
            end: Bound::Included(range.end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        let a: RangeInterval<_> = (1..10).into();
        assert_eq!(RangeType::Range, a.range_type());
        assert_eq!(Some(1..10), a.to_range());

        let a: RangeInterval<_> = (10..1).into();
        assert_eq!(RangeType::Range, a.range_type());
        assert_eq!(Some(10..1), a.to_range());

        let a: RangeInterval<_> = (1..=10).into();
        assert_eq!(RangeType::RangeInclusive, a.range_type());
        assert_eq!(Some(1..=10), a.to_range_inclusive());

        let a: RangeInterval<_> = (10..=1).into();
        assert_eq!(RangeType::RangeInclusive, a.range_type());
        assert_eq!(Some(10..=1), a.to_range_inclusive());

        let a: RangeInterval<_> = (1..).into();
        assert_eq!(RangeType::RangeFrom, a.range_type());
        assert_eq!(Some(1..), a.to_range_from());

        let a: RangeInterval<i32> = (..).into();
        assert_eq!(RangeType::RangeFull, a.range_type());
        assert_eq!(Some(..), a.to_range_full());

        let a: RangeInterval<_> = (..10).into();
        assert_eq!(RangeType::RangeTo, a.range_type());
        assert_eq!(Some(..10), a.to_range_to());

        let a: RangeInterval<_> = (..=10).into();
        assert_eq!(RangeType::RangeToInclusive, a.range_type());
        assert_eq!(Some(..=10), a.to_range_to_inclusive());
    }

    #[test]
    fn test_contains() {
        assert!(r!(1..10).contains(5));
        assert!(r!(1..10).contains(1));
        assert!(!r!(1..10).contains(10));
        assert!(!r!(1..10).contains(0));
        assert!(!r!(1..10).contains(11));
        assert!(!r!(1..1).contains(1));
        assert!(!r!(1..1).contains(2));
        assert!(!r!(1..1).contains(0));

        assert!(!r!(!1..1).contains(1));
        assert!(!r!(!1..1).contains(0));
        assert!(!r!(!1..1).contains(2));

        assert!(r!(1..=10).contains(5));
        assert!(r!(1..=10).contains(1));
        assert!(r!(1..=10).contains(10));
        assert!(!r!(1..=10).contains(0));
        assert!(!r!(1..=10).contains(11));

        assert!(r!(..10).contains(5));
        assert!(!r!(..10).contains(10));
        assert!(!r!(..10).contains(11));

        assert!(r!(..=10).contains(5));
        assert!(r!(..=10).contains(10));
        assert!(!r!(..=10).contains(11));

        assert!(r!(10..).contains(11));
        assert!(r!(10..).contains(10));
        assert!(!r!(10..).contains(9));

        assert!(r!(10..=10).contains(10));
        assert!(!r!(10..=10).contains(9));
        assert!(!r!(10..=10).contains(11));

        assert!(r!(..).contains(0));
        assert!(r!(..).contains(100));
        assert!(r!(..).contains(-100));
        assert!(r!(..).contains(0.0));

        assert!(r!(!1..=10).contains(10));
        assert!(r!(!1..=10).contains(5));
        assert!(!r!(!1..=10).contains(1));
        assert!(!r!(!1..=10).contains(0));
        assert!(!r!(!1..=10).contains(11));

        assert!(!r!(!1..10).contains(10));
        assert!(r!(!1..10).contains(5));
        assert!(!r!(!1..10).contains(1));
        assert!(!r!(!1..10).contains(0));
        assert!(!r!(!1..10).contains(11));

        assert!(r!(!1..).contains(10));
        assert!(!r!(!1..).contains(1));
        assert!(!r!(!1..).contains(0));

        assert!(r!(!10..1).contains(5));
        assert!(!r!(!10..1).contains(0));
        assert!(!r!(!10..1).contains(1));
        assert!(!r!(!10..1).contains(10));
        assert!(!r!(!10..1).contains(11));
    }
}
