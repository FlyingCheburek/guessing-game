use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n***Guess the number!***");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    //println!("************************\n");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        print!("You guessed: {guess}");
        let guess: u32 = match guess.trim().parse(){
            Ok(input) => input,
            Err(_) => continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("> Too small!\n-------------------"),
            Ordering::Greater => println!("> Too big!\n-------------------"),
            Ordering::Equal => {
                println!(">>> You won! <<<");
                break;
            }
        }
    } 
}
