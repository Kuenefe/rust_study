use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let err_msg_input: &str = "Failed to read input";
    let err_msg_num: &str = "Not a valid number!";

    loop {
        println!("Input your number for guessing:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(err_msg_input);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("{}", err_msg_num);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is too small!"),
            Ordering::Greater => println!("Your number is too big!"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
        }

    }
}
