fn main() {
    // implements lambda (python) kinda a function here
    // also, match case (switch case) is used here
    // also, closures are used here
    let fizbuzz = |x| match (x % 3, x % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        (_, _) => println!("{}", x),
    };

    // this is similar to map
    (1..24).into_iter().for_each(fizbuzz);
}
