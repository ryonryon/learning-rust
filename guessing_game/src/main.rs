use std::io;

fn main() {
    println!("Guess the number!!");

    println!("Pleaze input your guess!?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Filed in read line");
    
    println!("You guessed: {}", guess);
}
