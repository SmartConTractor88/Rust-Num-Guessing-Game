use std::io; // input/output from standard library
use std::cmp::Ordering; // for comparing values
use rand::Rng;

fn main() {

 println!("---- Guess the Number Between 1 and 100 ---");

 let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    
loop {
    println!("Enter your guess: ");
 let mut guess = String::new();
 io::stdin()
 .read_line(&mut guess)
 .expect("Failed to read line");

 let guess: u32 = guess
 .trim()// remove whitespace
 .parse() // convert to the specified type (u32)
 .expect("Please type a number!");

 println!("You guessed: {}", guess);

 match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
        }
    }
    println!("The secret number was: {}", secret_number);
}
}