// fn main() {
//     // let mut x:u8 = 10;

//     // for _i in 0..1000{
//     //     x = x*40;
//     // }

//     // let bool = true;
//     // let bool2 = false;
//     // println!(" {bool} && {bool2}");

//     let gretting = String::from("helloo world");
//     let char5 = gretting.chars().nth(4);
//     // println!("{str}")
//     // println!("{:?}", char5)
//     // println!("{}", char5.unwrap())

//     match char5 {
//         Some(x) => println!("{x}"),
//         None => println!("no charatar at this index")
//     }
// }

// fn main(){
//     // if true{
//     //     println!("hello shyam bhai ");
//     // }
// }

// fn main() {
//     // for i in 0..100{
//     //     println!("{i}")
//     // }

//     let sentence = String::from("shyam bhai");

//     let first_word = get_fist_word(sentence);

//     println!("{first_word}");
// }

// fn get_fist_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());

//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

// fn main() {
//     let gretting = String::from("hi");
//     let char1 = gretting.chars().nth(10);

//     match char1 {
//         Some(x) => println!("{x}"),
//         None => println!("no charatar at this index"),
//     }

//     // println!("{}", char1.unwrap());
//     // print!("{:?}", char1);
// }

// fn main() {
//     let sentence = String::from("hello world");
//     // print!("{}", sentence.len());
//     let first_word = get_fist_word(sentence);
//     println!("{first_word}");
// }

// fn get_fist_word(sentence: String) -> String {
//     let mut ans = String::from("");

//     for i in 1..sentence.len()+1 {
//         let char = sentence.chars().nth(sentence.len()-i);
//         match char {
//             Some(x) => ans.push_str(x.to_string().as_str()),
//             None => break,
//         }
//     }
// for char in sentence.chars() {
//     ans.push_str(char.to_string().as_str());

//     if char == ' ' {
//         break;
//     }
// }
//     return ans;
// }

// fn main() {
// let x: ()= ();

// print!("{:?}", x)

// let x:i8 = 255;

// let x: f32 = 255.0;
// let y: u8 = x as u8 - 5;

// let mystr = "A";

// let name = ("shyam", "raghuwanshi", 40 as i8);
// let (first, last, age) = name;
// println!("{first} {last} {age}");
// print!("{:?}", name.1)

// let ages = [44, 55, 555, 32];
// let ages = ["shyam", "aksdf", "skdfj"];
// let ages:[i8; 6] = [44, 5, 24, 32, 33 , 55];
// let new_ages = &ages[0..ages.len()];
// let new_ages = &ages[0..=5];
// print!("{:?}", ages[1])
// print!("{:?}", new_ages)
// }

// pub mod funs;
// fn main() {
//     let output = funs::funs_main::get_full_name("shyam", "raghuwanshi");
//     println!("{output}");
// }

// use std::u8;

// fn main() {
//     let age_to_drive = 18u8;

//     println!("Enter your age: ");
//     let my_input = &mut String::from("");
//     std::io::stdin().read_line(my_input).unwrap();

//     let age = my_input.replace("\n", "").parse::<u8>().unwrap();

//     if age >= age_to_drive {
//         println!("You can drive");
//     } else {
//         println!("You can't drive");
//     }
// }

// fn main() {
//     loop{
//         println!("hi shyam")
//     }
// }

// #[allow(dead_code)]
// fn test(){
//     println!("hi shyam")
// }

// pub mod closures;
// fn main() {
//     // closures::test_closures();
//     // let res = closures::test_closures2(9);
//     // println!("{}", res);
//     // let res4 = closures::test_closures4(44);
//     // println!("{}", res4);
//     // closures::test_closures5(55);
//     // closures::test_closures6()
// }

// pub mod stackandheap;
// fn main(){
//     stackandheap::main();
// }

// pub mod matchexpressionsandpattersns;
// fn main() {
//     matchexpressionsandpattersns::test_match_int(127);
// }

// pub mod enums;
// fn main(){
//     let res = enums::test_option_type();
//     println!("{0}", res.unwrap());
//     let res2 = enums::test_option_string();
//     println!("{0}", res2.unwrap())
// }

// pub mod structs;
// use crate::structs::test_create_typle_vehicle;


// fn main() {
//     structs::test_create_person();
//     structs::test_create_vehicle();
//     test_create_typle_vehicle();
// }


// pub mod traits;

// fn main() {
//     traits::create_person();
// }


pub mod vector;
pub mod myhashmap;
pub mod myiterator;
pub mod timemodule;
pub mod mythread;

fn main() {
    // vector::test_vector();
    // vector::test_string_vector();
    // vector::test_vec_car()
    // myhashmap::test_hashmap();
    // myiterator::test_iterator();
    // timemodule::test_time();
    mythread::mythread()
}