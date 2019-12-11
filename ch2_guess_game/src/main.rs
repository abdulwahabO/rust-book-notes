use std::io::{self, Stdin};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    let mut guess = String::new();

    let secret_num : u32 = rand::thread_rng().gen();

    // Create a handle to the standard input for this process.
    let std_in : Stdin = io::stdin();
    let num_bytes = std_in.read_line(&mut guess);

    let guess: u32 = guess.trim().parse().expect("error occurred");

    match guess.cmp(&secret_num) {
        Ordering::Equal => print!("Guessed ryt"),
        _ => println!("fuck you!! -- wrong answer")
    }


    match num_bytes {
        Ok(num) => println!("number of bytes in the guess {}", num),
        Err(err) => println!("error: {}", err)
    }
    
    println!("You guessed {}", guess);
}
