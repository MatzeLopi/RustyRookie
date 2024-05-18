use rand::Rng;
use std::cmp::Ordering;
use std::io;


// Function to generate the nth Fibonacci number
fn fibonacci() {
    println!("Which fibonacci number should be calculated: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("I am not a number");

    let mut a:u64= 0;
    let mut b:u64 = 1;
    let mut c:u64 = 0;

    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }

    println!("The {}th Fibonacci number is: {}", n, c);

}

// Function to convert Celsius to Fahrenheit
fn c_to_f(c: f64) -> f64 {
    return (c * 9.0 / 5.0) + 32.0;
}

// Function to convert Fahrenheit to Celsius
fn f_to_c(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

// Simple temperature converter
fn temperature_converter() {
    println!("Enter the temperature: ");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("I am not a number");

    println!("Enter the conversion you would like to make: ");
    println!("c: Celsius to Fahrenheit");
    println!("f: Fahrenheit to Celsius");

    let mut to = String::new();

    io::stdin()
        .read_line(&mut to)
        .expect("Failed to read line");

    let to: char = to.trim().chars().next().expect("I am not a character");
    
    let to = to.to_ascii_lowercase();

    match to {
        'c' => {
            let c = c_to_f(temp);
            println!("The temperature in Fahrenheit is: {}", c);
        }
        'f' => {
            let f = f_to_c(temp);
            println!("The temperature in Celsius is: {}", f);
        }
        _ => panic!("Invalid conversion"),
    };
}

// Implementation of the first_word function from Chapter 4 of the Rust book
fn first_word(s: &str) -> &str {
    println!("The string is: {}", s);
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

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
            }
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
    println!("2. First Word");
    println!("3. Temperature Converter");
    println!("4. Fibonacci Number");
    println!("0. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("I am not an Integer");

    match choice {
        0 => std::process::exit(0),
        1 => guessing_game(),
        2 => {
            let s = String::from("Hello, World!");
            let word = first_word(&s);
            println!("The first word is: {}", word);
        }
        3 => temperature_converter(),
        4 => fibonacci(),
        _ => println!("Invalid Choice"),
    }
}
