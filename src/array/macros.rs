/// Helper macro for creating an [`ArrayGrid`] type or instance.
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
                let _ = grid.set($p, true);
            )*
            grid
        }
    };
}
