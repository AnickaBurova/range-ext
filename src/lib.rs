//! Intersection trait for ranges.
macro_rules! try_unwrap {
    ($a: expr, $or: expr) => {
        match $a {
            Some(a) => a,
            None => return $or,
        }
    }
}
pub mod intersect;

pub mod split;
