pub trait AbsDiff<A, B> {
    fn abs_diff(&self, other: A) -> B;
}

macro_rules! absdiff {
    ($type:ty) => {
        impl AbsDiff<$type, $type> for $type {
            fn abs_diff(&self, other: $type) -> $type {
                if *self > other {
                    *self - other
                } else {
                    other - *self
                }
            }
        }

        impl AbsDiff<&$type, $type> for $type {
            fn abs_diff(&self, other: &$type) -> $type {
                if self > other {
                    self - other
                } else {
                    other - self
                }
            }
        }
    };
}

absdiff!(u128);
absdiff!(i128);
absdiff!(u64);
absdiff!(i64);
absdiff!(u32);
absdiff!(i32);
absdiff!(u8);
absdiff!(i8);
absdiff!(usize);

pub trait Clamp<A, B, C> {
    fn clamp(&self, min: B, max: C) -> A;
}

macro_rules! clamp {
    ($type:ty) => {
        impl Clamp<$type, $type, $type> for $type {
            fn clamp(&self, min: $type, max: $type) -> $type {
                if *self < min {
                    min
                } else if *self > max {
                    max
                } else {
                    *self
                }
            }
        }

        impl Clamp<$type, &$type, &$type> for $type {
            fn clamp(&self, min: &$type, max: &$type) -> $type {
                if self < min {
                    *min
                } else if self > max {
                    *max
                } else {
                    *self
                }
            }
        }
    };
}

clamp!(u128);
clamp!(i128);
clamp!(u64);
clamp!(i64);
clamp!(u32);
clamp!(i32);
clamp!(u8);
clamp!(i8);
clamp!(usize);
