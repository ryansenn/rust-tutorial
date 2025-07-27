use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}