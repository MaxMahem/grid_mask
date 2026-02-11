/// [`debug_assert!`] that the `test` expression is true, then evaluates and
/// returns `then`.
///
/// `debug_check_then!(test => then, msg?)`
///
/// # Macro Values
///
/// - `test`: The expression to test
/// - `then`: The expression to evaluate if `test` is true
/// - `msg`: An optional message to print if `test` is false
macro_rules! debug_check_then {
    // no message
    ($test:expr => $then:expr) => {{
        debug_assert!($test);
        $then
    }};

    // with message
    ($test:expr => $then:expr, $($msg:tt)+) => {{
        debug_assert!($test, $($msg)+);
        $then
    }};
}

/// A safety `check` that guards the `unsafe` `then` expression.
///
/// Note that the `check` is only performed in debug builds.
///
/// `safety_check!(Safety: [msg] check => then)`
///
/// # Macro Values
///
/// - `msg`: The message to assert if `check` is false
/// - `check`: The expression to test
/// - `then`: The `unsafe` expression to evaluate if `check` is true
macro_rules! safety_check {
    (Safety: [$($msg:tt)+] if $check:expr => $then:expr) => {{
        debug_assert!($check, $($msg)+);
        unsafe { $then }
    }};
}

/// [`assert!`]s in a `const` block that the `test` expression is true, then
/// evaluates and returns `then`.
///
/// `const_assert_then!(test => then, msg?)`
///
/// # Macro Values
///
/// - `test`: The expression to test
/// - `then`: The expression to evaluate if `test` is true
/// - `msg`: An optional message to print if `test` is false
macro_rules! const_assert_then {
    ($test:expr => $then:expr $(, $msg:literal)?) => {
        const {
            assert!($test $(, $msg)?);
            $then
        }
    };
}

/// [`assert!`]s in a `const` block that the `test` expression is true.
///
/// `const_assert!(test, msg?);`
///
/// # Macro Values
///
/// - `test`: The expression to test
/// - `msg`: An optional message to print if `test` is false
macro_rules! const_assert {
    ($test:expr $(, $msg:literal)?) => {
        const {
            assert!($test $(, $msg)?);
        }
    };
}

/// [`assert!`] that the `test` expression is true, then evaluates and
/// returns `then`.
///
/// `assert_then!(test => then, msg?)`
///
/// # Macro Values
///
/// - `test`: The expression to test
/// - `then`: The expression to evaluate if `test` is true
/// - `msg`: An optional message to print if `test` is false
macro_rules! assert_then {
    // no message
    ($test:expr => $then:expr) => {
        {
            assert!($test);
            $then
        }
    };

    // with message
    ($test:expr => $then:expr $(, $($msg:tt)+)?) => {
        {
            assert!($test $(, $($msg)+)?);
            $then
        }
    };

    ($test:expr => $then:expr $(, $($msg:tt)+)?) => {
        {
            assert!($test $(, $($msg)+)?);
            $then
        }
    };
}

pub(crate) use assert_then;
pub(crate) use const_assert;
pub(crate) use const_assert_then;
pub(crate) use debug_check_then;
// pub(crate) use safety_check;
