fn reverse_string(string_: String) -> String {
    let mut reversed_str = String::new();

    for i in (0..string_.len()).rev() {
        let c = string_.chars().nth(i);
        if let Some(ch) = c {
            reversed_str.push(ch);
        }
    }

    reversed_str
}

fn main() {
    let string = String::from("cool is cool");

    let reversed_string = reverse_string(string);

    println!("{:?}", reversed_string);
}
