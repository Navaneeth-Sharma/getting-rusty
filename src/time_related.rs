use std::time::{ Instant };
use chrono::{ Utc };

fn expensive_function(){
    for _i in 0..10000000{
        // do something
    }
}


pub fn time_related_fn() {
    let start = Instant::now();
    // some large function
    expensive_function();
    let duration = start.elapsed();

    println!("{:?}", duration);

    let now = Utc::now();
    println!("{}", now);
}

