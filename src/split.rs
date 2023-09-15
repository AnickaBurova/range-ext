//! Split a range by other range. The result can be None, single, or two ranges.


use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};

#[derive(PartialEq, Debug)]
pub enum SplitResultTwo<L, U> {
    None,
    First(L),
    Second(U),
    Both(L, U),
}

pub trait Split<O> {
    type Result: PartialEq;

    fn split(&self, other: &O) -> Self::Result;
}


impl<T: PartialOrd + Clone> Split<Range<T>> for RangeInclusive<T> {
    /// The lower part split in two would be non inclusive, as the start of the other range is inclusive
    type Result = SplitResultTwo<Range<T>, RangeInclusive<T>>;

    fn split(&self, other: &Range<T>) -> Self::Result {
        // a .. b is self
        // x .. y is other
        let ay = try_unwrap!(self.start().partial_cmp(&other.end), SplitResultTwo::None);
        // x .. y     a .. b
        if ay != Ordering::Less { // other is lower than this
            return SplitResultTwo::Second(self.clone());
        }
        let bx = try_unwrap!(self.end().partial_cmp(&other.start), SplitResultTwo::None);
        // a .. b    x .. y
        if bx == Ordering::Less { // other is higher than this
            return SplitResultTwo::Second(self.clone());
        }
        // a .. b=x .. y
        if bx == Ordering::Equal { // the other is touching the end of this, so remove the end
            return SplitResultTwo::First(self.start().clone()..self.end().clone());
        }
        let ax = try_unwrap!(self.start().partial_cmp(&other.start), SplitResultTwo::None);
        let by = try_unwrap!(self.end().partial_cmp(&other.end), SplitResultTwo::None);
        match (ax, by) {
            // x<=a    y<=b    - a can be equal or greater than x
            (Ordering::Equal | Ordering::Greater, Ordering::Greater | Ordering::Equal) => SplitResultTwo::Second(other.end.clone() ..=self.end().clone()),
            // x<=a    b  y   - nothing left
            (Ordering::Equal | Ordering::Greater, Ordering::Less) => SplitResultTwo::None,
            // a   x    y<=b
            (Ordering::Less, Ordering::Greater | Ordering::Equal) => SplitResultTwo::Both(self.start().clone() .. other.start.clone(), other.end.clone() ..=self.end().clone()),
            // a x b y
            (Ordering::Less, Ordering::Less) => SplitResultTwo::First(self.start().clone()..other.start.clone()),
        }
    }
}

impl<T: PartialOrd + Clone> Split<Range<T>> for Range<T> {
    type Result = ();
    fn split(&self, other: &Range<T>) -> Self::Result {
        // a .. b is self
        // x .. y is other
        // let ay = try_unwrap!(self.start().partial_cmp(&other.end), SplitResult::None);
        // // x .. y     a .. b
        // if ay != Ordering::Less { // other is lower than this
        //     return SplitResult::Upper(self.clone());
        // }
        // let bx = try_unwrap!(self.end().partial_cmp(&other.start), SplitResult::None);
        // // a .. b    x .. y
        // if bx == Ordering::Less { // other is higher than this
        //     return SplitResult::Upper(self.clone());
        // }
        // // a .. b=x .. y
        // if bx == Ordering::Equal { // the other is touching the end of this, so remove the end
        //     return SplitResult::Lower(self.start().clone()..self.end().clone());
        // }
        // let ax = try_unwrap!(self.start().partial_cmp(&other.start), SplitResult::None);
        // let by = try_unwrap!(self.end().partial_cmp(&other.end), SplitResult::None);
        // match (ax, by) {
        //     // x<=a    y<=b    - a can be equal or greater than x
        //     (Ordering::Equal | Ordering::Greater, Ordering::Greater | Ordering::Equal) => SplitResult::Upper(other.end.clone() ..=self.end().clone()),
        //     // x<=a    b  y   - nothing left
        //     (Ordering::Equal | Ordering::Greater, Ordering::Less) => SplitResult::None,
        //     // a   x    y<=b
        //     (Ordering::Less, Ordering::Greater | Ordering::Equal) => SplitResult::Two(self.start().clone() .. other.start.clone(), other.end.clone() ..=self.end().clone()),
        //     // a x b y
        //     (Ordering::Less, Ordering::Less) => SplitResult::Lower(self.start().clone()..other.start.clone()),
        // }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_vs_range_inclusive() {
        assert_eq!(SplitResultTwo::Second(1..=10), (1..=10).split(&(0..1)));
        assert_eq!(SplitResultTwo::Second(1..=10), (1..=10).split(&(-3..0)));
        assert_eq!(SplitResultTwo::Second(1..=10), (1..=10).split(&(11..30)));
        assert_eq!(SplitResultTwo::First(1..10), (1..=10).split(&(10..30)));

        assert_eq!(SplitResultTwo::Second(5..=10), (1..=10).split(&(1..5)));
        assert_eq!(SplitResultTwo::Second(5..=10), (1..=10).split(&(0..5)));
        assert_eq!(SplitResultTwo::Second(10..=10), (1..=10).split(&(0..10)));
        assert_eq!(SplitResultTwo::Second(10..=10), (1..=10).split(&(1..10)));

        assert_eq!(SplitResultTwo::None, (1..=10).split(&(1..11)));
        assert_eq!(SplitResultTwo::None, (1..=10).split(&(0..11)));

        assert_eq!(SplitResultTwo::Both(1..5, 10..=10), (1..=10).split(&(5..10)));
        assert_eq!(SplitResultTwo::Both(1..5, 9..=10), (1..=10).split(&(5..9)));

        assert_eq!(SplitResultTwo::First(1..5), (1..=10).split(&(5..11)));
    }
}