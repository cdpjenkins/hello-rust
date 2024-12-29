use rand::Rng;
use std::io;

fn main() {
    say_hello_world();
    play_around_with_basic_data_types();
    println!("2+3 = {}", add(2, 3));
    guessing_game();
    // odd_or_event();
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
