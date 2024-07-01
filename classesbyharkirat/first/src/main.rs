// fn main() {
//     println!("Hello, shyam!");
// }

// fn main(){
//     let x:i32 = 5;
//     println!("{x}");
// }

// fn main() {
//     let mut num: i8 = 124;

//     for i in 0..100000 {
//         num = num + 1;
//         println!("{num}")
//     }
// }

// fn main() {
//     let is_male = false;
//     let is_above_18 = true;

//     if is_male {
//         println!("you are male")
//     } else if is_above_18 {
//         println!("you are above 18")
//     } else {
//         println!("you are not eligible")
//     }
// }

// fn main() {
//     let gretting = String::from("Hello, shyam!");
//     println!("{gretting}");
//     println!("{}", gretting.chars().nth(433));
// }


fn main(){
    let x = 54;
    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    }
    else{
        println!("{} is odd", x);
    }
}

fn is_even(x:i32)->bool{
    return x%2==0;
}