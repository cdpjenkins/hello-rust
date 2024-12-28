use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Rust!");
    println!("Lalalalala!");

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
