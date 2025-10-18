// use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guessing the number !!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    println!("The secret number is: {secret_number}");

    loop {
        count += 1;
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        std::io::stdin()
            // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we are able to again use guess cause we can shadow variables and aviod creating new ones

        // u means it only supports +ve nums , and i32 would support + and -ve nums
        // trim removes spaces and /n characters added due to the nature of read_line
        // parse converts string to other data type , u32 is amongst them.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("BINGOOOOO!");
                break;
            }
        }

        if count == 5 {
            println!("You lose");
            break;
        }
    }
}
