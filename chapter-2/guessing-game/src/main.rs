use colored::*;
use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game started! Guess the number!");

    let secret = rng().random_range(1..101);
    println!("The secret number is {}", secret);

    loop {
        let mut user_input = String::new();

        println!("Enter your guess");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("{}", "Too small".red())
            }
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Greater => {
                println!("{}", "Too big".red())
            }
        }
    }
}
