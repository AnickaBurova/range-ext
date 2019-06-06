use std::cmp::PartialOrd;
use std::ops::*;

/// Intersection of two ranges from point of the first range
#[derive(Debug)]
pub enum Intersection {
    /// They have no common area
    Empty,
    /// They partially overlap
    Overlap,
    /// One is fully within the other
    Full,
}

impl Intersection {
    /// Test if there is any intersection
    /// '''
    ///     Intersection::Empty => false,
    ///     Intersection::Overlap => true,
    ///     Intersection::Full => true,
    pub fn is_any(&self) -> bool {
        match self {
            Intersection::Empty => false,
            _ => true,
        }
    }
}

/// More precise intersection of two ranges from point of the first range
#[derive(Debug, PartialEq)]
pub enum IntersectionExt {
    /// The self is below the other
    Bellow,
    /// The self is below but overlaping
    BellowOverlap,
    /// The self is within the other
    Within,
    /// The self is same as the other
    Same,
    /// The self is over the other, the other is within the self
    Over,
    /// The self is above but overlaping
    AboveOverlap,
    /// The self is above the other
    Above,
}

impl IntersectionExt {
    /// Get simpler intersection
    pub fn intersection(&self) -> Intersection {
        match self {
            IntersectionExt::Bellow => Intersection::Empty,
            IntersectionExt::BellowOverlap => Intersection::Overlap,
            IntersectionExt::Within => Intersection::Full,
            IntersectionExt::Same => Intersection::Full,
            IntersectionExt::Over => Intersection::Full,
            IntersectionExt::AboveOverlap => Intersection::Overlap,
            IntersectionExt::Above => Intersection::Empty,
        }
    }
    /// Test if there is any intersection
    pub fn is_any(&self) -> bool {
        match self {
            IntersectionExt::Bellow => false,
            IntersectionExt::Above => false,
            _ => true,
        }
    }
    /// Test if the range is fully within the other
    pub fn is_within(&self) -> bool {
        match self {
            IntersectionExt::Within | IntersectionExt::Same => true,
            _ => false,
        }
    }
}

pub trait Intersect<T: PartialOrd, U: RangeBounds<T>>: RangeBounds<T> {
    /// Test two ranges for an intersection
    fn intersect(&self, other: &U) -> IntersectionExt;
}

impl<T: PartialOrd> Intersect<T, Range<T>> for Range<T> {
    fn intersect(&self, other: &Range<T>) -> IntersectionExt {
        if self.end == other.end {
            if self.start < other.start {
                IntersectionExt::Over
            } else if self.start > other.start {
                IntersectionExt::Within
            } else {
                IntersectionExt::Same
            }
        } else if self.end < other.end {
            if self.end <= other.start {
                IntersectionExt::Bellow
            } else if self.start < other.start {
                IntersectionExt::BellowOverlap
            } else {
                IntersectionExt::Within
            }
        } else if self.start < other.end {
            if self.start <= other.start {
                IntersectionExt::Over
            } else {
                IntersectionExt::AboveOverlap
            }
        } else {
            IntersectionExt::Above
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFrom<T>> for Range<T> {
    fn intersect(&self, other: &RangeFrom<T>) -> IntersectionExt {
        if self.end <= other.start {
            IntersectionExt::Bellow
        } else if self.start < other.start {
            IntersectionExt::BellowOverlap
        } else {
            IntersectionExt::Within
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for Range<T> {
    fn intersect(&self, _: &RangeFull) -> IntersectionExt {
        IntersectionExt::Within
    }
}

impl<T: PartialOrd> Intersect<T, RangeTo<T>> for Range<T> {
    fn intersect(&self, other: &RangeTo<T>) -> IntersectionExt {
        if self.start >= other.end {
            IntersectionExt::Above
        } else if self.end > other.end {
            IntersectionExt::AboveOverlap
        } else {
            IntersectionExt::Within
        }
    }
}

/*
 * TODO:
use num::Integer;

impl<T: PartialOrd + Copy + Integer> Intersect<T, RangeInclusive<T>> for Range<T> {
    fn intersect(&self, other: &RangeInclusive<T>) -> IntersectionExt {
        let (a_start, a_end) = (self.start, self.end - T::one());
        let (b_start, b_end) = (*other.start(), *other.end());
        if a_end == b_end {
            if a_start < b_start {
                IntersectionExt::Over
            } else if a_start > b_start {
                IntersectionExt::Within
            } else {
                IntersectionExt::Same
            }
        } else if a_end < b_end {
            if a_end <= b_start {
                IntersectionExt::Bellow
            } else if a_start < b_start {
                IntersectionExt::BellowOverlap
            } else {
                IntersectionExt::Within
            }
        } else if a_start < b_end {
            if a_start <= b_start {
                IntersectionExt::Over
            } else {
                IntersectionExt::AboveOverlap
            }
        } else {
            IntersectionExt::Above
        }
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test {
        ($a: expr) => {
            assert_eq!($a.intersect(&(11..11)), IntersectionExt::Bellow);
            assert_eq!($a.intersect(&(10..11)), IntersectionExt::Bellow);
            assert_eq!($a.intersect(&(9..11)), IntersectionExt::BellowOverlap);
            assert_eq!($a.intersect(&(9..10)), IntersectionExt::Over);
            assert_eq!($a.intersect(&(3..10)), IntersectionExt::Same);
            assert_eq!($a.intersect(&(5..9)), IntersectionExt::Over);
            assert_eq!($a.intersect(&(3..9)), IntersectionExt::Over);
            assert_eq!($a.intersect(&(2..11)), IntersectionExt::Within);
            assert_eq!($a.intersect(&(3..11)), IntersectionExt::Within);
            assert_eq!($a.intersect(&(2..9)), IntersectionExt::AboveOverlap);
            assert_eq!($a.intersect(&(2..3)), IntersectionExt::Above);
            assert_eq!($a.intersect(&(1..2)), IntersectionExt::Above);

            assert_eq!($a.intersect(&(11..)), IntersectionExt::Bellow);
            assert_eq!($a.intersect(&(10..)), IntersectionExt::Bellow);
            assert_eq!($a.intersect(&(9..)), IntersectionExt::BellowOverlap);
            assert_eq!($a.intersect(&(3..)), IntersectionExt::Within);
            assert_eq!($a.intersect(&(2..)), IntersectionExt::Within);

            assert_eq!($a.intersect(&(..)), IntersectionExt::Within);

            assert_eq!($a.intersect(&(..11)), IntersectionExt::Within);
            assert_eq!($a.intersect(&(..10)), IntersectionExt::Within);
            assert_eq!($a.intersect(&(..9)), IntersectionExt::AboveOverlap);
            assert_eq!($a.intersect(&(..3)), IntersectionExt::Above);
            assert_eq!($a.intersect(&(..2)), IntersectionExt::Above);
        };
    }
    #[test]
    pub fn range_test() {
        test!(3..10);

        match (10..22).intersect(&(0..11)) {
            IntersectionExt::Bellow => (),
            IntersectionExt::BellowOverlap => (),
            IntersectionExt::Within => (),
            IntersectionExt::Same => (),
            IntersectionExt::Over => (),
            IntersectionExt::AboveOverlap => (),
            IntersectionExt::Above => (),
        }
    }
}
