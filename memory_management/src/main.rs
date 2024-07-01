const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // this function is story as stack frame this funtion has no variable it means it's stack frame is empty
    println!("Hello, World! Version: {}", VERSION);
    // stack_fun();
    // heap_fn();
    // update_string()
}

fn stack_fun() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("stack fucntion {c}")
}

fn heap_fn() {
    let s1 = String::from("shyam bhai");
    let s2 = String::from("shiv bhai");
    let s1_s2 = format!("{s1} {s2}");

    println!("heep function {s1_s2}")
}

fn update_string() {
    let mut s = String::from("hello shyam");
    println!("Before update: {s}");
    println!(
        "Before update: {}, {}, {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    s.push_str(" from google");
    println!("After update: {s}");
    println!(
        "After update: {}, {}, {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}
