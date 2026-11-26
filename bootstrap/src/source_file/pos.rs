use std::ops::{Add, AddAssign, Sub, SubAssign};

macro_rules! impl_pos {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
        )*
    ) => {
        $(
            $(#[$attr])*
            $vis struct $ident($inner_vis $inner_ty);

            impl From<usize> for $ident {
                fn from(value: usize) -> Self {
                    Self(value as $inner_ty)
                }
            }

            impl From<u32> for $ident {
                fn from(value: u32) -> Self {
                    Self(value as $inner_ty)
                }
            }

            impl From<$ident> for usize {
                fn from(value: $ident) -> Self {
                    value.0 as usize
                }
            }

            impl From<$ident> for u32 {
                fn from(value: $ident) -> Self {
                    value.0 as u32
                }
            }

            impl Add for $ident {
                type Output = $ident;

                fn add(self, rhs: Self) -> Self {
                    Self(self.0 + rhs.0)
                }
            }

            impl AddAssign for $ident {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0
                }
            }

            impl Sub for $ident {
                type Output = $ident;

                fn sub(self, rhs: Self) -> Self {
                    Self(self.0 - rhs.0)
                }
            }

            impl SubAssign for $ident {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0
                }
            }
        )*
    };
}

impl_pos! {
    /// A byte offset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BytePos(u32);
}
