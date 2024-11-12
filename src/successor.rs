/// A trait that represents a numerical next and previous value.
pub trait Successor: Sized {
    /// The next value from this value
    fn next(&self) -> Option<Self>;

    /// The previous value from this value
    fn prev(&self) -> Option<Self>;
}


macro_rules! impl_int {
    ($t: ty) => {
        impl Successor for $t {
            fn next(&self) -> Option<Self> {
                self.checked_add(1)
            }
            fn prev(&self) -> Option<Self> {
                self.checked_sub(1)
            }
        }
    }
}

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(isize);
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(usize);
