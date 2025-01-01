mod rust_things;

use crate::rust_things::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "hello" => say_hello_world(),
        "guess" => guessing_game(),
        "basic-data-types" => play_around_with_basic_data_types(),
        "add" => println!("2+3 = {}", add(2, 3)),
        "odd-or-even" => odd_or_event(),
        "loops" => for_loop_shizzle(),
        "strings" => fun_with_atrings_and_references(),
        "structs" => fun_with_structs(),
        "slices" => fun_with_slices(),
        "option" => fun_with_option(),
        "vectors" => vectors(),
        "hashmaps" => hashmaps(),
        _ => println!("Unknown command"),
    }
}

fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Brian"), 10);
    scores.insert(String::from("Colin"), 20);

    let brian_score = scores.get(&String::from("Brian"));

    println!("Brian has scored {:?}", brian_score);
}

fn vectors() {
    let mut vec: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", vec);

    vec.push(4);
    vec.push(-1);

    println!("{:?}", vec);

    vec.iter().for_each(|x| println!("{}", x));

    for x in &mut vec {
        *x += 1;
    }

    println!("{:?}", vec);
}
