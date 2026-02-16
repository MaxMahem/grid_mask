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
    // Branch for pattern matching with explicit 'matches' keyword and dot-syntax
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => matches $pattern:pat) => {
        #[test]
        #[allow(clippy::redundant_pattern_matching, unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let this = $ctor;
            let result = this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert!(matches!(result, $pattern), "Expected pattern {}, got {:?}", stringify!($pattern), result);
            Ok(())
        }
    };
    // // Branch for pattern matching with explicit 'matches' keyword and custom identifier/expression
    // ($id:ident: $this:ident = $ctor:expr => $call:expr => matches $pattern:pat) => {
    //     #[test]
    //     #[allow(clippy::redundant_pattern_matching, unused_variables)]
    //     fn $id() -> Result<(), Box<dyn std::error::Error>> {
    //         let $this = $ctor;
    //         let result = $call;
    //         assert!(matches!(result, $pattern), "Expected pattern {}, got {:?}", stringify!($pattern), result);
    //         Ok(())
    //     }
    // };
    // Branch for partial equality (expr) and dot-syntax
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        #[allow(unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let this = $ctor;
            let result = this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(result, $expected);
            Ok(())
        }
    };
    // // Branch for partial equality (expr) and custom identifier/expression
    // ($id:ident: $this:ident = $ctor:expr => $call:expr => $expected:expr) => {
    //     #[test]
    //     #[allow(unused_variables)]
    //     fn $id() -> Result<(), Box<dyn std::error::Error>> {
    //         let $this = $ctor;
    //         let result = $call;
    //         assert_eq!(result, $expected);
    //         Ok(())
    //     }
    // };
}

macro_rules! test_mutation {
    // Branch for mutation using dot-syntax
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        #[allow(unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let mut this = $ctor;
            this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(this, $expected);
            Ok(())
        }
    };
    // // Branch for mutation using custom identifier/expression
    // ($id:ident: $this:ident = $ctor:expr => $call:expr => $expected:expr) => {
    //     #[test]
    //     #[allow(unused_variables)]
    //     fn $id() -> Result<(), Box<dyn std::error::Error>> {
    //         let mut $this = $ctor;
    //         let _ = $call;
    //         assert_eq!($this, $expected);
    //         Ok(())
    //     }
    // };
}

// This is the macro that should be used for testing methods that return a value
// and do not mutate the struct.
macro_rules! test_self_method {
    // Branch for methods called using dot syntax: this.method(arg)
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
        #[test]
        #[allow(unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let this = $ctor;
            let prop = this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(prop, $expected);
            Ok(())
        }
    };
    // Branch for methods called using path syntax or complex expressions with explicit binding: id: this = ctor => Type::method(this, arg)
    ($id:ident: $this:ident = $ctor:expr => $call:expr => $expected:expr) => {
        #[test]
        #[allow(unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let $this = $ctor;
            let prop = $call;
            assert_eq!(prop, $expected);
            Ok(())
        }
    };
}

// macro_rules! test_iter {
//     // Branch for iterators using dot-syntax
//     ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => $expected:expr) => {
//         #[test]
//         #[allow(unused_variables)]
//         fn $id() -> Result<(), Box<dyn std::error::Error>> {
//             let this = $ctor;
//             let result: Vec<_> = this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? ).collect();
//             let expected: Vec<_> = $expected.into_iter().collect();
//             assert_eq!(result, expected);
//             Ok(())
//         }
//     };
// }

// macro_rules! test_foreach {
//     ($id:ident: $ctor:expr => $method:ident($arg_name:ident in $iter:expr) => $expected:expr) => {
//         #[test]
//         #[allow(unused_variables)]
//         fn $id() -> Result<(), Box<dyn std::error::Error>> {
//             let this = $ctor;
//             for $arg_name in $iter {
//                 let result = this.$method($arg_name);
//                 assert_eq!(result, $expected, "Failed for input {:?}", $arg_name);
//             }
//             Ok(())
//         }
//     };
// }

// macro_rules! test_foreach_mut {
//     ($id:ident: $ctor:expr => $method:ident($arg_name:ident in $iter:expr) => $expected:expr) => {
//         #[test]
//         fn $id() -> Result<(), Box<dyn std::error::Error>> {
//             for $arg_name in $iter {
//                 let mut val = $ctor;
//                 val.$method($arg_name);
//                 assert_eq!(val, $expected, "Failed for input {:?}", $arg_name);
//             }
//             Ok(())
//         }
//     };
// }

macro_rules! test_try_mutation {
    // Branch for try_mutation using dot-syntax
    ($id:ident: $ctor:expr => $method:ident $( ::< $($gen:ty),+ > )? $( ( $($arg:expr),* ) )? => ($result:expr, $expected:expr)) => {
        #[test]
        #[allow(unused_variables)]
        fn $id() -> Result<(), Box<dyn std::error::Error>> {
            let mut this = $ctor;
            let result = this.$method $( ::< $($gen),+ > )? ( $($($arg),*)? );
            assert_eq!(result, $result);
            assert_eq!(this, $expected);
            Ok(())
        }
    };
    // Branch for try_mutation using custom identifier/expression
    // ($id:ident: $this:ident = $ctor:expr => $call:expr => ($result:expr, $expected:expr)) => {
    //     #[test]
    //     #[allow(unused_variables)]
    //     fn $id() -> Result<(), Box<dyn std::error::Error>> {
    //         let mut $this = $ctor;
    //         let result = $call;
    //         assert_eq!(result, $result);
    //         assert_eq!($this, $expected);
    //         Ok(())
    //     }
    // };
}

pub(crate) use test_ctor;
//pub(crate) use test_foreach;
//pub(crate) use test_foreach_mut;
//pub(crate) use test_iter;
pub(crate) use test_mutation;
pub(crate) use test_panic;
pub(crate) use test_self_method;
pub(crate) use test_transform;
pub(crate) use test_try_mutation;
