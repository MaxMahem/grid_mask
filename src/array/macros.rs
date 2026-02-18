/// Helper macro for creating an [`ArrayGrid`](crate::array::ArrayGrid) type or instance.
#[macro_export]
macro_rules! array_grid {
    // Branch for creating the type
    ($W:expr, $H:expr) => {
        $crate::array::ArrayGrid<$W, $H, { usize::div_ceil($W * $H, u64::BITS as usize) }>
    };
    // Branch for creating an instance from a list of points
    ($W:expr, $H:expr; [ $( $p:expr ),* $(,)? ]) => {
        {
            let mut grid = <$crate::array::ArrayGrid<$W, $H, { usize::div_ceil($W * $H, u64::BITS as usize) }>>::EMPTY;
            $(
                let index = $crate::array::ArrayIndex::<$W, $H>::const_new::<{ let (x, y) = $p; x as u32 + y as u32 * $W as u32 }>();
                grid.const_set(index, true);
            )*
            grid
        }
    };
}
