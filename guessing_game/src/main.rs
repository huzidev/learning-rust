use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Lets play guess a number!");
    let low = 1;
    let high = 100;

    let secret_number = rand::thread_rng().gen_range(low, high);

    loop {
        println!("The secret number is {}", secret_number);
        println!("Please guess a number between {} and {}", low, high);

        // String::new() Creates an empty string
        let mut guess = String::new();

        // Taking input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        // convert string to int(u32)
        // parse() like parseInt()
        // match to check if user has entered the number Or String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // If user didn't add number
                println!("{}", "Please enter a valid number!".yellow());
                continue;
            }
        };

        println!("You've guessed {}", guess);

        if guess < low || guess > high {
            println!("{}", format!("Please enter a number between {} and {}!", low, high).yellow());
            continue;
        }


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Oops, You've guessed smaller number".red()),
            Ordering::Equal => {
                println!("{}", "Congratulations!, You've guessed correct".green());
                // Stop loop when win
                break;
            }
            Ordering::Greater => println!("{}", "Oops, You've guessed bigger number".red()),
        }


        let mut try_again = String::new();
        println!("Do wanna try again? (y/n)");

        io::stdin()
            .read_line(&mut try_again)
            .expect("Failed to read input");

        match try_again.trim().to_lowercase().as_str() {
            "y" => continue,
            "n" => {
                println!("Thanks for playing!");
                break;
            },
            _ => {
                println!("{}", "Invalid input! Exiting the game.".yellow());
                break;   
            }
        }
    }
}
