use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {}

fn check(a: i32) -> Result<i32> {
    if a == 42 {
        Ok(a)
    } else {
        Err(Box::new(Error {
            message: format!("{} is not 42", a),
        }))
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("Correct {}", n),
        Err(e) => println!("Error: {}", e)
    }
}


// this is too long, so use libraries


fn main(){
    print(check(42));
    print(check(100));
}