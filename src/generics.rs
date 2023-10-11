// Case: If you want to get largest number, you can implement function with i32 type,
// but if you also want to get largest char then you end up implememnting that funciton for 2 times.
//  to tackle use generics

// Monomorphisation : generic to specific types inside the rust compiler

// here PartialOrd and Copy are the traits, which trasfers the caparision 
// and copy features to this function

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    // Copy trait
    for &item in list {
        // PartialOrd trait
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![5, 100, 87, 89];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let number = largest(&number_list);
    let character = largest(&char_list);

    println!("The largest number is {}", number);
    println!("The largest character is {}", character);
}
