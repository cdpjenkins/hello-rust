use std::{env, io};
use rand::Rng;


#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl <T: std::fmt::Debug> Point<T> {
    fn print_me_do(&self) {
        println!("Point({:?},{:?}", self.x, self.y);
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_so_far = &list[0];

    for item in list {
        if item > largest_so_far {
            largest_so_far = item;
        }
    }

    return largest_so_far;
}

pub fn fun_with_generics() {
    println!("generics to come...");

    let list = vec![1,2,3,4,5,6,7,8,7,6,5,4,3,2,1];

    let result = largest(&list);

    println!("the biggest number is: {result}.");

    let p1 = Point { x: 10, y: 20};
    println!("{:?}", p1);

    let p2 = Point { x: 1.23, y: 4.56};
    println!("{:?}", p2);
    p2.print_me_do();
}

pub fn fun_with_option()  {
    let args: Vec<String> = env::args().collect();

    let number: Option<i32> = match args[2].parse() {
        Ok(num) => Some(num),
        Err(_) => None
    };

    match number {
        Some(num) => println!("The number is: {}.", num),
        None => println!("There is no number. At all.")
    };

    if let Some(num) = number {
        println!("The number is still: {}.", num);
    } else {
        println!("There is *still* no number.");
    }
}

pub fn fun_with_structs() {
    let rectangle1 = Rectangle {
        width: 10,
        height: 15
    };

    let little_square = Rectangle::square(5);

    let big_rectangle = Rectangle {
        width: 100,
        height: 100
    };

    println!("rectange1 is {:?}", rectangle1);
    println!("rectange1 area is {}", rectangle1.area());

    if rectangle1.can_hold(&little_square) {
        println!("rectangle1 can hold little_square");
    } else {
        println!("rectangle1 can't hold little_square");
    }

    if rectangle1.can_hold(&big_rectangle) {
        println!("rectangle1 can hold big_rectangle");
    } else {
        println!("rectangle1 can't hold big_rectangle");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

pub fn fun_with_slices() {
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

pub fn fun_with_atrings_and_references() {
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

pub fn append_stuff_to_string(p0: &mut String, p1: &str) {
    p0.push_str(p1);
}

pub fn print_me_do(p0: &String) {
    println!("The string is: {p0}");
    let the_length = p0.len();
    println!("Length is: {the_length}");
}

pub fn for_loop_shizzle() {
    let emojis = ["🐈", "🐕", "🐖", "🐄", "🦆"];
    for emoji in emojis {
        println!("{} ", emoji);
    }

    for i in 0..10 {
        println!("{} ", i);
    }
}

pub fn odd_or_event() {
    let num = read_number();
    let result = if num % 2 == 1 {
        "odd"
    } else {
        "even"
    };

    println!("The number is {}.", result);
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn guessing_game() {
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

pub fn read_number() -> u32 {

    println!("Enter a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    guess
}

pub fn secret_number_between_1_and_100(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

pub fn say_hello_world() {
    println!("Hello, Rust!");
    println!("Lalalalala!");
}

pub fn play_around_with_basic_data_types() {
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

pub fn fun_with_panic() {
    let v = vec![1, 2, 3];
    v[99];
    // panic!("Oh noes!");
}