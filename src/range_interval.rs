///! Unified structure for representing ranges and intervals.

use std::fmt::{Display, Formatter};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use crate::successor::Successor;

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

pub trait DisplayExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T: DisplayExt> Display for RangeInterval<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match if self.reverse { &self.end } else { &self.start } {
            Bound::Included(v) => {write!(f, "<")?; v.fmt(f)?;}
            Bound::Excluded(v) => {write!(f, "(")?; v.fmt(f)?;}
            Bound::Unbounded => {}
        }
        write!(f, "..")?;

        match if self.reverse { &self.start } else { &self.end } {
            Bound::Included(v) => { v.fmt(f)?;write!(f, ">")?;}
            Bound::Excluded(v) => { v.fmt(f)?;write!(f, ")")?;}
            Bound::Unbounded => {}
        }
        Ok(())
    }
}

fn into_bound<T, F: From<T>>(bound: Bound<T>) -> Bound<F> {
    match bound {
        Bound::Included(v) => Bound::Included(F::from(v)),
        Bound::Excluded(v) => Bound::Excluded(F::from(v)),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl<T> RangeInterval<T> {
    pub fn into<F: From<T>>(self) -> RangeInterval<F> {
        RangeInterval { reverse: self.reverse, start: into_bound(self.start), end: into_bound(self.end) }
    }
    
    /// Apply function to both ends of the range mapping the range in to different type
    pub fn map<R, F: Fn(&T) -> R>(&self, f: F) -> RangeInterval<R> {
        let start = match &self.start {
            Bound::Included(a) => {Bound::Included(f(a))}
            Bound::Excluded(a) => {Bound::Excluded(f(a))}
            Bound::Unbounded => { Bound::Unbounded }
        };
        let end = match &self.end {
            Bound::Included(a) => {Bound::Included(f(a))}
            Bound::Excluded(a) => {Bound::Excluded(f(a))}
            Bound::Unbounded => { Bound::Unbounded }
        };
        RangeInterval {
            start,
            end,
            reverse: self.reverse,
        }
    }
}

impl<T: Copy> RangeInterval<T> {
    pub fn display<D: DisplayExt + From<T>>(self) -> RangeInterval<D> {
        RangeInterval {
            reverse: self.reverse,
            start: match &self.start {
                Bound::Included(v) => Bound::Included(D::from(*v)),
                Bound::Excluded(v) => Bound::Excluded(D::from(*v)),
                Bound::Unbounded => Bound::Unbounded,
            },
            end: match &self.end {
                Bound::Included(v) => Bound::Included(D::from(*v)),
                Bound::Excluded(v) => Bound::Excluded(D::from(*v)),
                Bound::Unbounded => Bound::Unbounded,
            },
        }
    }

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
    /// Test if the range is not empty
    pub fn is_empty(&self) -> bool {
        match (&self.start, &self.end) {
            // it can only be empty if the start is equal to end, and at least one of the is excluded
            (Bound::Included(a), Bound::Excluded(b)) => a==b, // empty if the included is also excluded (!)
            (Bound::Excluded(a), Bound::Included(b)) => a==b,
            (Bound::Excluded(a), Bound::Excluded(b)) => a==b,
            _ => false,
        }
    }
    
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

impl<T: PartialOrd + Clone> RangeInterval<T> {
    /// Test the value if it is contained in the range, and return a value which is outside. Choose the
    /// direction to go either down or up. It requires a function to resolve `included` bounds, to calculate
    /// "next" or "previous" values.
    /// Return None if overflow occurs or the range is Full, or unbounded in the specified direction.
    pub fn avoid_with(&self, value: T, direction_end: bool, included: impl Fn(&T, bool) -> Option<T>) -> Option<T> {
        match (&self.start, &self.end) {
            (Bound::Unbounded, Bound::Unbounded) => None, // nowhere to avoid
            (Bound::Unbounded, Bound::Included(end)) => {
                if value <= *end {
                    // avoiding
                    if direction_end {
                        included(end, direction_end)
                    } else {
                        None
                    }
                } else {
                    Some(value)
                }
            },
            (Bound::Unbounded, Bound::Excluded(end)) => if value < *end {
                if direction_end {
                    Some(end.clone())
                } else {
                    None
                }
            } else {
                Some(value)
            }
            (Bound::Included(start), Bound::Unbounded) => if value >= *start {
                if direction_end {
                    None
                } else {
                    included(start, direction_end)
                }
            } else {
                Some(value)
            }
            (Bound::Excluded(start), Bound::Unbounded) => if value > *start {
                if direction_end {
                    None
                } else {
                    Some(start.clone())
                }
            } else {
                Some(value)
            }
            (Bound::Included(start), Bound::Included(end)) => if *start <= value && value <= *end {
                if direction_end {
                    included(end, direction_end)
                } else {
                    included(start, direction_end)
                }
            } else {
                Some(value)
            }
            (Bound::Included(start), Bound::Excluded(end)) => if *start <= value && value < *end {
                if direction_end {
                    Some(end.clone())
                } else {
                    included(start, direction_end)
                }
            } else {
                Some(value)
            }
            (Bound::Excluded(start), Bound::Included(end)) => if *start < value && value <= *end {
                if direction_end {
                    included(end, direction_end)
                } else {
                    Some(start.clone())
                }
            } else {
                Some(value)
            }
            (Bound::Excluded(start), Bound::Excluded(end)) => if *start < value && value < *end {
                if direction_end {
                    Some(end.clone())
                } else {
                    Some(start.clone())
                }
            } else {
                Some(value)
            }
        }
    }
}


impl<T: PartialOrd + Successor + Clone> RangeInterval<T> {
    pub fn avoid(&self, value: T, direction_end: bool ) -> Option<T> {
        self.avoid_with(value, direction_end, |v, dir| if dir { v.next() } else { v.prev() })
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

    #[test]
    fn test_avoid() {
        assert_eq!(r!(..).avoid(15, true), None);
        assert_eq!(r!(..).avoid(15, false), None);
        assert_eq!(r!(1..10).avoid(15, true), Some(15));
        assert_eq!(r!(1..10).avoid(15, false), Some(15));
        assert_eq!(r!(1..10).avoid(5, true), Some(10));
        assert_eq!(r!(1..10).avoid(5, false), Some(0));
        assert_eq!(r!(!1..10).avoid(5, false), Some(1));
        assert_eq!(r!(!1..).avoid(5, false), Some(1));
        assert_eq!(r!(1..).avoid(5, false), Some(0));
        assert_eq!(r!(1..).avoid(5, true), None);
        assert_eq!(r!(!1..).avoid(5, true), None);
        assert_eq!(r!(..10).avoid(5, false), None);
        assert_eq!(r!(..=10).avoid(5, false), None);
        assert_eq!(r!(..10).avoid(5, true), Some(10));
        assert_eq!(r!(..=10).avoid(5, true), Some(11));

        assert_eq!(r!(!1..10).avoid(15, false), Some(15));
        assert_eq!(r!(!1..10).avoid(15, true), Some(15));
        assert_eq!(r!(1..=10).avoid(15, false), Some(15));
        assert_eq!(r!(1..=10).avoid(15, true), Some(15));
        assert_eq!(r!(..=10).avoid(15, true), Some(15));
        assert_eq!(r!(..10).avoid(15, true), Some(15));
        assert_eq!(r!(..=10).avoid(15, false), Some(15));
        assert_eq!(r!(..10).avoid(15, false), Some(15));
        assert_eq!(r!(1..).avoid(-15, true), Some(-15));
        assert_eq!(r!(!1..).avoid(-15, true), Some(-15));
        assert_eq!(r!(1..).avoid(-15, false), Some(-15));
        assert_eq!(r!(!1..).avoid(-15, false), Some(-15));
    }

    #[test]
    fn test_is_empty() {
        assert!(!r!(1..).is_empty());
        assert!(!r!(..1).is_empty());
        assert!(!r!(1..10).is_empty());
        assert!(!r!(!1..10).is_empty());
        assert!(!r!(!1..=10).is_empty());
        assert!(!r!(1..=10).is_empty());
        assert!(!r!(1..=1).is_empty());
        assert!(r!(1..1).is_empty());
        assert!(r!(!1..1).is_empty());
        assert!(r!(!1..=1).is_empty());
    }
    
    #[test]
    fn test_map() {
        assert_eq!(r!(1..10).map(|v| 1.0 + (*v as f64)), r!(2.0 .. 11.0));
        assert_eq!(r!(1..=10).map(|v| 1.0 + (*v as f64)), r!(2.0 ..= 11.0));
        assert_eq!(r!(!1..=10).map(|v| 1.0 + (*v as f64)), r!(!2.0 ..=11.0));
        assert_eq!(r!(!1..10).map(|v| 1.0 + (*v as f64)), r!(!2.0 ..11.0));
        assert_eq!(r!(1..).map(|v| 1.0 + (*v as f64)), r!(2.0 ..));
    }
}
