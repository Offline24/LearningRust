extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number game");

    let secret = random_from_1_to_100();

    loop {
        match check(secret, take_guess()) {
            CheckResult::Equal => {
                println!("You win!");
                break
            }
            CheckResult::Error(error) => print_bad_guess_message(error)
        }
    }
}

fn random_from_1_to_100() -> i32 { 
    rand::thread_rng().gen_range(1, 101)
}

fn take_guess() -> i32 {
    println!("Input your guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Type number");
            take_guess()
        }
    }
}

fn check(secret: i32, guess: i32) -> CheckResult {
    match guess.cmp(&secret) {
        Ordering::Less => CheckResult::Error(CheckError::TooSmall),
        Ordering::Greater => CheckResult::Error(CheckError::TooBig),
        Ordering::Equal => CheckResult::Equal
    }
}

fn print_bad_guess_message(check_error: CheckError) {
    match check_error {
        CheckError::TooSmall => println!("Too small"),
        CheckError::TooBig => println!("Roo big"),
    }
}

enum CheckResult {
    Error(CheckError),
    Equal
}

enum CheckError {
    TooSmall,
    TooBig
}