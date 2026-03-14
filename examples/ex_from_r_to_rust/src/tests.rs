use crate::*;

#[test]
fn test_example_fn_sig() {
    struct TestStruct {}
    impl ExampleFnSig for TestStruct {
        fn mean(x: Vec<u32>) -> i32 {
            let _ = x;
            0i32
        }
        fn do_something(x: &mut Vec<u32>) -> i32 {
            let _ = x;
            0i32
        }
    }
    assert_eq!(TestStruct::mean(vec![1, 2, 3]), 0i32,);

    let mut data: Vec<u32> = vec![1, 2, 3];
    assert_eq!(TestStruct::do_something(&mut data), 0i32,);
}

#[test]
fn test_let_example() {
    assert!(let_example().is_ok())
}

#[test]
fn test_string_manipulation() {
    assert!(string_manipulation().is_ok())
}

#[test]
fn test_matching_options() {
    assert!(matching_options().is_ok())
}

#[test]
fn test_struct_examples() {
    assert!(struct_examples().is_ok())
}

#[test]
fn test_enum_examples() {
    assert!(enum_examples().is_ok())
}

#[test]
fn test_method_examples() {
    assert!(method_examples().is_ok())
}

#[test]
fn test_trait_examples() {
    assert!(trait_examples().is_ok())
}

#[test]
fn test_derive_macro() {
    assert!(derive_macro().is_ok())
}

#[test]
fn test_mut_examples() {
    assert!(mut_examples().is_ok())
}

#[test]
fn test_mut_ref() {
    assert!(mut_ref().is_ok())
}

#[test]
fn test_scope_examples() {
    assert!(scope_examples().is_ok())
}

#[test]
fn test_lifetime_example() {
    assert!(lifetime_example().is_ok())
}

#[test]
fn test_modules_example() {
    assert!(modules_example().is_ok())
}
