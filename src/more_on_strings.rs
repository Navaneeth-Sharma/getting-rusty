fn example_1() {
    // Rust by practice
    // ways to concatenate the strings
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += &"!".to_string();

    println!("{}", s);
}

// Fix errors without removing any line
fn example_2() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2.clone();
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

fn main() {
    example_1();
    example_2();
}
