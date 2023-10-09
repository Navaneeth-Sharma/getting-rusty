use std::collections::HashMap;

// external lib
// #[macro_use] extern crate maplit;

fn main() {
    // to define directly
    let _dict: HashMap<_, _> = vec![("name", "cool"), ("job", "yo")].into_iter().collect();

    let mut mutable = HashMap::new();
    mutable.insert("one", 1);
    mutable.insert("two", 2);

    println!("{:?}", mutable.get("one"));

    // you can use & to get from directly from address
    for (k, v) in &mutable {
        println!("{}: {}", k, v);
    }

    println!("------");

    mutable.remove("one");

    println!("{:?}", mutable.get("one"));

    // or u can use iter()
    // .keys() or .values() to loop over the keys or values
    for (k, v) in mutable.iter() {
        println!("{}: {}", k, v);
    }

    // let map = hashmap!{
    //     "a" => 1,
    //     "b" => 2
    // }
}
