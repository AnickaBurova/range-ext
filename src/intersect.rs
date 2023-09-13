use std::cmp::{Ordering, PartialOrd};
use std::ops::*;

/// Represents the three possible types of intersection between two ranges.
#[derive(Debug, PartialEq)]
pub enum Intersection {
    /// The two ranges do not overlap at all.
    Empty,

    /// The two ranges overlap each other but none is contained within the other.
    Overlap,

    /// One range is fully contained in the other (they may be identical).
    Full,
}

impl Intersection {
    /// Checks if there is any intersection. Returns false only when the intersection is Empty.
    pub fn is_any(&self) -> bool {
        match self {
            Intersection::Empty => false,
            _ => true,
        }
    }
}

/// Extends the Intersection enum with additional cases specific to ranges, from the point of the first
/// range.
#[derive(Debug, PartialEq)]
pub enum IntersectionExt {
    /// One or the both of the ranges are empty, hence cannot intersect.
    Empty,
    /// The first range starts and ends before the second range, hence is fully less than.
    Less,

    /// The first range starts before the second is less, but overlaps with it with the end.
    LessOverlap,

    /// The first range is contained within the second range.
    Within,

    /// Two ranges are identical.
    Same,

    /// The first range contains the second range.
    Over,

    /// The first range ends after (greater) the first range, but overlaps with it.
    GreaterOverlap,

    /// The first range is after the second range, is fully greater than.
    Greater,
}

impl IntersectionExt {
    /// Converts an IntersectionExt instance into an Intersection instance.
    pub fn intersection(&self) -> Intersection {
        match self {
            IntersectionExt::Empty => Intersection::Empty,
            IntersectionExt::Less => Intersection::Empty,
            IntersectionExt::LessOverlap => Intersection::Overlap,
            IntersectionExt::Within => Intersection::Full,
            IntersectionExt::Same => Intersection::Full,
            IntersectionExt::Over => Intersection::Full,
            IntersectionExt::GreaterOverlap => Intersection::Overlap,
            IntersectionExt::Greater => Intersection::Empty,
        }
    }

    /// Checks if there is any intersection.
    pub fn is_any(&self) -> bool {
        match self {
            IntersectionExt::Empty |
            IntersectionExt::Less |
            IntersectionExt::Greater => false,
            _ => true,
        }
    }

    /// Checks if this range is contained within the other range. Identical is within as well.
    pub fn is_within(&self) -> bool {
        match self {
            IntersectionExt::Within | IntersectionExt::Same => true,
            _ => false,
        }
    }
}

/// A type implementing this trait can return the intersection between itself and another range.
pub trait Intersect<T: PartialOrd, U: RangeBounds<T>>: RangeBounds<T> {
    /// Return extended intersection between the two ranges, from the point of the first range.
    fn intersect_ext(&self, other: &U) -> IntersectionExt;
    /// Return the intersection between the two ranges.
    fn intersect(&self, other: &U) -> Intersection;
    /// Checks if the two ranges intersect.
    fn does_intersect(&self, other:&U) -> bool;
}

macro_rules! empty_and_reverse {
    ($a: ident, $b: ident, $fun: ident, $empty: expr) => {
        if $a.start == $a.end || $b.start == $b.end {
            return $empty;
        }
        if $a.start > $a.end {
            return ($a.end .. $a.start).$fun($b);
        }
        if $b.start > $b.end {
            return $a.$fun(&($b.end .. $b.start));
        }
    }
}

fn do_intersect_ext<T: PartialOrd>(a: &T, b: &T, x: &T, y: &T) -> IntersectionExt {
    if b == y {
        if a < x {
            IntersectionExt::Over
        } else if a > x {
            IntersectionExt::Within
        } else {
            IntersectionExt::Same
        }
    } else if b < y {
        if b <= x {
            IntersectionExt::Less
        } else if a < x {
            IntersectionExt::LessOverlap
        } else {
            IntersectionExt::Within
        }
    } else if a < y {
        if a <= x {
            IntersectionExt::Over
        } else {
            IntersectionExt::GreaterOverlap
        }
    } else {
        IntersectionExt::Greater
    }
}

impl<T: PartialOrd + Copy> Intersect<T, Range<T>> for Range<T> {
    /// Determines and returns the IntersectionExt between two ranges.
    fn intersect_ext(&self, other: &Range<T>) -> IntersectionExt {
        let (a, b) = match self.start.partial_cmp(&self.end) {
            None | Some(Ordering::Equal) => return IntersectionExt::Empty,
            Some(Ordering::Less) => (&self.start, &self.end),
            Some(Ordering::Greater) => (&self.end, &self.start),
        };
        let (x, y) = match other.start.partial_cmp(&other.end) {
            None | Some(Ordering::Equal) => return IntersectionExt::Empty,
            Some(Ordering::Less) => (&other.start, &other.end),
            Some(Ordering::Greater) => (&other.end, &other.start),
        };
        do_intersect_ext(a, b, x, y)
    }

    fn intersect(&self, other: &Range<T>) -> Intersection {
        empty_and_reverse!(self, other, intersect, Intersection::Empty);
        if self.end <= other.start || self.start >= other.end {
            Intersection::Empty
        } else {
            match (self.start.partial_cmp(&other.start), self.end.partial_cmp(&other.end)) {
                (None, _) => Intersection::Empty,
                (_, None) => Intersection::Empty,
                (Some(Ordering::Equal), _) => Intersection::Full,
                (Some(Ordering::Less), Some(Ordering::Less)) => Intersection::Overlap,
                (Some(Ordering::Less), _) => Intersection::Full,
                (Some(Ordering::Greater), Some(Ordering::Greater)) => Intersection::Overlap,
                (Some(Ordering::Greater), _) => Intersection::Full,
            }
        }
    }

    fn does_intersect(&self, other: &Range<T>) -> bool {
        empty_and_reverse!(self, other, does_intersect, false);
        if self.end <= other.start || self.start >= other.end {
            false
        } else {
            match (self.start.partial_cmp(&other.start), self.end.partial_cmp(&other.end)) {
                (None, _) => false,
                (_, None) => false,
                _ => true,
            }
        }
    }
}

macro_rules! empty_and_reverse_a {
    ($a: ident, $b: ident, $fun: ident, $empty: expr) => {
        if $a.start == $a.end {
            return $empty;
        }
        if $a.start > $a.end {
            return ($a.end .. $a.start).$fun($b);
        }
    }
}

impl<T: PartialOrd + Copy> Intersect<T, RangeFrom<T>> for Range<T> {
    /// Determines and returns the IntersectionExt between a bounded range and a range starting from a value.
    fn intersect_ext(&self, other: &RangeFrom<T>) -> IntersectionExt {
        empty_and_reverse_a!(self, other, intersect_ext, IntersectionExt::Empty);
        if self.end <= other.start {
            IntersectionExt::Less
        } else if self.start >= other.start {
            IntersectionExt::Within
        } else {
            IntersectionExt::LessOverlap
        }
    }

    fn intersect(&self, other: &RangeFrom<T>) -> Intersection {
        empty_and_reverse_a!(self, other, intersect, Intersection::Empty);
        if self.end <= other.start {
            Intersection::Empty
        } else if self.start >= other.start {
            Intersection::Full
        } else {
            Intersection::Overlap
        }
    }

    fn does_intersect(&self, other: &RangeFrom<T>) -> bool {
        empty_and_reverse_a!(self, other, does_intersect, false);
        if self.end <= other.start {
            false
        } else {
            true
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for Range<T> {
    /// A range is always within a full range, so always return IntersectionFull::Within.
    fn intersect_ext(&self, _: &RangeFull) -> IntersectionExt {
        if self.start == self.end {
            IntersectionExt::Empty
        } else {
            IntersectionExt::Within
        }
    }

    fn intersect(&self, _: &RangeFull) -> Intersection {
        if self.start == self.end {
            Intersection::Empty
        } else {

            Intersection::Full
        }
    }

    fn does_intersect(&self, _: &RangeFull) -> bool {
        if self.start == self.end {
            false
        } else {
            true
        }
    }
}

impl<T: PartialOrd + Copy> Intersect<T, RangeTo<T>> for Range<T> {
    /// Determines and returns the IntersectionExt between a bounded range and a range ending at a value.
    fn intersect_ext(&self, other: &RangeTo<T>) -> IntersectionExt {
        empty_and_reverse_a!(self, other, intersect_ext, IntersectionExt::Empty);
        if self.start >= other.end {
            IntersectionExt::Greater
        } else if self.end > other.end {
            IntersectionExt::GreaterOverlap
        } else {
            IntersectionExt::Within
        }
    }

    fn intersect(&self, other: &RangeTo<T>) -> Intersection {
        empty_and_reverse_a!(self, other, intersect, Intersection::Empty);
        if self.start >= other.end {
            Intersection::Empty
        } else if self.end > other.end {
            Intersection::Overlap
        } else {
            Intersection::Full
        }
    }

    fn does_intersect(&self, other: &RangeTo<T>) -> bool {
        empty_and_reverse_a!(self, other, does_intersect, false);
        if self.start >= other.end {
            false
        } else if self.end > other.end {
            true
        } else {
            true
        }
    }
}

macro_rules! test_if_reverse {
    ($a: expr, $b: expr, $x: expr, $y: expr, $empty: expr) => {{
        let (a, b) = match $a.partial_cmp($b) {
            None => return $empty,
            Some(Ordering::Less) | Some(Ordering::Equal) => ($a, $b),
            Some(Ordering::Greater) => ($b, $a),
        };
        let (x, y) = match $x.partial_cmp($y) {
            None  => return $empty,
            Some(Ordering::Less) | Some(Ordering::Equal) => ($x, $y),
            Some(Ordering::Greater) => ($y, $x),
        };
        (a, b, x, y)
    } }
}

impl<T: PartialOrd + Copy> Intersect<T, RangeInclusive<T>> for RangeInclusive<T> {
    fn intersect_ext(&self, other: &RangeInclusive<T>) -> IntersectionExt {
        // no empty ranges as these are inclusive
        let (a, b, x, y) = test_if_reverse!(self.start(), self.end(), other.start(), other.end(), IntersectionExt::Empty);
        if b == y {
            if a < x {
                IntersectionExt::Over
            } else if a > x {
                IntersectionExt::Within
            } else {
                IntersectionExt::Same
            }
        } else if a == y {
            IntersectionExt::GreaterOverlap
        } else if b < y {
            if b < x {
                IntersectionExt::Less
            } else if a < x {
                IntersectionExt::LessOverlap
            } else {
                IntersectionExt::Within
            }
        } else if a < y {
            if a <= x {
                IntersectionExt::Over
            } else {
                IntersectionExt::GreaterOverlap
            }
        } else {
            IntersectionExt::Greater
        }
    }

    fn intersect(&self, other: &RangeInclusive<T>) -> Intersection {
        // no empty ranges as these are inclusive
        let (a, b, x, y) = test_if_reverse!(self.start(), self.end(), other.start(), other.end(), Intersection::Empty);
        let ax = try_unwrap!(a.partial_cmp(&x), Intersection::Empty);
        let by = try_unwrap!(b.partial_cmp(&y), Intersection::Empty);
        let ay = try_unwrap!(a.partial_cmp(&y), Intersection::Empty);
        let bx = try_unwrap!(b.partial_cmp(&x), Intersection::Empty);

        match (ax, ay, bx, by) {
            (_, Ordering::Greater, _ , _) => Intersection::Empty,
            (_, _, Ordering::Less, _) => Intersection::Empty,
            (Ordering::Equal, _,_,_) => Intersection::Full,
            (_, _, _, Ordering::Equal) => Intersection::Full,
            (Ordering::Less, _,_,Ordering::Greater) => Intersection::Full,
            (Ordering::Greater, _, _, Ordering::Less) => Intersection::Full,
            _ => Intersection::Overlap,
        }
    }

    fn does_intersect(&self, other: &RangeInclusive<T>) -> bool {
        let (a, b, x, y) = test_if_reverse!(self.start(), self.end(), other.start(), other.end(), false);
        let ay = try_unwrap!(a.partial_cmp(&y), false);
        let bx = try_unwrap!(b.partial_cmp(&x), false);

        match (ay, bx) {
            (Ordering::Greater, _) => false,
            (_, Ordering::Less) => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_intersect() {
        // Overlapping cases
        assert_eq!((5..10).intersect_ext(&(3..7)), IntersectionExt::GreaterOverlap);
        assert_eq!((5..10).intersect_ext(&(8..15)), IntersectionExt::LessOverlap);
        assert_eq!((10..5).intersect_ext(&(3..7)), IntersectionExt::GreaterOverlap);
        assert_eq!((10..5).intersect_ext(&(8..15)), IntersectionExt::LessOverlap);
        assert_eq!((10..5).intersect_ext(&(7..3)), IntersectionExt::GreaterOverlap);
        assert_eq!((10..5).intersect_ext(&(15..8)), IntersectionExt::LessOverlap);

        // Complete overlap/within
        assert_eq!((5..10).intersect_ext(&(5..10)), IntersectionExt::Same);
        assert_eq!((5..10).intersect_ext(&(4..11)), IntersectionExt::Within);
        assert_eq!((5..10).intersect_ext(&(5..20)), IntersectionExt::Within);

        // Containing
        assert_eq!((5..10).intersect_ext(&(6..9)), IntersectionExt::Over);
        assert_eq!((5..10).intersect_ext(&(5..6)), IntersectionExt::Over);
        assert_eq!((5..10).intersect_ext(&(6..10)), IntersectionExt::Over);

        // Completely below or above
        assert_eq!((5..10).intersect_ext(&(10..15)), IntersectionExt::Less);
        assert_eq!((5..10).intersect_ext(&(0..5)), IntersectionExt::Greater);

        // Intersecting with empty ranges
        assert_eq!((5..10).intersect_ext(&(5..5)), IntersectionExt::Empty);
        assert_eq!((5..5).intersect_ext(&(3..7)), IntersectionExt::Empty);
        assert_eq!((5..5).intersect_ext(&(6..6)), IntersectionExt::Empty);


        assert_eq!((5..10).intersect(&(0..5)), Intersection::Empty);
        assert_eq!((5..10).intersect(&(11..20)), Intersection::Empty);
        assert_eq!((5..5).intersect(&(11..20)), Intersection::Empty);
        assert_eq!((5..10).intersect(&(5..5)), Intersection::Empty);
        assert_eq!((5..10).intersect(&(5..10)), Intersection::Full);
        assert_eq!((5..10).intersect(&(4..10)), Intersection::Full);
        assert_eq!((5..10).intersect(&(4..12)), Intersection::Full);
        assert_eq!((5..10).intersect(&(5..12)), Intersection::Full);
        assert_eq!((5..10).intersect(&(6..12)), Intersection::Overlap);
        assert_eq!((5..10).intersect(&(4..8)), Intersection::Overlap);
        assert_eq!((5..10).intersect(&(5..11)), Intersection::Full);


        assert_eq!((5..10).does_intersect(&(5..10)), true);
        assert_eq!((5..10).does_intersect(&(5..10)), true);
        assert_eq!((5..5).does_intersect(&(5..10)), false);

        assert_eq!((5..10).does_intersect(&(5..5)), false);
        assert_eq!((5..10).does_intersect(&(5..10)), true);
        assert_eq!((5..10).does_intersect(&(4..10)), true);
        assert_eq!((5..10).does_intersect(&(4..12)), true);
        assert_eq!((5..10).does_intersect(&(5..12)), true);
        assert_eq!((5..10).does_intersect(&(6..12)), true);
        assert_eq!((5..10).does_intersect(&(4..8)), true);
        assert_eq!((5..10).does_intersect(&(5..11)), true);
    }

    #[test]
    fn range_from_intersect() {
        assert_eq!((1..10).intersect_ext(&(5..)), IntersectionExt::LessOverlap);
        assert_eq!((1..10).intersect_ext(&(11..)), IntersectionExt::Less);
        assert_eq!((1..10).intersect_ext(&(1..)), IntersectionExt::Within);
        assert_eq!((1..10).intersect_ext(&(0..)), IntersectionExt::Within);
        assert_eq!((1..10).intersect_ext(&(10..)), IntersectionExt::Less);

        // reverse
        assert_eq!((10..1).intersect_ext(&(5..)), IntersectionExt::LessOverlap);
        assert_eq!((10..1).intersect_ext(&(11..)), IntersectionExt::Less);
        assert_eq!((10..1).intersect_ext(&(1..)), IntersectionExt::Within);
        assert_eq!((10..1).intersect_ext(&(0..)), IntersectionExt::Within);

        assert_eq!((1..1).intersect_ext(&(0..)), IntersectionExt::Empty);


        assert_eq!((1..10).intersect(&(10..)), Intersection::Empty);
        assert_eq!((1..10).intersect(&(5..)), Intersection::Overlap);
        assert_eq!((1..10).intersect(&(11..)), Intersection::Empty);
        assert_eq!((1..10).intersect(&(1..)), Intersection::Full);
        assert_eq!((1..10).intersect(&(0..)), Intersection::Full);
        assert_eq!((1..1).intersect(&(0..)), Intersection::Empty);
        assert_eq!((10..1).intersect(&(5..)), Intersection::Overlap);
        assert_eq!((10..1).intersect(&(11..)), Intersection::Empty);
        assert_eq!((10..1).intersect(&(1..)), Intersection::Full);
        assert_eq!((10..1).intersect(&(0..)), Intersection::Full);


        assert_eq!((1..10).does_intersect(&(10..)), false);
        assert_eq!((1..10).does_intersect(&(5..)), true);
        assert_eq!((1..10).does_intersect(&(11..)), false);
        assert_eq!((1..10).does_intersect(&(1..)), true);
        assert_eq!((1..10).does_intersect(&(0..)), true);
        assert_eq!((1..1) .does_intersect(&(0..)), false);
        assert_eq!((10..1).does_intersect(&(5..)), true);
        assert_eq!((10..1).does_intersect(&(11..)), false);
        assert_eq!((10..1).does_intersect(&(1..)), true);
        assert_eq!((10..1).does_intersect(&(0..)), true);
    }

    #[test]
    fn range_full_intersect() {
        assert_eq!((1..10).intersect_ext(&..), IntersectionExt::Within);
        assert_eq!((1..1).intersect_ext(&..), IntersectionExt::Empty);

        assert_eq!((1..10).intersect(&..), Intersection::Full);
        assert_eq!((1..1).intersect(&..), Intersection::Empty);

        assert_eq!((1..10).does_intersect(&..), true);
        assert_eq!((1..1).does_intersect(&..), false);
    }

    #[test]
    fn range_to_intersect() {
        assert_eq!((1..1).intersect_ext(&(..0)), IntersectionExt::Empty);
        assert_eq!((1..10).intersect_ext(&(..0)), IntersectionExt::Greater);
        assert_eq!((1..10).intersect_ext(&(..1)), IntersectionExt::Greater);
        assert_eq!((1..10).intersect_ext(&(..2)), IntersectionExt::GreaterOverlap);
        assert_eq!((1..10).intersect_ext(&(..10)), IntersectionExt::Within);
        assert_eq!((1..10).intersect_ext(&(..11)), IntersectionExt::Within);

        assert_eq!((10..1).intersect_ext(&(..0)), IntersectionExt::Greater);
        assert_eq!((10..1).intersect_ext(&(..1)), IntersectionExt::Greater);
        assert_eq!((10..1).intersect_ext(&(..2)), IntersectionExt::GreaterOverlap);
        assert_eq!((10..1).intersect_ext(&(..10)), IntersectionExt::Within);
        assert_eq!((10..1).intersect_ext(&(..11)), IntersectionExt::Within);

        assert_eq!((1..1). intersect(&(..0)), Intersection::Empty);
        assert_eq!((1..10).intersect(&(..0)), Intersection::Empty);
        assert_eq!((1..10).intersect(&(..1)), Intersection::Empty);
        assert_eq!((1..10).intersect(&(..2)), Intersection::Overlap);
        assert_eq!((1..10).intersect(&(..10)), Intersection::Full);
        assert_eq!((1..10).intersect(&(..11)), Intersection::Full);

        assert_eq!((10..1).intersect(&(..0)), Intersection::Empty);
        assert_eq!((10..1).intersect(&(..1)), Intersection::Empty);
        assert_eq!((10..1).intersect(&(..2)), Intersection::Overlap);
        assert_eq!((10..1).intersect(&(..10)), Intersection::Full);
        assert_eq!((10..1).intersect(&(..11)), Intersection::Full);
    }

    #[test]
    fn functions_test() {
        assert_eq!(Intersection::Overlap.is_any(), true);
        assert_eq!(Intersection::Empty.is_any(), false);
        assert_eq!(IntersectionExt::Less.is_any(), false);
        assert_eq!(IntersectionExt::LessOverlap.is_any(), true);
        assert_eq!(IntersectionExt::Within.is_any(), true);
        assert_eq!(IntersectionExt::Same.is_any(), true);
        assert_eq!(IntersectionExt::Over.is_any(), true);
        assert_eq!(IntersectionExt::GreaterOverlap.is_any(), true);
        assert_eq!(IntersectionExt::Greater.is_any(), false);
        assert_eq!(IntersectionExt::Less.is_within(), false);
        assert_eq!(IntersectionExt::LessOverlap.is_within(), false);
        assert_eq!(IntersectionExt::Within.is_within(), true);
        assert_eq!(IntersectionExt::Same.is_within(), true);
        assert_eq!(IntersectionExt::Over.is_within(), false);
        assert_eq!(IntersectionExt::GreaterOverlap.is_within(), false);
        assert_eq!(IntersectionExt::Greater.is_within(), false);
    }

    #[test]
    fn range_inclusive_test() {
        assert_eq!((10..=1).intersect_ext(&(15..=1)), IntersectionExt::Within);
        assert_eq!((10..=1).intersect_ext(&(10..=1)), IntersectionExt::Same);
        assert_eq!((10..=1).intersect_ext(&(9..=1)), IntersectionExt::Over);
        assert_eq!((10..=1).intersect_ext(&(10..=9)), IntersectionExt::Over);
        assert_eq!((10..=1).intersect_ext(&(10..=0)), IntersectionExt::Within);
        assert_eq!((10..=1).intersect_ext(&(1..=0)), IntersectionExt::GreaterOverlap);
        assert_eq!((10..=1).intersect_ext(&(0..=-1)), IntersectionExt::Greater);
        assert_eq!((10..=1).intersect_ext(&(11..=10)), IntersectionExt::LessOverlap);
        assert_eq!((10..=1).intersect_ext(&(12..=11)), IntersectionExt::Less);

        assert_eq!((10..=1).intersect(&(15..=1)), Intersection::Full);
        assert_eq!((10..=1).intersect(&(10..=1)), Intersection::Full);
        assert_eq!((10..=1).intersect(&(9..=1)), Intersection::Full);
        assert_eq!((10..=1).intersect(&(10..=9)), Intersection::Full);
        assert_eq!((10..=1).intersect(&(10..=0)), Intersection::Full);
        assert_eq!((10..=1).intersect(&(1..=0)), Intersection::Overlap);
        assert_eq!((10..=1).intersect(&(0..=-1)), Intersection::Empty);
        assert_eq!((10..=1).intersect(&(11..=10)), Intersection::Overlap);
        assert_eq!((10..=1).intersect(&(12..=11)), Intersection::Empty);

        assert_eq!((10..=1).does_intersect(&(15..=1)), true);
        assert_eq!((10..=1).does_intersect(&(10..=1)), true);
        assert_eq!((10..=1).does_intersect(&(9..=1)), true);
        assert_eq!((10..=1).does_intersect(&(10..=9)), true);
        assert_eq!((10..=1).does_intersect(&(10..=0)), true);
        assert_eq!((10..=1).does_intersect(&(1..=0)), true);
        assert_eq!((10..=1).does_intersect(&(0..=-1)), false);
        assert_eq!((10..=1).does_intersect(&(11..=10)), true);
        assert_eq!((10..=1).does_intersect(&(12..=11)), false);
    }
}