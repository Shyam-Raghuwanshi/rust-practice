pub fn test_match_int(age: i8) {
    match age {
        0 => {
            println!("You are a new born")
        }
        1..=12 => {
            println!("You are a child")
        }

        13..=19 => {
            println!("You are a teenager")
        }
        20..=35 => {
            println!("You are a young adult")
        }
        36..=60 => {
            println!("You are a adult")
        }, 

        99 | 100 =>{
            println!("You are a centenarian")
        },
        61.. =>{
            println!("You are a senior citizen")
        },
        _ => {
            println!("You are a senior citizen")
        }
    }
}
