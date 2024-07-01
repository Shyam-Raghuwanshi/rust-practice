// pub enum Option<T> {
//     Some(T),
//     None,
// }

pub fn test_option_type() -> Option<u8> {
    let mut opt1 = None;
    opt1 = Some(22);
    return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1 = None;
    opt1 = Some("hello".to_string());
    return opt1;
}



