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
            result => print_bad_guess_message(result)
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
        Ordering::Less => CheckResult::TooSmall,
        Ordering::Greater => CheckResult::TooBig,
        Ordering::Equal => CheckResult::Equal
    }
}

fn print_bad_guess_message(check_result: CheckResult) {
    match check_result {
        CheckResult::TooSmall => println!("Too small"),
        CheckResult::TooBig => println!("Roo big"),
        _ => panic!("Unsupported value passed")
    }
}

enum CheckResult {
    TooSmall,
    TooBig,
    Equal
}