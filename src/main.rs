use std::io; // input/output from standard library
use rand::Rng;

fn main() {

 println!("---- Guess the Number Between 1 and 100 ---");

 let secret_number = rand::thread_rng().gen_range(1..=100);
    
 println!("Please input your guess.");
 let mut guess = String::new();
 io::stdin()
 .read_line(&mut guess)
 .expect("Failed to read line");
 print!("You guessed: {}", guess);
 println!("The secret number was {secret_number}");

}