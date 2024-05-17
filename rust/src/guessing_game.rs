use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("----------------------------");
    println!("!---  GUESS THE NUMBER  ---!");
    println!("----------------------------\n");

    // Compute random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Collect user Input
        println!("Please input a guess between 1-100: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare both numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You lose! Your guess is too small!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
            Ordering::Greater => println!("You lose! Your guess is too big!\n"),
        }
    }
}
