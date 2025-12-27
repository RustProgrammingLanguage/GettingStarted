use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // ask for user input
    println!("\nGood evening fellow. Can you guess the number?\n");

    loop {
        println!("Please input your guess: ");

        // process that input & check that the input is in the expected form
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // typecast the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // compare the input to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
