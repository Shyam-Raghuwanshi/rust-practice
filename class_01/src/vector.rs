

pub fn test_vector(){
    let mut my_ints:Vec<i32> = Vec::new();

    my_ints.push(33);
    my_ints.push(35);
    my_ints.push(44);
    my_ints.push(31);

    println!("{:?}", my_ints);
    println!("{:?}", my_ints.len());
    println!("{:?}", my_ints.capacity());
    println!("{:?}", my_ints.as_ptr());
    println!("{:?}", my_ints.get(5));
    println!("{:?}", &(&my_ints.as_slice()[0..=3]));

}

pub fn test_string_vector(){

    let names = vec!["shyam".to_string(), "shiv".to_string(), "shyam_raghuonec".to_string()];

    for name in names.as_slice() {
        println!("{:?}", name);
    }

    println!("{:?}", names);

}

#[derive(Debug)]
struct Car{
    manufacturer: String,
    model: String,
}

pub fn test_vec_car(){
    let mut cars: Vec<Car> = vec![Car{manufacturer: "Toyota".to_string(), model: "Corolla".to_string()}, Car{manufacturer: "Honda".to_string(), model: "Civic".to_string()}];

    let mut cars2: Vec<Car> = vec![Car{manufacturer: "Toyota2".to_string(), model: "Corolla2".to_string()}, Car{manufacturer: "Honda2".to_string(), model: "Civic".to_string()}];

    cars.insert(0, Car{manufacturer: "Suzuki".to_string(), model: "Swift".to_string()});

    cars.append(&mut cars2);

    cars.remove(1);
    println!("{:?}", cars);
}