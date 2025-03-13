use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== NUMBER GUESSING GAME ===");
    println!("Try to guess the secret number between 1 and 100!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let max_attempts = 7;
    let mut attempts_left = max_attempts;
    
    loop {
        if attempts_left == 0 {
            println!("\nGame Over! You've used all your attempts.");
            println!("The secret number was: {}", secret_number);
            break;
        }
        
        println!("\nAttempts remaining: {}", attempts_left);
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        attempts_left -= 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\nCongratulations! You guessed the number: {}!", secret_number);
                println!("You won with {} attempts remaining!", attempts_left);
                break;
            }
        }
        
        if attempts_left == 1 {
            println!("Last chance! Make it count!");
        } else if attempts_left == 2 {
            let range_hint = if secret_number <= 50 {
                "The number is between 1 and 50"
            } else {
                "The number is between 51 and 100"
            };
            println!("Hint: {}", range_hint);
        }
    }
    
    println!("\nWould you like to play again? (y/n)");
    let mut play_again = String::new();
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line");
    
    if play_again.trim().to_lowercase() == "y" {
        main();
    } else {
        println!("Thanks for playing!");
    }
}