mod rust_things;
mod fun_with_ints;
mod grep;

use crate::rust_things::*;
use crate::fun_with_ints::*;
use std::env;
use std::io::Read;
use crate::grep::minigrep;

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
        "panic" => fun_with_panic(),
        "result" => fun_with_result(),
        "generics" => fun_with_generics(),
        "fun-with-ints" => have_fun_with_ints(),
        "grep" => minigrep(),
        _ => println!("Unknown command"),
    }
}

fn fun_with_result() {
    use std::fs::File;

    // let file = File::open("hello.txt")
    //     .unwrap_or_else(|error| {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     });

    let mut file = File::open("hello.txt").unwrap();

    dbg!(&file);


    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("The contents of hello.txt are: {}", contents);
}

fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Brian"), 10);
    scores.insert(String::from("Colin"), 20);

    println!("Brian has scored {:?}", scores.get(&String::from("Brian")));

    // overwrite what is already there
    scores.insert(String::from("Brian"), 50);
    println!("Brian has now scored {:?}", scores.get(&String::from("Brian")));

    // insert if not already present
    scores.entry(String::from("Marmaduke")).or_insert(100);
    println!("Marmaduke has now scored {:?}", scores.get(&String::from("Marmaduke")));

    // update what is already there
    let this_score = scores.entry(String::from("Colin")).or_insert(0);
    *this_score += 1;


    println!();
    println!("Look out, updated scores coming up!");
    for (dude, score) in scores {
        println!("{} has scored {}", dude, score);
    }

    // honestly, I'm still getting used to all this stuff
    // especially move/copy semantics and ownership and "oops, calling a method on that object
    // just moved it and now the object can never be used again"

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
