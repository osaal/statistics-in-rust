use crate::common::TestError;
mod common;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
trait ExampleFnSig {
    // ANCHOR: example_mean
    fn mean(x: Vec<u32>) -> i32;
    // ANCHOR_END: example_mean

    // ANCHOR: example_mut_fn
    fn do_something(x: &mut Vec<u32>) -> i32;
    // ANCHOR_END: example_mut_fn
}

struct MyFunctions {}

impl ExampleFnSig for MyFunctions {
    fn mean(_x: Vec<u32>) -> i32 {
        0
    }

    fn do_something(_x: &mut Vec<u32>) -> i32 {
        todo!()
    }
}

#[inline(always)]
#[allow(dead_code)]
fn use_do_something() -> Result<(), TestError> {
    // ANCHOR: example_mutability
    let mut data = vec![1, 2, 3];
    let _res_1 = MyFunctions::do_something(&mut data);
    let _res_2 = MyFunctions::do_something(&mut data);
    // ANCHOR_END: example_mutability
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn let_example() -> Result<(), TestError> {
    // ANCHOR: let_example
    let x = 5;
    // ANCHOR_END: let_example
    let _ = x;
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn string_manipulation() -> Result<(), TestError> {
    // ANCHOR: example_string_manipulation
    let hello = String::from("Hello");
    let world = String::from("world!");

    let mut mystring = hello.clone();
    mystring.push_str(", ");
    mystring.push_str(&world);

    let _newstring = hello + ", " + &world;
    // ANCHOR_END: example_string_manipulation
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn option_def() -> Result<(), TestError> {
    // ANCHOR: option_def
    pub enum Option<T> {
        // No value.
        None,
        // Some value of type `T`.
        Some(T),
    }
    // ANCHOR_END: option_def
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn matching_options() -> Result<(), TestError> {
    // ANCHOR: matching_options
    let value = Some(5);
    match value {
        Some(val) => println!("Found a value of {}", val),
        None => println!("Nope, nothing here."),
    }
    // ANCHOR_END: matching_options
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn result_def() -> Result<(), TestError> {
    // ANCHOR: result_def
    pub enum Result<T, E> {
        // Contains the success value
        Ok(T),
        // Contains the error value
        Err(E),
    }
    // ANCHOR_END: result_def
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn struct_examples() -> Result<(), TestError> {
    // ANCHOR: struct_examples
    struct UnitStruct;
    struct TupleStruct(u32, String, f64);
    struct EmptyStruct {}
    struct FilledStruct {
        field: u32,
    }
    // ANCHOR_END: struct_examples
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn enum_examples() -> Result<(), TestError> {
    // ANCHOR: enum_examples
    enum EmptyEnum {}
    enum SimpleEnum {
        First,
        Second,
        Third,
    }
    enum ComplexEnum {
        Nothing,
        Something(String),
        StructData {
            is_valuable: bool,
            value: Option<isize>,
        },
    }
    // ANCHOR_END: enum_examples
    // ANCHOR: model_enum
    enum ExtractionMethod {
        Frequentist,
        Bayesian,
    }
    // ANCHOR_END: model_enum
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn fun_examples() -> Result<(), TestError> {
    // ANCHOR: basic_fun
    fn x(_a: i32, _b: bool) -> Result<(), i32> {
        Ok(())
    }
    // ANCHOR_END: basic_fun
    // ANCHOR: fun_usage
    let result = x(5, true);
    // ANCHOR_END: fun_usage
    let _ = result;
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn method_examples() -> Result<(), TestError> {
    // ANCHOR: naive_method
    struct MyModelResults;
    fn extract_fit_indices(_model: &MyModelResults) -> Vec<f64> {
        // Extract the stuff from `_model`
        vec![0., 0., 0.]
    }
    // ANCHOR_END: naive_method

    // ANCHOR: method_ex
    impl MyModelResults {
        fn extract_fit_indices(&self) -> Vec<f64> {
            // Extract the stuff from `self`
            vec![0., 0., 0.]
        }
    }
    // ANCHOR_END: method_ex

    // ANCHOR: using_methods
    let model_result = MyModelResults {};

    let indices_fun = extract_fit_indices(&model_result);
    let indices_method = model_result.extract_fit_indices();
    // ANCHOR_END: using_methods
    let _ = indices_fun;
    let _ = indices_method;

    // ANCHOR: method_chaining
    struct MyObject {}
    impl MyObject {
        fn first_method(self) -> Self {
            // Imagine something cool happening here...
            self
        }
        fn second_method(self) -> Self {
            // Imagine an even cooler, but distinct, thing here...
            self
        }
    }

    let x = MyObject {};
    x.first_method().second_method().first_method();
    // ANCHOR_END: method_chaining
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn trait_examples() -> Result<(), TestError> {
    struct MyModelResults;
    // ANCHOR: trait_examples
    trait Model {
        fn extract_fit_indices(&self) -> Vec<f64>;
    }
    impl Model for MyModelResults {
        fn extract_fit_indices(&self) -> Vec<f64> {
            // Define how fit indices are extracted from `MyModelObject`
            vec![0., 0., 0.]
        }
    }
    // ANCHOR_END: trait_examples
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn derive_macro() -> Result<(), TestError> {
    // ANCHOR: derive_macro
    #[derive(Debug, Clone, PartialEq)]
    struct MyModelResults;

    let x = MyModelResults;
    dbg!(&x);
    let y = x.clone();
    assert!(x == y);
    // ANCHOR_END: derive_macro
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn ref_examples() -> Result<(), TestError> {
    // ANCHOR: ref_examples
    let x: i32 = 5;
    let y: &i32 = &x;
    let z: i32 = *y;
    // ANCHOR_END: ref_examples
    let _ = z;
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn mut_examples() -> Result<(), TestError> {
    // ANCHOR: mut_examples
    let mut x = 5;
    x = 7;
    x += 2;
    // ANCHOR_END: mut_examples
    let _ = x;
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn mut_ref() -> Result<(), TestError> {
    // ANCHOR: mut_ref
    trait MyTrait {
        fn first_method(&self) -> ();
        fn second_method(&mut self) -> ();
    }
    // ANCHOR_END: mut_ref
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn scope_examples() -> Result<(), TestError> {
    // ANCHOR: scope_examples
    let x = 5;
    {
        let x = 7;
    }
    println!("{x}");
    // ANCHOR_END: scope_examples
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn lifetime_example() -> Result<(), TestError> {
    // ANCHOR: lifetime_example
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() { x } else { y }
    }
    // ANCHOR_END: lifetime_example
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn modules_example() -> Result<(), TestError> {
    // ANCHOR: modules_example
    mod inner {
        pub const X: i32 = 5;
        pub fn myfun() -> i32 {
            5
        }
    }

    let x = inner::X;

    fn myfun() -> i32 {
        7
    }

    let y = inner::myfun();
    // ANCHOR_END: modules_example
    let _ = x;
    let _ = y;
    Ok(())
}
