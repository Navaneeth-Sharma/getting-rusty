extern crate chrono;

mod time_related;
pub use time_related::time_related_fn;

// use std::env;
// use std::fs;
// use std::process;

// struct Config {
//     query: String,
//     file: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &str>{

//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

fn main() {
    //     let args: Vec<String> = env::args().collect();

    //     let config = Config::new(&args).unwrap_or_else(|err|{process::exit(1)
    //     })

    //     let contents = fs::read_to_string(config.filename).expect("Something went wrong!");

    //     println!("Contents: {:?}", contents);

    //    time_related_fn();
}
