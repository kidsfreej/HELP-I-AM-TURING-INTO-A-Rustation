use std::io;
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen();

    let mut isloop = true;
    let mut guess = String::new();
    let mut parsed:i32=-100;
    while(isloop) {
        println!("Guess a number you idiot I hate you!");
        io::stdin().read_line(&mut guess).expect("I hate you. For some reason there was an error. WTF!");

        match guess.trim().parse::<i32>() {
            Err(_e) => isloop = true,
            Ok(n) => {parsed = n;
                isloop=false}
        }
    }

    println!("You guessed incorrectly bruh!!! {}",parsed);
}
