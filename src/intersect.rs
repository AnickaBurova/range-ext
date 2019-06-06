use std::cmp::PartialOrd;
use std::ops::*;

/// More precise intersection of two ranges from point of the first range
#[derive(Debug, PartialEq)]
pub enum Intersection {
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

impl Intersection {
    /// Test if there is any intersection
    pub fn is_any(&self) -> bool {
        match self {
            Intersection::Bellow => false,
            Intersection::Above => false,
            _ => true,
        }
    }
    /// Test if the range is fully within the other
    pub fn is_within(&self) -> bool {
        match self {
            Intersection::Within | Intersection::Same => true,
            _ => false,
        }
    }

    /// Test if the range is fully over the other
    pub fn is_over(&self) -> bool {
        match self {
            Intersection::Over | Intersection::Same => true,
            _ => false,
        }
    }
}

pub trait Intersect<T: PartialOrd, U: RangeBounds<T>>: RangeBounds<T> {
    /// Test two ranges for an intersection
    fn intersect(&self, other: &U) -> Intersection;
}

impl<T: PartialOrd> Intersect<T, Range<T>> for Range<T> {
    fn intersect(&self, other: &Range<T>) -> Intersection {
        if self.end == other.end {
            if self.start < other.start {
                Intersection::Over
            } else if self.start > other.start {
                Intersection::Within
            } else {
                Intersection::Same
            }
        } else if self.end < other.end {
            if self.end <= other.start {
                Intersection::Bellow
            } else if self.start < other.start {
                Intersection::BellowOverlap
            } else {
                Intersection::Within
            }
        } else if self.start < other.end {
            if self.start <= other.start {
                Intersection::Over
            } else {
                Intersection::AboveOverlap
            }
        } else {
            Intersection::Above
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFrom<T>> for Range<T> {
    fn intersect(&self, other: &RangeFrom<T>) -> Intersection {
        if self.end <= other.start {
            Intersection::Bellow
        } else if self.start < other.start {
            Intersection::BellowOverlap
        } else {
            Intersection::Within
        }
    }
}

impl<T: PartialOrd> Intersect<T, RangeFull> for Range<T> {
    fn intersect(&self, _: &RangeFull) -> Intersection {
        Intersection::Same
    }
}

impl<T: PartialOrd> Intersect<T, RangeTo<T>> for Range<T> {
    fn intersect(&self, other: &RangeTo<T>) -> Intersection {
        if self.start >= other.end {
            Intersection::Above
        } else if self.end > other.end {
            Intersection::AboveOverlap
        } else {
            Intersection::Within
        }
    }
}

/*
 * TODO:
use num::Integer;

impl<T: PartialOrd + Copy + Integer> Intersect<T, RangeInclusive<T>> for Range<T> {
    fn intersect(&self, other: &RangeInclusive<T>) -> Intersection {
        let (a_start, a_end) = (self.start, self.end - T::one());
        let (b_start, b_end) = (*other.start(), *other.end());
        if a_end == b_end {
            if a_start < b_start {
                Intersection::Over
            } else if a_start > b_start {
                Intersection::Within
            } else {
                Intersection::Same
            }
        } else if a_end < b_end {
            if a_end <= b_start {
                Intersection::Bellow
            } else if a_start < b_start {
                Intersection::BellowOverlap
            } else {
                Intersection::Within
            }
        } else if a_start < b_end {
            if a_start <= b_start {
                Intersection::Over
            } else {
                Intersection::AboveOverlap
            }
        } else {
            Intersection::Above
        }
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test {
        ($a: expr) => {
            assert_eq!($a.intersect(&(11..11)), Intersection::Bellow);
            assert_eq!($a.intersect(&(10..11)), Intersection::Bellow);
            assert_eq!($a.intersect(&(9..11)), Intersection::BellowOverlap);
            assert_eq!($a.intersect(&(9..10)), Intersection::Over);
            assert_eq!($a.intersect(&(3..10)), Intersection::Same);
            assert_eq!($a.intersect(&(5..9)), Intersection::Over);
            assert_eq!($a.intersect(&(3..9)), Intersection::Over);
            assert_eq!($a.intersect(&(2..11)), Intersection::Within);
            assert_eq!($a.intersect(&(3..11)), Intersection::Within);
            assert_eq!($a.intersect(&(2..9)), Intersection::AboveOverlap);
            assert_eq!($a.intersect(&(2..3)), Intersection::Above);
            assert_eq!($a.intersect(&(1..2)), Intersection::Above);

            assert_eq!($a.intersect(&(11..)), Intersection::Bellow);
            assert_eq!($a.intersect(&(10..)), Intersection::Bellow);
            assert_eq!($a.intersect(&(9..)), Intersection::BellowOverlap);
            assert_eq!($a.intersect(&(3..)), Intersection::Within);
            assert_eq!($a.intersect(&(2..)), Intersection::Within);

            assert_eq!($a.intersect(&(..)), Intersection::Within);

            assert_eq!($a.intersect(&(..11)), Intersection::Within);
            assert_eq!($a.intersect(&(..10)), Intersection::Within);
            assert_eq!($a.intersect(&(..9)), Intersection::AboveOverlap);
            assert_eq!($a.intersect(&(..3)), Intersection::Above);
            assert_eq!($a.intersect(&(..2)), Intersection::Above);
        };
    }
    #[test]
    pub fn range_test() {
        test!(3..10);

        match (10..22).intersect(&(0..11)) {
            Intersection::Bellow => (),
            Intersection::BellowOverlap => (),
            Intersection::Within => (),
            Intersection::Same => (),
            Intersection::Over => (),
            Intersection::AboveOverlap => (),
            Intersection::Above => (),
        }
    }
}
