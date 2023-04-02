use rand::Rng;
use std::io;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please Guess the number (1-100):");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to load the guess :(");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your Guess: {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Higher"),
            std::cmp::Ordering::Greater => println!("Lower"),
            std::cmp::Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}
