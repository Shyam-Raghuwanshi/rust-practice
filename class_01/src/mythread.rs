use std::thread::spawn;

pub fn mythread() {
    // let mut x = 0u128;
    // for i in 1..=500_000_000_000 {
    //     x += i;
    // }

    let my_fn = ||{
        let mut x = 0u128;
        for i in 1..=500_000_000_000 {
            x += i;
        }
        println!("{}", x)
    };

    let handle = spawn(my_fn);

    handle.join();

}