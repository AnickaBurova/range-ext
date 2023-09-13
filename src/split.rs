//! Split a range by other range. The result can be None, single, or two ranges.


use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};

#[derive(PartialEq, Debug)]
pub enum SplitResult<L: PartialEq, U: PartialEq> {
    None,
    Lower(L),
    Upper(U),
    Two(L, U),
}

pub trait Split<O> {
    type Lower: PartialEq;
    type Upper: PartialEq;

    fn split(&self, other: &O) -> SplitResult<Self::Lower, Self::Upper>;
}


impl<T: PartialOrd + Clone> Split<Range<T>> for RangeInclusive<T> {
    /// The lower part split in two would be non inclusive, as the start of the other range is inclusive
    type Lower = Range<T>;
    type Upper = RangeInclusive<T>;

    fn split(&self, other: &Range<T>) -> SplitResult<Self::Lower, Self::Upper> {
        // a .. b is self
        // x .. y is other
        let ay = try_unwrap!(self.start().partial_cmp(&other.end), SplitResult::None);
        // x .. y     a .. b
        if ay != Ordering::Less { // other is lower than this
            return SplitResult::Upper(self.clone());
        }
        let bx = try_unwrap!(self.end().partial_cmp(&other.start), SplitResult::None);
        // a .. b    x .. y
        if bx == Ordering::Less { // other is higher than this
            return SplitResult::Upper(self.clone());
        }
        // a .. b=x .. y
        if bx == Ordering::Equal { // the other is touching the end of this, so remove the end
            return SplitResult::Lower(self.start().clone()..self.end().clone());
        }
        let ax = try_unwrap!(self.start().partial_cmp(&other.start), SplitResult::None);
        let by = try_unwrap!(self.end().partial_cmp(&other.end), SplitResult::None);
        match (ax, by) {
            // x<=a    y<=b    - a can be equal or greater than x
            (Ordering::Equal | Ordering::Greater, Ordering::Greater | Ordering::Equal) => SplitResult::Upper(other.end.clone() ..=self.end().clone()),
            // x<=a    b  y   - nothing left
            (Ordering::Equal | Ordering::Greater, Ordering::Less) => SplitResult::None,
            // a   x    y<=b
            (Ordering::Less, Ordering::Greater | Ordering::Equal) => SplitResult::Two(self.start().clone() .. other.start.clone(), other.end.clone() ..=self.end().clone()),
            // a x b y
            (Ordering::Less, Ordering::Less) => SplitResult::Lower(self.start().clone()..other.start.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_vs_range_inclusive() {
        assert_eq!(SplitResult::Upper(1..=10), (1..=10).split(&(0..1)));
        assert_eq!(SplitResult::Upper(1..=10), (1..=10).split(&(-3..0)));
        assert_eq!(SplitResult::Upper(1..=10), (1..=10).split(&(11..30)));
        assert_eq!(SplitResult::Lower(1..10), (1..=10).split(&(10..30)));

        assert_eq!(SplitResult::Upper(5..=10), (1..=10).split(&(1..5)));
        assert_eq!(SplitResult::Upper(5..=10), (1..=10).split(&(0..5)));
        assert_eq!(SplitResult::Upper(10..=10), (1..=10).split(&(0..10)));
        assert_eq!(SplitResult::Upper(10..=10), (1..=10).split(&(1..10)));

        assert_eq!(SplitResult::None, (1..=10).split(&(1..11)));
        assert_eq!(SplitResult::None, (1..=10).split(&(0..11)));

        assert_eq!(SplitResult::Two(1..5, 10..=10), (1..=10).split(&(5..10)));
        assert_eq!(SplitResult::Two(1..5, 9..=10), (1..=10).split(&(5..9)));

        assert_eq!(SplitResult::Lower(1..5), (1..=10).split(&(5..11)));
    }
}