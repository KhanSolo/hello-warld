use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now();    

    // emulate some work
    std::thread::sleep(Duration::from_millis(1100));

    let elapsed = now.elapsed().unwrap();

    println!("{:?}", elapsed);
    //println("");

}