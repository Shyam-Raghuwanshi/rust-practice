pub mod funs_main {
    pub fn get_full_name(first_name: &str, last_name: &str) -> String {
        // let full_name = format!("{first_name} {last_name}");
        let full_name = format!("{} {}", first_name, last_name);
        return full_name;
    }
}

#[allow(dead_code)]
fn fool() {
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("{z}");
}
