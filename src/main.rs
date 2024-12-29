use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Rust!");
    println!("Lalalalala!");

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

    let array_with_repeating_values = ["hello!!!!";5];
    println!("{:?}", array_with_repeating_values);

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Guess the numebr!!!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let ur_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad stuff happened that we don't yet know how to deal with");
                continue;
            },
        };

        println!("You guessed: {}", ur_guess);

        match ur_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Your guess appears to be correct!");
                break;
            }
        }
    }
}
