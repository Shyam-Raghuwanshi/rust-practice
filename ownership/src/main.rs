// fn main() {
//     let s1 = String::from("Hello shyam");
//     let s2 = s1;

//     println!("{s2}")
// }

// fn main() {
//     let mut s1 = String::from("Hello shyam");
//     s1 = takes_ownership(s1);

//     println!("{s1}")
// }

// fn takes_ownership(some_string: String)-> String{
//     println!("{some_string}");
//     return some_string;
// }

// fn main(){
//     let s1 = String::from("Hello shyam");
//     let s2 = &s1;

//     println!("{s1} {s2}")
// }
fn main(){
    let mut s1 = String::from("Hello");
    let s2 = &mut  s1;
    update(&mut s1); 
    println!("{s1}");
    println!("{s2}")
}

fn update(s: &mut String){
    s.push_str(" world")
}
