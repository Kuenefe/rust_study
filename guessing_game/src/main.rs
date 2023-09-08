use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Input your number for guessing:");

    let mut guess = String::new();

    let err_msg_input: &str = "Failed to read input";

    io::stdin().read_line(&mut guess).expect(err_msg_input);

    let err_msg_num: &str = "Not a valid number!";

    let mut guess: u32 = guess.trim().parse().expect(err_msg_num);

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    while guess != secret_number {
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is too small!"),
            Ordering::Greater => println!("Your number is too big!"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
        }

        println!("\nInput your number for another guess:");

        let mut new_guess = String::new();
        io::stdin().read_line(&mut new_guess).expect(err_msg_input);

        guess = new_guess.trim().parse().expect(err_msg_num);
    } 
}
