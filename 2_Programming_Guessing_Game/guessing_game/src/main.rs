use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input an integer number :D");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess = guess.trim().parse::<i32>().expect("Please type an integer number"); porque não fazer deste jeito ?
        /*
        ok com o match tem o tratamento do erro mas ainda dá para usar o parse::<i32>() porque não usar ?
            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        */ 
        let guess= match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed the number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!\n "),
            Ordering::Greater => println!("Too Big!\n "),
            //Ordering::Equal => return println!("Congratulations, you Win :)"), não era só colocar um return e pronto ?
            Ordering::Equal => {
                () = { //não precisava desse ()={} era só pra testar mesmo, nem sabia que atinha anonymous function no Rust :/
                    println!("Congratulations, you Win :)");
                    break;
                }
            }
        }
    }
}
