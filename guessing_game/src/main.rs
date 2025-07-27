use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::rng().random_range(1..=100);
    println!("the secret num is {secret_num}");
    
    let mut guess = String::new();
    loop {
        guess.clear();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");
        println!("you guessed {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => {println!("{} : {guess}", err); continue;} ,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => {
                println!("too small.");
            }
            Ordering::Greater => {
                println!("too big.");
            }
            Ordering::Equal => {
                println!("You Win!.");
                break;
            }
        }
    }
}
