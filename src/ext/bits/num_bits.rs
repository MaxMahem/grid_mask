pub trait NumBits {
    /// The number of bits in the type.
    const BITS: u8;
}

macro_rules! impl_bits {
    ($($ty:ty => $bits:expr),* $(,)?) => {
        $(
            impl NumBits for $ty {
                const BITS: u8 = $bits;
            }
        )*
    };
}

impl_bits!(
    u8 => 8,
    u16 => 16,
    u32 => 32,
    u64 => 64,
    u128 => 128,
);
