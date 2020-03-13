use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_numb = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_numb);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_numb) {
            Ordering::Less => {
                if secret_numb - guess < 5 {
                    println!("C'mon, you got it..");
                }
                println!("Too small");
            }
            Ordering::Greater => {
                if guess - secret_numb < 5 {
                    println!("C'mon, you got it...");
                }
                println!("Too high");
            }
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
