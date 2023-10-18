extern crate chrono;

mod time_related;
pub use time_related::time_related_fn;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query: &String = &args[1];
    let filename: &String = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong!");

    println!("Contents: {:?}", contents);

   time_related_fn(); 
}
