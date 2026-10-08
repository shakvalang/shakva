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

            impl $ident {
                pub fn from_usize(n: usize) -> Self {
                    Self(n as $inner_ty)
                }

                pub fn to_usize(&self) -> usize {
                    self.0 as usize
                }

                pub fn from_u32(n: u32) -> Self {
                    Self(n as $inner_ty)
                }

                pub fn to_u32(&self) -> u32 {
                    self.0 as u32
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
    pub struct BytePos(pub u32);
}
