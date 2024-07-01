pub fn test_closures() {
    let add = |x, y| {
        println!("Hello from closures.rs {}", x + y);
        x + y
    };

    let result = add(44, 4433);

    let print_result = || println!("Result is: {}", result);

    print_result();
}

const OUTER_VALUE: i8 = 3;

pub fn test_closures2(i: i8) -> i8 {
    return i + OUTER_VALUE;
}

// this should give an error

// pub fn test_closures3(i:i8){
//     let outer_value:i8 = 3;
//     fn inner_function(i:i8)->i8{
//         return i + outer_value;
//     }
// }

pub fn test_closures4(i: i8) -> i8 {
    let outer_value: i8 = 3;

    let inner_function = |x: i8| {
        return x + outer_value;
    };

    return inner_function(i);
}

pub fn test_closures5(i: i8) {
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

// #[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
}

pub fn test_closures6() {
    let mut p1 = Person {
        first_name: "Shyam".to_string(),
        last_name: "Raghuwanshi".to_string(),
    };

    let mut change_name = |name: &str| p1.first_name = name.to_string();

    change_name("shyam");
    change_name("shiv");
    println!("{:?}", p1.first_name);

}
