use std::io;
fn main() {
    println!("Input your number for guessing:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("Your number is: {guess}");


}
