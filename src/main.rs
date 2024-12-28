use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Rust!");
    println!("Lalalalala!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the numebr!!!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        println!("Too bad the number is, in fact, {}.", secret_number);
    }
}
