use crate::macros::test_self_method;
use grid_mask::ext::NotWhitespace;

test_self_method!(pass: 'a' => is_not_whitespace() => true);
test_self_method!(fail: ' ' => is_not_whitespace() => false);
