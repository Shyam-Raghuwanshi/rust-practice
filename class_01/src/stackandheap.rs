pub fn main() {
    // stack_fn();
    // heap_fn();
    // update_string();
    // ownership()
    ownershipfun2()
}

fn stack_fn() {
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("{}", z);
}

fn heap_fn() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);
}

fn update_string() {
    let mut s1 = String::from("hi");
    println!("{}", s1);
    println!(
        "capacity {}, Length {}, pointer {:p}",
        s1.capacity(),
        s1.len(),
        s1.as_ptr()
    );

    s1.push_str(" there");
    println!(
        "capacity {}, Length {}, pointer {:p}",
        s1.capacity(),
        s1.len(),
        s1.as_ptr()
    );
    println!("{}", s1);
}

fn ownership() {
    let s1 = String::from("hi");
    println!(
        "capacity {}, Length {}, pointer {:p}",
        s1.capacity(),
        s1.len(),
        s1.as_ptr()
    );
    let s2 = s1;
    println!(
        "capacity {}, Length {}, pointer {:p}",
        s2.capacity(),
        s2.len(),
        s2.as_ptr()
    );

    // this will gives an ownership error
    // println!("{}", s1);
}

// fn ownershipfun2() {
//     let my_str = String::from("hi");
//     // takes_ownership(my_str);
//     takes_ownership(my_str.clone());
//     println!("{}", my_str);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string)
// }

fn ownershipfun2() {
    let mut my_str = String::from("hi");
    // takes_ownership(my_str);
    my_str = takes_ownership(my_str);
    println!("{}", my_str);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}
