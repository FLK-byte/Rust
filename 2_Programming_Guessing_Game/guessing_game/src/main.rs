use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input a number :D");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed the number: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small! "),
        Ordering::Greater => println!("Too Big! "),
        Ordering::Equal => println!("You Win")
    }

}