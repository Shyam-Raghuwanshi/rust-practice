use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Silver,
    Black,
    White,
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Brown,
}

#[derive(Debug)]
struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    age: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle {
            manufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 2024,
            color: VehicleColor::Silver,
        };
        return new_vehicle;
    }
}

fn person_fun() -> Person<'static> {
    let p1 = Person {
        first_name: Cell::from("Shyam"),
        last_name: "Raghuwanshi".to_string(),
        age: 19,
    };
    return p1;
}

fn create_person_and_print_fun() {
    let p1 = Person {
        first_name: Cell::from("Shyam"),
        last_name: "Raghuwanshi".to_string(),
        age: 19,
    };

    p1.first_name.set("Shiv");

    println!("{:?}", p1);
}

fn new_vehicle() -> Vehicle {
    let mut vehicle1 = Vehicle {
        manufacturer: "Toyota".to_string(),
        model: "Corolla".to_string(),
        year: 2021,
        color: VehicleColor::Red,
    };

    vehicle1.paint(VehicleColor::Black);

    let mut my_new_vehicle = Vehicle::create_vehicle();
    my_new_vehicle.paint(VehicleColor::Green);
    println!("------> {:?}", my_new_vehicle);
    return vehicle1;
}

pub fn test_create_person() {
    let my_person = person_fun();
    println!(
        "{} {} {}",
        my_person.first_name.get(),
        my_person.last_name,
        my_person.age
    );

    create_person_and_print_fun()
}

// pub fn test_create_vehicle() {
//     let my_vehicle = new_vehicle();
//     println!(
//         "{} {} {}",
//         my_vehicle.manufacturer, my_vehicle.model, my_vehicle.year
//     );
// }

pub fn test_create_vehicle() {
    let my_vehicle: Vehicle = new_vehicle();
    println!("{:?}", my_vehicle);
}

#[derive(Debug)]
#[allow(dead_code)]
struct VehicleTuple(String, String, u16, VehicleColor);

fn new_vehicle_tuple() -> VehicleTuple {
    let vehicle1: VehicleTuple = VehicleTuple(
        "Toyota".to_string(),
        "Corolla".to_string(),
        2021,
        VehicleColor::Red,
    );
    return vehicle1;
}

pub fn test_create_typle_vehicle() {
    let my_vehicle: VehicleTuple = new_vehicle_tuple();
    println!("{:?}", my_vehicle);
}
