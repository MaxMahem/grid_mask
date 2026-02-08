use crate::macros::test_property;
use grid_mask::ext::NotWhitespace;

test_property!(pass: 'a' => is_not_whitespace() => true);
test_property!(fail: ' ' => is_not_whitespace() => false);
