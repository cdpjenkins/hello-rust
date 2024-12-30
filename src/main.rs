use rand::Rng;
use std::io;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("hello")) {
        say_hello_world();
    } else if args.contains(&String::from("guess")) {
        guessing_game();
    } else if args.contains(&String::from("basic-data-types")) {
        play_around_with_basic_data_types();
    } else if args.contains(&String::from("add")) {
        println!("2+3 = {}", add(2, 3));
    } else if args.contains(&String::from("odd-or-even")) {
        odd_or_event();
    } else if args.contains(&String::from("loops")) {
        for_loop_shizzle();
    } else if args.contains(&String::from("atrings")) {
        fun_with_atrings_and_references();
    } else if args.contains(&String::from("slices")) {
        fun_with_slices();
    } else if args.contains(&String::from("structs")) {
        fun_with_structs();
    }
}

fn fun_with_structs() {
    let rectangle = Rectangle {
        width: 10,
        height: 15
    };

    println!("Rectange is {:?}", rectangle);
    println!("Rectange area is {}", area(&rectangle));

    dbg!(&rectangle);
    dbg!(&rectangle);
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}

fn fun_with_slices() {
    let hello_world_string = String::from("hello world");
    let hello = &hello_world_string[0..5];
    println!("The first 5 characters of hello_world_string are: {}", hello);

    let strings = ["zero", "one", "two", "three", "four", "five"];
    strings.iter().enumerate().for_each(|(i, s)| {
        println!("{}: {}", i, s);
    });
    let slice = &strings[1..=3];
    println!("slide: {:?}", slice);
}

fn fun_with_atrings_and_references() {
    let mut s2 = String::from("hello");
    s2.push_str(" world");
    println!("{}", s2);

    println!("About to do stuff with strings....");

    let mut s3 = String::new();
    s3.push_str("hello ");
    s3.push_str("world");
    s3.push_str("!!!");
    println!("{}", s3);

    let mut s4 = s3.clone();
    s4.push_str(" Wahahahahaha!");
    println!("{}", s4);

    print_me_do(&s2);
    print_me_do(&s3);
    print_me_do(&s4);

    append_stuff_to_string(&mut s4, " moar stuff");
    print_me_do(&s4);
}

fn append_stuff_to_string(p0: &mut String, p1: &str) {
    p0.push_str(p1);
}

fn print_me_do(p0: &String) {
    println!("The string is: {p0}");
    let the_length = p0.len();
    println!("Length is: {the_length}");
}

fn for_loop_shizzle() {
    let emojis = ["🐈", "🐕", "🐖", "🐄", "🦆"];
    for emoji in emojis {
        println!("{} ", emoji);
    }

    for i in 0..10 {
        println!("{} ", i);
    }
}

fn odd_or_event() {
    let num = read_number();
    let result = if num % 2 == 1 {
        "odd"
    } else {
        "even"
    };

    println!("The number is {}.", result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn guessing_game() {
    let secret_number = secret_number_between_1_and_100(1, 100);

    let mut number_of_guesses = 0;
    println!("Guess the number!!!");
    let mut ur_guess: u32 = 0xFFFFFFFF;
    while ur_guess != secret_number {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        number_of_guesses += 1;

        ur_guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad stuff happened that we don't yet know how to deal with");
                continue;
            },
        };

        println!("You guessed: {}", ur_guess);

        if ur_guess < secret_number {
            println!("Too low!");
        } else if ur_guess > secret_number {
            println!("Too high!");
        }
    };

    println!("Your guess of {} appears to be correct!", ur_guess);
    println!("It took you {} guesses", number_of_guesses);
}

fn read_number() -> u32 {

    println!("Enter a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    guess
}

fn secret_number_between_1_and_100(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

fn say_hello_world() {
    println!("Hello, Rust!");
    println!("Lalalalala!");
}

fn play_around_with_basic_data_types() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    let i: u64 = 0x1234567890abcdef;
    println!("{:x}", i);

    let a_tuple = (1isize, '😻', 1.23456);
    println!("{:?}", a_tuple);

    let an_array = [1, 2, 3, 4, 5];
    println!("{:?}", an_array);

    let string_array = ["Hello", "World", "!!"];
    println!("{:?}", string_array);
    println!("{} {}{}", string_array[0], string_array[1], string_array[2]);

    let array_with_repeating_values = ["hello!!!!"; 5];
    println!("{:?}", array_with_repeating_values);
}
