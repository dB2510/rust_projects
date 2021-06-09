use std::io;
// "use" keyword is used to import a library. Here we need io library
// which comes from the standard (std) library.
use rand::Rng; // Rng trait defines methods that random number generators will implement

use std::cmp::Ordering;

fn main() {
    println!("Guessing the number!");

    // The random number generator here is local to the current thread of execution
    // and seeded by the operating system.
    let secret_number = rand::thread_rng().gen_range(1..101); // gen_range is defined by the Rng trait. It takes range as (start..end) or (start..=end)

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default
        // "mut" keyword is used to make a variable mutable
        let mut guess = String::new();

        // Like variables, references are also immutable by default
        io::stdin()
            .read_line(&mut guess) // & indicates arguments is a reference
            .expect("Failed to read line"); // For error handing. read_line() return a
                                            // type called io::Result which is an enum as "ok" or "err". expect executes if it encounters "err".

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // match is used to move from crashing on an error to handling an error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // {} is used a placeholder for variables

        match guess.cmp(&secret_number) {
            // match expression is made of arms
            // The following 3 are 3 different arms
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
