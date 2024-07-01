#[derive(Debug)]
// struct Person<PetType: Pet + NotDangerous> {
//     first_name: String,
//     pet: PetType,
// }

// or
struct Person<PetType, PetType2: Pet + Dangerous>
where
    PetType: Pet + NotDangerous,
{
    first_name: String,
    pet: PetType,
    pet2: PetType2
}

trait Pet {}

trait NotDangerous {}

trait Dangerous {
    
}

#[derive(Debug)]
struct Dog {}

impl Pet for Dog {}
impl NotDangerous for Dog {}

#[derive(Debug)]
struct Cat {}

impl Pet for Cat {}
impl NotDangerous for Cat {}

#[derive(Debug)]
struct Lion {}

impl Pet for Lion {}
impl Dangerous for Lion {}

pub fn create_person() {
    let pet1 = Dog {};
    let lion = Lion {};
    let p1 = Person {
        first_name: "Shyam".to_string(),
        pet: pet1,
        pet2: lion
    };

    let pet2 = Cat {};
    let lion = Lion {};
    let p2 = Person {
        first_name: "Shiv".to_string(),
        pet: pet2,
        pet2: lion
    };

    // let pet2 = Lion {};
    // let p2 = Person {
    //     first_name: "Shiv".to_string(),
    //     pet: pet2,
    // };

    println!("{:?}", p1);
    println!("{:?}", p2);
}
