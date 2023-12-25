use std::cmp::Ordering;

pub fn learn() {
    // Parsing
    const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;

    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;

    println!("I'm {} and {}", age, ONE_MIL);

    // Get max value
    println!("Max u32 : {}", u32::MAX);
    println!("Max f64 : {}", f64::MAX);

    // rand
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random number : {} ", random_num);

    let age2 = 8;
    match age2 {
        1..=18 => print!("Important Birthday!"),
        21 | 50 => print!("Important Birthday"),
        _ => print!("Not important birthday"),
    }

    let voting_age = 18;

    match age2.cmp(&voting_age) {
        Ordering::Less => println!("cant vote!"),
        Ordering::Greater => println!("can vote!"),
        Ordering::Equal => println!("gained! can vote!"),
    }

    // strings
    // let mut str1 = String::new();
    // str1.push('A');
    // str1.push_str("string");
    // for word in str1.split_whitespace() {
    //     println!("{}", word);
    // }
    // let str2 = str1.replace("A", "B");

    // let int_u8 = 5;
    // let int2_u8 = 4;
    // let int3_u32 = (int_u8 as u32) + (int_u8 as u32);

    
}
