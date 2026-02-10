#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! test_ctor {
    ($id:ident: $ctor:expr => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            assert_eq!($ctor, $expected);
            Ok(())
        }
    };
}

macro_rules! test_panic {
    ($id:ident: $ctor:expr => $msg:expr) => {
        #[test]
        #[should_panic(expected = $msg)]
        fn $id() {
            let _ = $ctor;
        }
    };
}

macro_rules! test_transform {
    // Branch for pattern matching with explicit 'matches' keyword
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => matches $pattern:pat) => {
        #[test]
        #[allow(clippy::redundant_pattern_matching)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let val = $ctor;
            let result = val.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert!(matches!(result, $pattern), "Expected pattern {}, got {:?}", stringify!($pattern), result);
            Ok(())
        }
    };
    // Branch for partial equality (expr)
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let val = $ctor;
            let result = val.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(result, $expected);
            Ok(())
        }
    };
}

macro_rules! test_mutation {
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let mut val = $ctor;
            val.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(val, $expected);
            Ok(())
        }
    };
}

macro_rules! test_property {
    ($id:ident: $ctor:expr => $prop:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let val = $ctor;
            let prop = val.$prop $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(prop, $expected);
            Ok(())
        }
    };
}

macro_rules! test_iter {
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let val = $ctor;
            let result: Vec<_> = val.$method $( ::< $($gen),+ > )? ( $($($arg),*)? ).collect();
            let expected_slice: &[bool] = $expected.as_ref();
            assert_eq!(result.as_slice(), expected_slice);
            Ok(())
        }
    };
}

macro_rules! test_foreach {
    ($id:ident: $ctor:expr => $method:ident($arg_name:ident in $iter:expr) => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let val = $ctor;
            for $arg_name in $iter {
                let result = val.$method($arg_name);
                assert_eq!(result, $expected, "Failed for input {:?}", $arg_name);
            }
            Ok(())
        }
    };
}

macro_rules! test_foreach_mut {
    ($id:ident: $ctor:expr => $method:ident($arg_name:ident in $iter:expr) => $expected:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            for $arg_name in $iter {
                let mut val = $ctor;
                val.$method($arg_name);
                assert_eq!(val, $expected, "Failed for input {:?}", $arg_name);
            }
            Ok(())
        }
    };
}

pub(crate) use test_ctor;
pub(crate) use test_foreach;
pub(crate) use test_foreach_mut;
pub(crate) use test_iter;
pub(crate) use test_mutation;
pub(crate) use test_panic;
pub(crate) use test_property;
pub(crate) use test_transform;
