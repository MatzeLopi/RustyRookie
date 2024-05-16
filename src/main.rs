use rand::Rng;
use std::io;
use std::cmp::Ordering;

// Implementation of the guessing game from Chapter 2 of the Rust book
fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }, 
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    println!("Which function would you like to run?");
    println!("1. Guessing Game");
    println!("2. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("I am not an Integer");

    match choice {
        1 => guessing_game(),
        2 => std::process::exit(0),
        _ => println!("Invalid Choice"),
    }
}
