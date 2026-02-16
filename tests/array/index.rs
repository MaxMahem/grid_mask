use crate::macros::{test_ctor, test_self_method};
use grid_mask::{ArrayIndex, err::OutOfBounds};

type Index8 = ArrayIndex<8, 8>;
type Index4x3 = ArrayIndex<4, 3>;

const INDEX_10: Index8 = Index8::const_new::<10>();

mod new {
    use super::*;
    test_ctor!(min: Index8::new(0) => Ok(Index8::MIN));
    test_ctor!(max: Index8::new(63) => Ok(Index8::MAX));
    test_ctor!(square_oob: Index8::new(64) => Err(OutOfBounds));
    test_ctor!(rect_oob: Index4x3::new(12) => Err(OutOfBounds));
}

mod get {
    use super::*;
    test_self_method!(min: Index8::MIN => get() => 0);
    test_self_method!(max: Index8::MAX => get() => 63);
    test_self_method!(val: INDEX_10 => get() => 10);
}

mod eq {
    use super::*;
    test_self_method!(eq_min: Index8::MIN => eq(&0) => true);
    test_self_method!(eq_max: Index8::MAX => eq(&63) => true);
    test_self_method!(eq_val: INDEX_10 => eq(&10) => true);
    test_self_method!(ne_val: INDEX_10 => eq(&11) => false);
}
