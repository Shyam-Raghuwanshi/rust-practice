use std::time::{Duration, Instant};

extern crate chrono;


pub fn test_time() {
    let dur1 = Duration::from_secs(5);
    // println!("{:?}", dur1.as_micros());
    // println!("{:?}", dur1.as_millis());
    // println!("{:?}", dur1.as_nanos());
    // println!("{:?}", dur1.as_secs());

    let dur2 = Duration::from_millis(5000);

    // let res = dur1.clone().sub(dur2);
    // println!("{:?}", res.as_millis());

    // let dur3 = dur1.checked_sub((dur2));
    // println!("{:?}", dur3.unwrap_or_default().as_millis());


    // let c1 = Instant::now();

    // std::thread::sleep(Duration::from_millis(200));

    // println!("{:?}", c1.elapsed().as_micros());

    //chorono ------------------------------------------------

    let utc_now = chrono::Utc::now();

    println!("{}", utc_now.format("%Y-%m-%d %H:%M:%S"));

    let local_now = chrono::Local::now();
    println!("{}", local_now.format("%Y-%m-%d %H:%M:%S"));


}
