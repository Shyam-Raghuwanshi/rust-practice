pub fn test_iterator() {
    let mut fruits = vec!["Apple", "Banana", "Cherry", "Dates", "Elderberry"];
    let mut nuts = vec!["Almond", "Brazil", "Cashew", "Dandelion", "Elderberry"];
    let mut fruits_iter = fruits.iter();

    // for fruit in fruits_iter.clone() {
    //     println!("{}", fruit)
    // }

    // let n = fruits_iter.next();

    // println!("{:?}", n);
    // let n = fruits_iter.next();
    // println!("{:?}", n);
    // let n = fruits_iter.next();
    // println!("{:?}", n);
    // let n = fruits_iter.next();
    // println!("{:?}", n);
    // let n = fruits_iter.next();
    // println!("{:?}", n);
    // let n = fruits_iter.next();
    // println!("{:?}", n);

    // let aggregate_foods = fruits.iter().chain(&nuts);

    // let coll: Vec<&&str> = aggregate_foods.clone().collect();
    // println!("{:?}", coll);
    // println!("{:?}", aggregate_foods);
    // for food in aggregate_foods {
    //     println!("{}", food);
    // }

    let fr_map = fruits.iter().map(|e| String::from(*e));

    let new_fr = fr_map.map(|mut e| {
        e.push_str(" fruit");
        return e;
    });

    new_fr.clone().for_each(|e| println!("{}", e));

    // for f in fr_map {
    //     println!("{}", f);
    // }

    let last_one = new_fr.clone().last();
    println!("{:?}", last_one.unwrap());


    let step_by = new_fr.clone().step_by(2);

    for f in step_by {
        println!("{}", f);
    }

    let first_names = vec!["Shyam", "Vishal", "Tarun", "Shiv"];

    let last_names = vec!["Sharma", "Gupta", "Kumar", "Singh"];

    let firts_name_strings= first_names.iter().map(|e| String::from(*e));


    let last_name_strings= last_names.iter().map(|e| String::from(*e));

    let full_names = firts_name_strings.zip(last_name_strings);    

    // for (f, l) in full_names {
    //     println!("{} {}", f, l);
    // }

    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));

    
    for (index, value) in full_names.enumerate(){
        println!("{} {} {}", index, value.0, value.1);
    }


}
