use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // println!("hey safee guess a number!");

    
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=77);
    
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("The secret number is: {secret_number}");
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

    }
}

// let name = "safeer"
// let age = "20"

// println!("name of the person is: {name} and age of the person is {}", age+3)