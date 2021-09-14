use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Create a simple guessing game that prompts a user for number
// and informs the user if their choice is higher than, lower than or equal to
// a randomly generated secret number

fn main() {
    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // Get the user's input from the command line
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // Ensure that the user's input is a valid number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        // Inform the user if their guess is <, > or = to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
