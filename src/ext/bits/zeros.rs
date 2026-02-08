#[sealed::sealed]
pub trait BitZeros {
    /// Returns the number of trailing zeros as a `u8`.
    fn trailing_zeros_u8(self) -> u8;

    /// Returns the number of leading zeros as a `u8`.
    fn leading_zeros_u8(self) -> u8;
}

macro_rules! impl_bit_zeros {
    ($($t:ty),*) => {
        $(
            #[sealed::sealed]
            impl BitZeros for $t {
                #[inline]
                #[expect(clippy::cast_possible_truncation, reason = "bit counts for primitive types always fit in u8")]
                fn trailing_zeros_u8(self) -> u8 {
                    self.trailing_zeros() as u8
                }

                #[inline]
                #[expect(clippy::cast_possible_truncation, reason = "bit counts for primitive types always fit in u8")]
                fn leading_zeros_u8(self) -> u8 {
                    self.leading_zeros() as u8
                }
            }
        )*
    };
}

// impl_bit_zeros!(u8, u16, u32, u64, u128, usize);
impl_bit_zeros!(u8, u64);
