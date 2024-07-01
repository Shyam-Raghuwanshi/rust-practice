use std::collections::HashMap;


pub fn test_hashmap(){
    let mut my_hashmap: HashMap<String, u8> = HashMap::<String, u8>::new();

    my_hashmap.insert("Shyam".to_string(), 19);
    my_hashmap.insert("Vishal".to_string(), 19);
    my_hashmap.insert("Tarun".to_string(), 19);
    my_hashmap.insert("Shiv".to_string(), 19);

    println!("{:#?}", my_hashmap);
} 